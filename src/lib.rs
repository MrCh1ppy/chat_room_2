pub mod lib {
    use std::io::{ErrorKind, Read};
    use std::net::TcpListener;
    use std::sync::mpsc;
    use std::thread;
    use std::time::Duration;

    pub enum Order {
        Client,
        Server,
    }

    pub struct TcpSession {
        local_host: String,
        user_name: String,
        sleep_millis: u64,
        msg_size: usize,
    }

    impl TcpSession {
        pub fn server(local_host: &str, sleep_millis: u64, msg_size: usize) -> TcpSession {
            let local_host = local_host.to_string();
            let user_name = "server".to_string();
            TcpSession {
                local_host,
                user_name,
                sleep_millis,
                msg_size,
            }
        }

        pub fn client(
            local_host: &str,
            user_name: &str,
            sleep_millis: u64,
            msg_size: usize,
        ) -> TcpSession {
            let local_host = local_host.to_string();
            let user_name = user_name.to_string();
            TcpSession {
                local_host,
                user_name,
                sleep_millis,
                msg_size,
            }
        }

        pub fn run(self, order: Order) {
            match order {
                Order::Server => self.run_server(),
                Order::Client => self.run_client(),
            }
        }

        fn run_server(self) {
            let listener = TcpListener::bind(self.local_host).expect("bind failed");
            listener.set_nonblocking(true).unwrap();
            let mut clients = vec![];
            let (sender, receiver) = mpsc::channel::<_>();
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
                                println!("connect to {} lost", address);
                                break;
                            }
                        }
                    });
                }
                thread::sleep(Duration::from_millis(self.sleep_millis as u64));
            }
        }

        fn run_client(self) {
            todo!()
        }
    }
}
