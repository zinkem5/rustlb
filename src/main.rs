
use std::net::{TcpStream, TcpListener};
use std::io::{Read, Write};
use std::{thread, time};

fn pipe_to(mut source: &TcpStream, mut dest: TcpStream, label: String) -> TcpStream {
    //println!("piping...");
    let mut buf = [0u8 ;4096];

    let mut queued = source.peek(&mut buf).unwrap();

    println!("size: {}", queued);

    while queued <= 0 {
        queued = source.peek(&mut buf).unwrap();
        println!("size: {}", queued);
        thread::sleep(time::Duration::from_millis(500));
    }
    
    match source.read(&mut buf) {
        Ok(_) => {
            let bytes_read = String::from_utf8_lossy(&buf);
            if bytes_read != "" {
                println!("{} read {} end", label, bytes_read);
            }
        }
        Err(_e) => {} //println!("{} Unable to read stream: {}", label, e)
    }
    
    match dest.write(&buf) {
        Ok(_) => { println!("{} wrote :{}:", label, String::from_utf8_lossy(&buf))}
        Err(_e) => {}// println!("{} Unable to write stream: {}", label, e)
    }
    
    (dest)
}


fn tcp_forward(stream: TcpStream, port: u8) {
    let src = stream;
    println!("127.0.0.1:800{}", port);
    //let mut dest = TcpStream::connect("50.19.229.188:80").unwrap();
    let mut dest = TcpStream::connect("127.0.0.1:8000").unwrap();
    dest.set_nodelay(true);

    let local = dest.local_addr().unwrap().port();
    println!("Connecting client to server at port 800{} from {}", port, local);
    //src.set_nonblocking(true);
    //dest.set_nonblocking(true);
    (dest) = pipe_to(&src, dest, "src -> dest".to_string());
    //thread::sleep(time::Duration::from_millis(5000));
    let _ = pipe_to(&dest, src, "dest -> src".to_string());
}

fn main() {
    println!("Starting rust Load Balancer");

    let mut count = 0;
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Listening for TCP connetions at port 3000");


    
    for stream in listener.incoming() {
        count = count + 1;
        if count > 9 {
            count = 0;
        }
        
        match stream {
            Ok(stream) => {
                let port = count;
                thread::spawn(move || {
                    tcp_forward(stream, port);
                });
            }
            Err(e) => {

                println!("Unable to connect: {}", e);
            }
        }
    }
}
