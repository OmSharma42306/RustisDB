use std::io::{Read,Write};
use std::net::{TcpListener,TcpStream};
use std::thread;
use std::collections::HashMap;
use std::sync::{Arc,Mutex};


type Db = Arc<Mutex<HashMap<String,String>>>;


fn handle_client(mut stream : TcpStream,db : Db){
    let mut buffer = [0;512];

    loop{
        match stream.read(&mut buffer){
            Ok(0)=>{
                // because there's are 0 bytes in req
                println!("Client Disconnected!");
                break;
            }
            Ok(n)=>{
                // println!("Received : {:?}",String::from_utf8_lossy(&buffer[..n]));
                // stream.write_all(b"+PONG\r\n").unwrap(); 

                let input = String::from_utf8_lossy(&buffer[..n]);
                let parts : Vec<&str> = input.trim().split_whitespace().collect();

                let response = match parts.as_slice(){
                    ["P"] => "+PONG\r\n".to_string(),
                    ["SET",key,value] =>{
                        db.lock().unwrap().insert(key.to_string(), value.to_string()); 
                        "+OK\r\n".to_string()
                    }
                    ["GET",key] =>{
                        if let Some(val) = db.lock().unwrap().get(*key){
                            format!("${}\r\n{}\r\n",val.len(),val)
                        }else{
                            "$-1\r\n".to_string() // Null Bulk String return.
                        }
                    }
                    _=> "-ERR unknown command\r\n".to_string(),
                };

                stream.write_all(response.as_bytes()).unwrap();
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

    let db : Db = Arc::new(Mutex::new(HashMap::new()));

    for stream in listener.incoming(){
                // create an pointer to an copy of db;
                let db = Arc::clone(&db);
                // spawn thread.
                thread::spawn(move || handle_client(stream.unwrap(),db));

    }
}