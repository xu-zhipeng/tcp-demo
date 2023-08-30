use std::net::Shutdown;
use tokio::io;
use tokio_vsock::VsockStream;
use tcp::vsock::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let cid = AWS_PARENT_CID;
    let port = 5000;
    let mut stream = VsockStream::connect(cid, port).await.expect(&format!("vsock connect error,cid:{} port:{}", cid, port));
    println!("连接成功");

    //发送数据
    // let msg = "abcdefghijklmnop";
    let msg = "abcdefghijklmnopqrstuvwxyz";
    // send_len 通过content-length 标识数据长度
    stream.send_len(msg.to_string()).await?;
    // send_line 通过空行\n\n 作为结束标识
    // stream.send_line(msg.to_string()).await?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    // stream.send(msg.to_string()).await?;
    // stream.shutdown().await?;

    process_data(stream).await?;

    Ok(())
}

pub async fn process_data(mut stream: VsockStream) -> Result<(), io::Error> {
    // 接收回复
    let response = stream.read_len().await?;
    println!("Server Response: {}", &response);
    Ok(())
}

async fn send_test() -> Result<String, io::Error> {
    println!("发送消息测试");
    let cid = AWS_PARENT_CID;
    let port = 5000;
    let mut client = VsockClient::new(cid, port).await;
    client.send_line("测试测试".to_string()).await?;
    println!("发送成功");
    let response = client.read_line().await?;
    println!("接受到：{}", &response);
    println!("发送消息测试结束");
    Ok(response)
}


const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const AWS_PARENT_CID: u32 = 2;

pub struct VsockClient {
    pub stream: VsockStream,
}

impl VsockClient {
    pub async fn new(cid: u32, port: u32) -> Self {
        println!("new client cid:{},port:{}", cid, port);
        let stream = VsockStream::connect(cid, port).await.expect(&format!("vsock connect error,cid:{} port:{}", cid, port));
        VsockClient {
            stream,
        }
    }

    pub async fn send_line(&mut self, msg: String) -> Result<usize, io::Error> {
        self.stream.send_line(msg).await
    }

    pub async fn send_len(&mut self, msg: String) -> Result<usize, io::Error> {
        self.stream.send_len(msg).await
    }

    pub async fn read_len(&mut self) -> Result<String, io::Error> {
        self.stream.read_len().await
    }

    pub async fn read_line(&mut self) -> Result<String, io::Error> {
        self.stream.read_line().await
    }

    pub fn close(self, shutdown: Shutdown) -> Result<(), io::Error> {
        self.stream.shutdown(shutdown)?;
        drop(self.stream);
        Ok(())
    }
}


