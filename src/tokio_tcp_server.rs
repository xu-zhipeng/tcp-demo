use tokio::{
    io::{self},
    net::TcpListener,
};
use tokio::io::{ReadHalf, WriteHalf};
use tokio::net::TcpStream;
use tcp::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

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
    let request = rd.recv().await?;
    println!("Client Request: {}", &request);
    // 发送回复
    let response = format!("The server receives your message, msg: {}", request);
    wr.send(response).await?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    //这里可以不关,因为到此程序已经结束了,整个tcp连接都会关闭
    //wr.shutdown().await?;
    Ok(())
}