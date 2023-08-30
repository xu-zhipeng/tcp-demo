# Tcp Socket以及VSocket连接示例

## TCP连接描述

一共有两个demo，一个是Rust std库写的连接，一个是用tokio写的异步非阻塞连接

tcp 是一个双向数据通道的连接，所以

client 的 write （发送请求） 对应 server read（接收请求）

server  process（处理接收的数据）

server的 write （返回响应） 对应 client read（接收响应）



Client发送数据   这时候连接没有关闭，  server read 阻塞 不会返回0

​		有两种方案：

​				1：需要两边协商规定，提供结束标识，例如http 的keepalive 用空行做标识,或者规定数据长度

​				2：写入数据后将 写通道 关闭，在这之后 无法再写入数据。

​				一般如果只发送一次tcp请求可以选第二种，如果发送多个，选择方案一，自行协商数据结束标识。



tcp 底层连接过程描述：

server bind 

server listen

server accept



client connect 

client write   如果选择方案2：write shutdown

server read

handle data

server write 如果选择方案2，并且socket 没有立即销毁：write shutdown

client read

## Vsock

vosock 和tcp 正常的socket一样没什么区别，就是所使用的stream对象不一样，这里推荐tokio_vsock，可以是用 VsockStream

## 其他

rust 读写通道关闭

```rust
//std
stream.shutdown(Shutdown::Write)?;
stream.shutdown(Shutdown::Read)?;
stream.shutdown(Shutdown::Both)?;//全部

//tokio tokio::io::ReadHalf<TcpStream>和WriteHalf<TcpStream>
wr.shutdown().await?;
wr.shutdown().await?;
//tokio::net::TcpStream
stream.shutdown(Shutdown::Write).await?;
stream.shutdown(Shutdown::Read).await?;
stream.shutdown(Shutdown::Both).await?;//全部


```

