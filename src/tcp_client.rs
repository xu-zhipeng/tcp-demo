use std::{io};
use std::net::{Shutdown, TcpStream};
use tcp::{SocketRecvTrait, SocketSendTrait};

fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:5005")?;
    println!("连接成功");

    // 发送数据
    // let msg = "abcdefghijklmnop";
    let msg = "abcdefghijklmnopqrstuvwxyz";
    stream.send(msg.to_string())?;
    // 关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    stream.shutdown(Shutdown::Write)?;

    process_data(stream)?;

    Ok(())
}

pub fn process_data(mut stream: TcpStream) -> Result<(), io::Error> {
    println!("test");
    // 接收回复
    let response = stream.recv()?;
    println!("Server Response: {}", response);
    Ok(())
}

