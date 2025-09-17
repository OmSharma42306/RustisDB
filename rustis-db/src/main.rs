use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::thread;



fn handle_client(mut stream : TcpStream){
    let mut buffer = [0;512];

    loop{
        match stream.read(&mut buffer){
            Ok(0)=>{
                // because there's are 0 bytes in req
                println!("Client Disconnected!");
                break;
            }
            Ok(n)=>{
                println!("Received : {:?}",String::from_utf8_lossy(&buffer[..n]));
                stream.write_all(b"+PONG\r\n").unwrap(); 
            }
            Err(e)=>{
                eprintln!("Error : {}",e);
                break;
            }
        }
    }

}
fn main(){
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Mini Redis by Om Sharma Running at PORT : 6379..");

    for stream in listener.incoming(){
        match stream{
            Ok(stream)=>{
                // spawn thread.
                thread::spawn(|| handle_client(stream));
            }
            Err(e)=> eprintln!("Connection Failed! {}",e),
        }
    }
}