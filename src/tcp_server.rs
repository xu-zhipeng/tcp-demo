use std::{io, thread};
use std::net::{Shutdown, TcpListener, TcpStream};
use std::time::Duration;
use tcp::{SocketRecvTrait, SocketSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    let listener = TcpListener::bind("127.0.0.1:5005")?;
    println!("启动监听");


    // for stream in listener.incoming() {
    loop {
        if let Ok((stream, addr)) = listener.accept() {
            //由于Client写通道没有关闭,Read的问题,一直读不到EOF的问题 默认超时5秒
            // stream.set_read_timeout(Some(Duration::new(5, 0)));
            println!("Accepted connection from {}", addr);

            thread::spawn(move || {
                let result = process_data(stream);
                if result.is_err() {
                    let error = result.err().unwrap();
                    println!("处理数据错误：{:?}", error);
                }
            });
        }
    }
}

fn process_data(mut stream: TcpStream) -> Result<(), io::Error> {
    // 接收数据
    let request = stream.recv()?;
    println!("Client Request: {}", &request);
    // 发送回复
    let response = format!("The server receives your message, msg: {}", request);
    stream.send(response)?;
    //关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    //这里可以不关,因为到此程序已经结束了,整个tcp连接都会关闭
    // stream.shutdown(Shutdown::Write)?;
    Ok(())
}