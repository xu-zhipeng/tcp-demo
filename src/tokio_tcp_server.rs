use tokio::{
    io::{self},
    net::TcpListener,
};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tcp::socket::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    println!("启动监听");

    loop {
        let (stream, addr) = listener.accept().await?;
        println!("Accepted connection from {}", addr);

        let (rd, wr) = io::split(stream);

        //处理数据
        process_data(rd, wr).await?;
    }
}

async fn process_data(mut rd: ReadHalf<TcpStream>, mut wr: WriteHalf<TcpStream>) -> Result<(), io::Error> {
    // 接收数据
    let request = rd.read_len().await?;
    println!("Client Request: {}", &request);
    // 发送回复
    let response = format!("The server receives your message, msg: {}", &request);
    // send_len 通过content-length 标识数据长度
    wr.send_len(response).await?;
    // send_line 通过空行\n\n 作为结束标识
    // wr.send_line(response).await?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    //这里可以不关,因为到此程序已经结束了,整个tcp连接都会关闭
    // wr.send(response).await?;
    // wr.shutdown().await?;
    Ok(())
}