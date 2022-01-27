use std::io::{ErrorKind, Read, Write};
use std::net::TcpListener;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;

use crate::lib::Connect;

pub struct ServerParam {
    local_host: String,
    sleep_millis: u64,
    msg_size: usize,
}

pub fn new(local_host: &str, sleep_millis: u64, msg_size: usize) -> ServerParam {
    let local_host = local_host.to_string();
    ServerParam {
        local_host,
        sleep_millis,
        msg_size,
    }
}

impl Connect for ServerParam {
    fn run(self) {
        let listener = TcpListener::bind(self.local_host.clone()).expect("bind failed");
        listener.set_nonblocking(true).expect("no blocking failed");
        let mut clients = vec![];
        let (sender, receiver) = mpsc::channel::<String>();
        println!("server started({})",&self.local_host);
        loop {
            if let Ok((mut stream, address)) = listener.accept() {
                println!("{} has connected", address);
                clients.push(stream.try_clone().expect("clone fail"));
                let sender = sender.clone();
                thread::spawn(move || loop {
                    let mut msg_buffer = vec![0; self.msg_size];
                    match stream.read_exact(&mut msg_buffer) {
                        Ok(_) => {
                            let msg = msg_buffer
                                .into_iter()
                                .take_while(|&cur| cur != 0)
                                .collect::<Vec<_>>();
                            let msg = String::from_utf8(msg).expect("change fail");
                            println!("one msg have got and sent");
                            sender.send(msg).expect("send fail");
                        }
                        Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
                        Err(_) => {
                            println!("connection to {} lost", address);
                            break;
                        }
                    }
                    thread::sleep(Duration::from_millis(self.sleep_millis));
                });
            }
            //需要使用try_recy
            if let Ok(msg) = receiver.try_recv() {
                println!("msg[{:?}] is received",msg);
                clients = clients
                    .into_iter()
                    .flat_map(|mut client| {
                        let mut byte_msg = msg.clone().into_bytes();
                        byte_msg.resize(self.msg_size, 0);
                        client.write_all(&byte_msg).map(|_| client).ok()
                    })
                    .collect::<Vec<_>>();
            }
            thread::sleep(Duration::from_millis(self.sleep_millis));
        }
    }
}
