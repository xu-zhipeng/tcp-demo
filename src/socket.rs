use std::{i32, io, mem};
use std::io::ErrorKind;
// use std::time::Duration;
use tokio::io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use async_trait::async_trait;
// use tokio::time::error::Elapsed;
// use tokio::time::timeout;

pub const CONTENT_LENGTH_SIZE: usize = mem::size_of::<i32>();
pub const BUFFER_SIZE: usize = 1024;


/// tokio::net::TcpStream Trait实现
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
impl SocketAsyncSendTrait for TcpStream {
    async fn send(&mut self, msg: String) -> Result<usize, io::Error> {
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
        };
        Ok(write_size)
    }

    async fn send_len(&mut self, msg: String) -> Result<usize, io::Error> {
        let mut bytes = vec![];
        //头部插入4个byte的content-length值
        let content_len: i32 = msg.as_bytes().len().try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error usize to i32"))?;
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
        };
        Ok(write_size)
    }

    async fn send_line(&mut self, mut msg: String) -> Result<usize, io::Error> {
        msg.push_str("\n\n");
        self.send(msg).await
    }
}

#[async_trait]
impl SocketAsyncRecvTrait for TcpStream {
    async fn recv(&mut self) -> Result<String, io::Error> {
        let mut msg = vec![];
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = self.read(&mut buf).await?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    async fn read_len(&mut self) -> Result<String, io::Error> {
        //读取内容长度
        let mut content_len = [0u8; CONTENT_LENGTH_SIZE];
        let n = self.read(&mut content_len).await?;
        if n == 0 {
            return Err(io::Error::new(ErrorKind::NotFound, "Not found content-length"));
        }
        let len = i32::from_be_bytes(content_len);
        let len: usize = len.try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error i32 to usize"))?;

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
            //read_line 读取到换行（包含换行）,或者EOF就返回
            //所以这里用\n\n来做 数据结束标识
            reader.read_line(&mut buffer).await?;
            if buffer.trim().is_empty() {
                break;
            }
            msg.push_str(&buffer);
        }
        //去掉数据末尾的\n\n
        if !msg.is_empty() {
            msg.remove(msg.len() - 1);
        }
        Ok(msg)
    }
    /*async fn recv(&mut self) -> Result<String, io::Error> {
        let mut msg = vec![];
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            ///read 超时5秒会报错
            let res = timeout(Duration::from_secs(5), self.read(&mut buf)).await;
            match res {
                Ok(Ok(n)) => {
                    // 处理读取到的数据
                    if n == 0 {
                        //end of Stream
                        break;
                    }
                    msg.extend_from_slice(&buf[..n]);
                }
                Ok(Err(e)) => {
                    // 处理读取错误
                    return Err(e);
                }
                Err(e) => {
                    // 处理超时
                    return Err(Error::from(ErrorKind::TimedOut));
                }
            }
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }*/
}

#[async_trait]
impl SocketAsyncSendTrait for WriteHalf<TcpStream> {
    async fn send(&mut self, msg: String) -> Result<usize, io::Error> {
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
        };
        Ok(write_size)
    }

    async fn send_len(&mut self, msg: String) -> Result<usize, io::Error> {
        let mut bytes = vec![];
        //头部插入4个byte的content-length值
        let content_len: i32 = msg.as_bytes().len().try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error usize to i32"))?;
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
        };
        Ok(write_size)
    }

    async fn send_line(&mut self, mut msg: String) -> Result<usize, io::Error> {
        msg.push_str("\n\n");
        self.send(msg).await
    }
}

#[async_trait]
impl SocketAsyncRecvTrait for ReadHalf<TcpStream> {
    async fn recv(&mut self) -> Result<String, io::Error> {
        let mut msg = vec![];
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = self.read(&mut buf).await?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    async fn read_len(&mut self) -> Result<String, io::Error> {
        //读取内容长度
        let mut content_len = [0u8; CONTENT_LENGTH_SIZE];
        let n = self.read(&mut content_len).await?;
        if n == 0 {
            return Err(io::Error::new(ErrorKind::NotFound, "Not found content-length"));
        }
        let len = i32::from_be_bytes(content_len);
        let len: usize = len.try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error i32 to usize"))?;

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
            //read_line 读取到换行（包含换行）,或者EOF就返回
            //所以这里用\n\n来做 数据结束标识
            reader.read_line(&mut buffer).await?;
            if buffer.trim().is_empty() {
                break;
            }
            msg.push_str(&buffer);
        }
        //去掉数据末尾的\n\n
        if !msg.is_empty() {
            msg.remove(msg.len() - 1);
        }
        Ok(msg)
    }
}


