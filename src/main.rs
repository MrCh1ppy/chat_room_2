use std::io;

use chrono::format::parse;

use chat_room_2::lib;
use chat_room_2::lib::Connect;

const TARGET: &str = "127.0.0.1:7878";
const SLEEP_MILLIS: u64 = 10;
const MSG_SIZE: usize = 256;

fn main() {
    println!("1:server,2:client,3:quit");
    loop {
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).unwrap();
        let take = msg
            .trim()
            .split(' ')
            .into_iter()
            .collect::<Vec<_>>()
            .get(0)
            .cloned()
            .unwrap()
            .parse::<i32>()
            .unwrap_or(4);
        println!("{}", take);
        match take {
            1 => {
                println!("server starting...");
                lib::server::new(TARGET, SLEEP_MILLIS, MSG_SIZE).run();
            }
            2 => {
                println!("input u username");
                let mut msg = String::new();
                io::stdin().read_line(&mut msg).unwrap();
                let username = if msg.is_empty() {
                    continue;
                } else {
                    msg.trim()
                };
                lib::client::new(TARGET, username, SLEEP_MILLIS, MSG_SIZE).run()
            }
            3 => break,
            _ => continue,
        };
    }
    println!("quit...");
}
