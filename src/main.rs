use chat_room_2::lib;

fn main() {
    lib::TcpSession::server("127.0.0.1:7878", 10, 128).run();
    println!("over");
}
