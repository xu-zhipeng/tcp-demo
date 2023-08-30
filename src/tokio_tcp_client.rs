use tokio::io::{self, AsyncWriteExt, ReadHalf};
use tokio::net::TcpStream;
use socket::{SocketAsyncRecvTrait, SocketAsyncSendTrait};
use tcp::socket::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let stream = TcpStream::connect("127.0.0.1:5000").await?;
    let (rd, mut wr) = io::split(stream);
    println!("连接成功");

    //发送数据
    // let msg = "abcdefghijklmnop";
    let msg = "abcdefghijklmnopqrstuvwxyz";
    // send_len 通过content-length 标识数据长度
    wr.send_len(msg.to_string()).await?;
    // send_line 通过空行\n\n 作为结束标识
    // wr.send_line(msg.to_string()).await?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    // wr.send(msg.to_string()).await?;
    // wr.shutdown().await?;

    process_data(rd).await?;

    Ok(())
}

pub async fn process_data(mut rd: ReadHalf<TcpStream>) -> Result<(), io::Error> {
    // 接收回复
    let response = rd.read_len().await?;
    println!("Server Response: {}", &response);
    Ok(())
}


