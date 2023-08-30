// client.rs

use std::io;
use tokio::net::UnixStream;
use tokio_vsock::VsockStream;
use tcp::vsock::SocketAsyncRecvTrait;

#[tokio::main]
async fn main() -> io::Result<()> {
    let stream = UnixStream::connect("vsock://2:5000").await?;
    println!("Connected to server: {:?}", stream);

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

