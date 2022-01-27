use std::io;

use chat_room_2::lib;
use chat_room_2::lib::Connect;

const TARGET: &str = "127.0.0.1:7878";
const SLEEP_MILLIS: u64 = 100;
const MSG_SIZE: usize = 128;

fn main() {
    println!("1:server,0:client,3:quit");
    loop {
        let mut msg = String::new();
        io::stdin().read_line(&mut msg).unwrap();
        let take = msg
            .split(" ")
            .into_iter()
            .collect::<Vec<_>>()
            .get(0)
            .cloned()
            .unwrap_or("4");
        match take {
            "1" => lib::server::new(TARGET, SLEEP_MILLIS, MSG_SIZE).run(),
            "2" => {
                println!("input u username");
                let mut msg = String::new();
                io::stdin().read_line(&mut msg).unwrap();
                let username = if msg.is_empty() {
                    continue;
                } else {
                    msg
                };
                lib::client::new(TARGET, username.as_str(), SLEEP_MILLIS, MSG_SIZE).run()
            }
            "3" => break,
            _ => continue,
        };
    }
    lib::client::new(TARGET, "user_name", SLEEP_MILLIS, MSG_SIZE).run();
}
