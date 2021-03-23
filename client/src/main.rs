use std::io::{self, prelude::*, BufReader,Write};
use std::net::TcpStream;
use std::str;

fn main() -> io::Result<()>{
    //创建stream流，链接服务端8080端口 
    let mut stream =TcpStream::connect("127.0.0.1:8080")?;
    /**
     * 发送十次请求
     */
    for _ in 0..10 {
        //创建输入
        let mut input =String::new();
        //从输入中读取到input中
        io::stdin().read_line(&mut input).expect("Failed to read");
        //将input以bytes的方式写入到流中
        stream.write(input.as_bytes()).expect("Failed to write");
        //创建reader
        let mut reader = BufReader::new(&stream);
        //创建一个vec的buffer
        let mut buffer: Vec<u8> =Vec::new();
        //使用reader读取服务端返回信息的到buffer中去
        reader.read_until(b'\n' , &mut buffer).expect("Failed to read into buffer");
        //打印输出将格式转换为UTF8
        println!("read from server : {}", str::from_utf8(&buffer).unwrap());
        println!("");
    }
    Ok(())
}