/// std::net::TcpStream Trait实现
pub trait SocketSendTrait {
    /// 阻塞等待写通道关闭（read 返回 0）
    fn send(&mut self, msg: String) -> Result<usize, io::Error>;
    /// 无需等待写通道关闭
    /// read直接根据头部提供的content-length 来读取消息内容
    /// 头部插入content-length 标识
    fn send_len(&mut self, msg: String) -> Result<usize, io::Error>;
    /// 无需等待写通道关闭
    /// read直接根据空行（/n/n）来作为结束标识符
    /// 尾部插入空行（/n/n）
    fn send_line(&mut self, msg: String) -> Result<usize, io::Error>;
}

pub trait SocketRecvTrait {
    /// 阻塞等待写通道关闭（read 返回 0）
    fn recv(&mut self) -> Result<String, io::Error>;
    /// 无需等待写通道关闭
    /// 直接根据头部提供的content-length 来读取消息内容
    fn read_len(&mut self) -> Result<String, io::Error>;
    /// 无需等待写通道关闭
    /// 直接根据空行（/n/n）来作为结束标识符
    fn read_line(&mut self) -> Result<String, io::Error>;
}

impl SocketSendTrait for std::net::TcpStream {
    fn send(&mut self, msg: String) -> Result<usize, io::Error> {
        use std::io::Write;
        let bytes = msg.as_bytes();
        let len = bytes.len();
        let mut write_size = 0;
        while write_size < len {
            let mut end = write_size + BUFFER_SIZE;
            if end > len {
                end = len;
            }
            self.write_all(&bytes[write_size..end])?;
            write_size = end;
        };
        Ok(write_size)
    }

    fn send_len(&mut self, msg: String) -> Result<usize, io::Error> {
        use std::io::Write;
        let mut bytes = vec![];
        //头部插入4个byte的content-length值
        let content_len: i32 = msg.as_bytes().len().try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error usize to i32"))?;
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
            self.write_all(&bytes[write_size..end])?;
            write_size = end;
        };
        Ok(write_size)
    }

    fn send_line(&mut self, mut msg: String) -> Result<usize, io::Error> {
        msg.push_str("\n\n");
        self.send(msg)
    }
}

impl SocketRecvTrait for std::net::TcpStream {
    fn recv(&mut self) -> Result<String, io::Error> {
        use std::io::Read;
        let mut msg = vec![];
        let mut buf = [0u8; BUFFER_SIZE];
        loop {
            let n = self.read(&mut buf)?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    fn read_len(&mut self) -> Result<String, io::Error> {
        use std::io::Read;
        //读取内容长度
        let mut content_len = [0u8; CONTENT_LENGTH_SIZE];
        let n = self.read(&mut content_len)?;
        if n == 0 {
            return Err(io::Error::new(ErrorKind::NotFound, "Not found content-length"));
        }
        let len = i32::from_be_bytes(content_len);
        let len: usize = len.try_into().map_err(|_| io::Error::new(ErrorKind::InvalidData, "Convert Error i32 to usize"))?;
        println!("len: {}", len);

        //读取消息内容
        let mut msg = vec![];
        let mut read_size = 0;
        let mut buf = [0u8; BUFFER_SIZE];
        while read_size < len {
            let n = self.read(&mut buf)?;
            if n == 0 {
                //end of Stream
                break;
            }
            msg.extend_from_slice(&buf[..n]);
            read_size = read_size + n;
        }
        Ok(String::from_utf8_lossy(&msg).to_string())
    }

    fn read_line(&mut self) -> Result<String, io::Error> {
        use std::io::BufReader;
        use std::io::BufRead;
        // 创建一个异步 reader，用于读取数据
        let mut reader = BufReader::new(self);
        let mut msg = String::new();
        // 读取数据并处理
        loop {
            let mut buffer = String::new();
            //read_line 读取到换行（包含换行）,或者EOF就返回
            //所以这里用\n\n来做 数据结束标识
            reader.read_line(&mut buffer)?;
            if buffer.trim().is_empty() {
                break;
            }
            msg.push_str(&buffer);
        }
        //去掉数据末尾的\n\n
        if !msg.is_empty() {
            msg.remove(msg.len() - 1);
        }
        Ok(msg)
    }
}

/*impl SocketRecvTrait for std::net::TcpStream {
    fn read_to_end(&mut self) -> Result<String, io::Error> {
        use std::io::Read;
        let mut msg = vec![];
        ///处理方法1
        let n = self.read_to_end(&mut msg)?;
        println!("n:{}", n);

        ///处理方法2
        /*loop {
            println!("进入循环");
            let mut buf = vec![0u8; BUFFER_SIZE];
            match self.read_exact(&mut buf) {
                Ok(_) => {
                    let read = buf.into_iter().take_while(|&x: &u8| x != 0).collect::<Vec<u8>>();
                    println!("read:{}", String::from_utf8_lossy(&read).to_string());
                    msg.extend_from_slice(&read[..]);
                }
                Err(ref e) if e.kind() == ErrorKind::Interrupted => {
                    println!("Interrupted");
                    continue;
                }
                Err(_) => {
                    println!("error");
                    break;
                }
            };
        }*/

        Ok(String::from_utf8_lossy(&msg[..]).to_string())
    }
}*/

