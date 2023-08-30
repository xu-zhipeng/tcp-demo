use std::{io};
use std::io::Write;
use std::net::{Shutdown, TcpStream};
use socket::{SocketRecvTrait, SocketSendTrait};
use tcp::socket::{SocketRecvTrait, SocketSendTrait};

fn main() -> Result<(), io::Error> {
    let mut stream = TcpStream::connect("127.0.0.1:5005")?;
    println!("连接成功");

    // 发送数据
    // let msg = "abcdefghijklmnop";
    let msg = "abcdefghijklmnopqrstuvwxyz";
    // send_len 通过content-length 标识数据长度
    stream.send_len(msg.to_string())?;
    // send_line 通过空行\n\n 作为结束标识
    // stream.send_line(msg.to_string())?;
    // 关闭 TcpStream 的写操作 否则 read会阻塞 无法返回0
    // stream.send(msg.to_string())?;
    // stream.shutdown(Shutdown::Write)?;

    process_data(stream)?;

    Ok(())
}

pub fn process_data(mut stream: TcpStream) -> Result<(), io::Error> {
    // 接收回复
    let response = stream.read_len()?;
    println!("Server Response: {}", &response);
    Ok(())
}

