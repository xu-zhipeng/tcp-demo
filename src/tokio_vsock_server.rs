use tokio::io;
use tokio_vsock::{VsockListener, VsockStream};
use tcp::vsock::{SocketAsyncRecvTrait, SocketAsyncSendTrait};

#[tokio::main]
async fn main() -> Result<(), io::Error> {
    VsockServer::init().await
}

pub struct VsockServer {}

const VMADDR_CID_ANY: u32 = 0xFFFFFFFF;
const AWS_PARENT_CID: u32 = 2;

impl VsockServer {
    ///初始化监听端口
    pub async fn init() -> Result<(), io::Error> {
        let cid = VMADDR_CID_ANY;
        let port = 5000;
        println!("start listening cid:{},port:{}", cid, port);
        let mut listener = VsockListener::bind(cid, port)?;

        loop {
            let (stream, addr) = listener.accept().await?;
            println!("Accepted connection from {}", addr);

            //处理数据
            process_data(stream).await?;
        }
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