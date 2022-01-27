use chat_room_2::lib;
use chat_room_2::lib::Connect;

fn main() {
    lib::server::new("127.0.0.1:7878", 100, 256).run();
}
