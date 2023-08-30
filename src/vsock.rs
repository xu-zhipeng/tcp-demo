use async_trait::async_trait;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::{self};
use std::convert::TryInto;
use std::io::ErrorKind;
use std::{io, mem};
use tokio_vsock::VsockStream;

/**
以下是针对VsockStream的封装
 */

/// 消息体头部 content-length 数据的存储空间size
pub const CONTENT_LENGTH_SIZE: usize = mem::size_of::<i32>();
///buff 缓冲区大小
pub const BUFFER_SIZE: usize = 1024;

///tokio_vsock::VsockStream Trait实现
#[async_trait]
pub trait SocketAsyncSendTrait {
    /// 阻塞等待写通道关闭（read 返回 0）
    async fn send(&mut self, msg: String) -> Result<usize, io::Error>;
    /// 无需等待写通道关闭
    /// read直接根据头部提供的content-length 来读取消息内容
    /// 头部插入content-length 标识
    async fn send_len(&mut self, msg: String) -> Result<usize, io::Error>;
    /// 无需等待写通道关闭
    /// read直接根据空行（/n/n）来作为结束标识符
    /// 尾部插入空行（/n/n）
    async fn send_line(&mut self, msg: String) -> Result<usize, io::Error>;
}

#[async_trait]
pub trait SocketAsyncRecvTrait {
    /// 阻塞等待写通道关闭（read 返回 0）
    async fn recv(&mut self) -> Result<String, io::Error>;
    /// 无需等待写通道关闭
    /// 直接根据头部提供的content-length 来读取消息内容
    async fn read_len(&mut self) -> Result<String, io::Error>;
    /// 无需等待写通道关闭
    /// 直接根据空行（/n/n）来作为结束标识符
    async fn read_line(&mut self) -> Result<String, io::Error>;
}

#[async_trait]
impl SocketAsyncSendTrait for VsockStream {
    async fn send(&mut self, msg: String) -> Result<usize, io::Error> {
        use tokio::io::AsyncWriteExt;
        let bytes = msg.as_bytes();
        let len = bytes.len();
        let mut write_size = 0;
        while write_size < len {
            let mut end = write_size + BUFFER_SIZE;
            if end > len {
                end = len;
            }
            self.write_all(&bytes[write_size..end]).await?;
            write_size = end;
        }
        Ok(write_size)
    }

    async fn send_len(&mut self, msg: String) -> Result<usize, io::Error> {
        use tokio::io::AsyncWriteExt;
        let mut bytes = vec![];
        //头部插入4个byte的content-length值
        let content_len: i32 = msg
            .as_bytes()
            .len()
            .try_into()
            .map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error usize to i32"))?;
        let content_len: [u8; CONTENT_LENGTH_SIZE] = content_len.to_be_bytes();
        bytes.extend_from_slice(&content_len[..]);
        //插入消息内容
        bytes.extend_from_slice(msg.as_bytes());

        let mut write_size = 0;
        let len = bytes.len();
        while write_size < len {
            let mut end = write_size + BUFFER_SIZE;
            if end > len {
                end = len;
            }
            self.write_all(&bytes[write_size..end]).await?;
            write_size = end;
        }
        Ok(write_size)
    }

    async fn send_line(&mut self, mut msg: String) -> Result<usize, io::Error> {
        msg.push_str("\n\n");
        self.send(msg).await
    }
}

#[async_trait]
impl SocketAsyncRecvTrait for VsockStream {
    async fn recv(&mut self) -> Result<String, io::Error> {
        use tokio::io::AsyncReadExt;
        let mut msg = vec![];
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = self.read(&mut buf).await?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
            if n != BUFFER_SIZE {
                break;
            }
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    async fn read_len(&mut self) -> Result<String, io::Error> {
        use tokio::io::AsyncReadExt;
        //读取内容长度
        let mut content_len = [0u8; CONTENT_LENGTH_SIZE];
        let n = self.read(&mut content_len).await?;
        if n == 0 {
            return Err(io::Error::new(ErrorKind::NotFound, "Not found content-length"));
        }
        let len = i32::from_be_bytes(content_len);
        let len: usize =
            len.try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error i32 to usize"))?;

        //读取消息内容
        let mut msg = vec![];
        let mut read_size = 0;
        let mut buf = [0u8; BUFFER_SIZE];
        while read_size < len {
            let n = self.read(&mut buf).await?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
            read_size = read_size + n;
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    async fn read_line(&mut self) -> Result<String, io::Error> {
        // 创建一个异步 reader，用于读取数据
        let mut reader = BufReader::new(self);
        let mut msg = String::new();
        // 读取数据并处理
        loop {
            let mut buffer = String::new();
            reader.read_line(&mut buffer).await?;
            if buffer.trim().is_empty() {
                break;
            }
            msg.push_str(&buffer);
        }
        msg.remove(msg.len() - 1);
        Ok(msg)
    }
}
