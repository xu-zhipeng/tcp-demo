// server.rs

use std::io;
use tokio::net::UnixListener;
use tokio_vsock::VsockStream;
use tcp::vsock::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = UnixListener::bind("vsock://2:5000").await?;

    println!("Server started, waiting for connections...");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            println!("New client connected: {:?}", socket);
            // Handle client connection and data here
            //处理数据
            process_data(stream).await?;
        });
    }
}

pub async fn process_data(mut stream: VsockStream) -> Result<(), io::Error> {
    //接收数据
    let request = stream.read_line().await?;
    println!("server received, {}", &request);

    // 发送回复
    let response = format!("The server receives your message, msg: {}", &request);
    //写入数据
    stream.send_line(response).await?;
    //关闭写入流  不关闭，另一端 read 会发生阻塞
    // stream.shutdown(Shutdown::Both)?;
    // drop(stream);
    Ok(())
}
