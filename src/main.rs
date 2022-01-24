use chat_room_2::lib;
use chat_room_2::lib::Order;

fn main() {
    lib::TcpSession::server("127.0.0.1:7878", 10, 128).run(Order::Server);
    println!("over");
}
