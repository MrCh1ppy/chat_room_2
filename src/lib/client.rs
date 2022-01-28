use std::cmp::Ordering;
use std::io::{ErrorKind, Read, Write};
use std::net::TcpStream;
use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::mpsc;
use std::sync::mpsc::{RecvError, SendError, TryRecvError};
use std::time::Duration;
use std::{io, thread};

use chrono::Utc;

use crate::lib::Connect;

pub struct ClientPram {
    target_host: String,
    user_name: String,
    sleep_millis: u64,
    msg_size: usize,
}

pub fn new(local_host: &str, user_name: &str, sleep_millis: u64, msg_size: usize) -> ClientPram {
    let local_host = local_host.to_string();
    let user_name = user_name.to_string();
    ClientPram {
        target_host: local_host,
        user_name,
        sleep_millis,
        msg_size,
    }
}

impl Connect for ClientPram {
    fn run(self) {
        let local_target_address_for_closures = self.target_host.clone();
        let local_target_address_for_text = self.target_host.clone();
        let mut client = match TcpStream::connect(self.target_host) {
            Ok(stream) => stream,
            Err(ref error) if error.kind() == ErrorKind::ConnectionRefused => {
                println!("服务器尚未启动\n进程退出\n");
                panic!("host not founded");
            }
            Err(_) => {
                panic!("connect failed");
            }
        };
        client.set_nonblocking(true).expect("no block failed");
        let (sender, receiver) = mpsc::channel::<String>();
        let socket_address = client
            .local_addr()
            .expect("find address failed")
            .to_string();
        let text_for_from = socket_address.clone();
        //新开了一个线程,与下面并行
        thread::spawn(move || loop {
            let mut buffer = vec![0; self.msg_size];
            match client.read_exact(&mut buffer) {
                Ok(_) => {
                    match String::from_utf8(buffer)
                        .expect("u8 -> str failed")
                        .parse::<TextMessage>()
                    {
                        Ok(msg) => match &msg.from.cmp(&socket_address) {
                            Ordering::Equal => (),
                            _ => {
                                println!("{}({}): {}", msg.username(), msg.m_date(), msg.content())
                            }
                        },
                        Err(_) => {
                            println!("msg parse failed");
                        }
                    }
                }
                Err(ref error) if error.kind() == ErrorKind::WouldBlock => (),
                Err(_) => {
                    println!("connect to {} lost", &local_target_address_for_closures);
                    break;
                }
            }
            match receiver.try_recv() {
                Ok(msg) => {
                    //这里少了一个clone(已加)
                    let mut byte_msg = msg.clone().into_bytes();
                    byte_msg.resize(self.msg_size, 0);
                    client.write_all(&byte_msg).expect("unable to send msg");
                }
                Err(TryRecvError::Empty) => (),
                Err(TryRecvError::Disconnected) => break,
            }
            thread::sleep(Duration::from_millis(self.sleep_millis))
        });
        println!("已进入聊天室");
        loop {
            let mut buffer = String::new();
            io::stdin().read_line(&mut buffer).expect("fail to read");
            let msg = buffer.trim();
            if msg == "quit" || msg == "exit" {
                break;
            }
            let message = TextMessage {
                from: text_for_from.clone(),
                to: local_target_address_for_text.clone(),
                content: msg.trim().to_string(),
                m_date: Utc::now().naive_local().format("%T").to_string(),
                username: self.user_name.to_string(),
            };
            if sender.send(message.to_string()).is_err() {
                break;
            }
        }
        println!("bye");
    }
}

struct TextMessage {
    from: String,
    to: String,
    content: String,
    m_date: String,
    username: String,
}

impl TextMessage {
    pub fn from(&self) -> &str {
        &self.from
    }

    pub fn to(&self) -> &str {
        &self.to
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn m_date(&self) -> &str {
        &self.m_date
    }

    pub fn username(&self) -> &str {
        &self.username
    }
}

//实现了toString接口
impl ToString for TextMessage {
    fn to_string(&self) -> String {
        format!(
            "^{}&{}&{}&{}&{}^",
            self.from, self.to, self.content, self.m_date, self.username
        )
    }
}

//实现了通过字符串解析的接口
impl FromStr for TextMessage {
    type Err = ParseIntError;

    //用str反向解析获取对象
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let msg: Vec<&str> = s.trim_matches(|p| p == '^').split('&').collect();
        let from = msg[0].to_string();
        let to = msg[1].to_string();
        let content = msg[2].to_string();
        let m_date = msg[3].to_string();
        let username = msg[4].to_string();
        Ok(TextMessage {
            from,
            to,
            content,
            m_date,
            username,
        })
    }
}
