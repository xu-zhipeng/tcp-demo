use tokio::io::{self, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use tcp::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let stream = TcpStream::connect("127.0.0.1:5000").await?;
    let (rd, mut wr) = io::split(stream);
    println!("连接成功");

    //发送数据
    // let msg = "abcdefghijklmnop";
    let msg = "abcdefghijklmnopqrstuvwxyz";
    wr.send(msg.to_string()).await?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    wr.shutdown().await?;

    process_data(rd).await?;

    Ok(())
}

pub async fn process_data(mut rd: ReadHalf<TcpStream>) -> Result<(), io::Error> {
    // 接收回复
    let response = rd.recv().await?;
    println!("Server Response: {}", response);
    Ok(())
}


