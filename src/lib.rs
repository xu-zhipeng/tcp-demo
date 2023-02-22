use std::io;
use std::time::Duration;
use tokio::io::{AsyncReadExt, AsyncWriteExt, ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use async_trait::async_trait;
use tokio::time::error::Elapsed;
use tokio::time::timeout;

pub const BUFFER_SIZE: usize = 1024;


/// tokio::net::TcpStream Trait实现
#[async_trait]
pub trait SocketAsyncSendTrait {
    async fn send(&mut self, msg: String) -> Result<usize, io::Error>;
}

#[async_trait]
pub trait SocketAsyncRecvTrait {
    async fn recv(&mut self) -> Result<String, io::Error>;
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
}


/// std::net::TcpStream Trait实现
pub trait SocketSendTrait {
    fn send(&mut self, msg: String) -> Result<usize, io::Error>;
}

pub trait SocketRecvTrait {
    fn recv(&mut self) -> Result<String, io::Error>;
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

