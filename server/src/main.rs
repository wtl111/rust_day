use std::net::{TcpListener, TcpStream};
use std::time;
use std::thread;
use std::io::{self, Read, Write};

fn handle_client(mut stream: TcpStream) -> io::Result<()>{
    let mut buf = [0; 512];
    //代表服务只能进行1000次 如果达到1000次则关闭服务
    for _ in 0..1000{
        //将内如读取到buf里面
        let bytes_read = stream.read(&mut buf)?;
        //当长度为0则结束读取
        if bytes_read == 0 {
            return Ok(());
        }
        //将读取到的内容写回到客户端
        stream.write(&buf[..bytes_read])?;
    }
    Ok(())
}

//如果存在一场直接返回result
fn main() -> io::Result<()>{
    //监听8080
    let listener = TcpListener::bind("127.0.0.1:8080")?; 
    //将监听到的转变为流
    for stream in listener.incoming(){
        let stream = stream.expect("failed");
        thread::spawn(move || {
            //对流进行处理
            handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));

        }); 
    }
    Ok(())

}