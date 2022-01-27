pub mod lib {
    pub mod client;
    pub mod server;

    pub trait Connect {
        fn run(self);
    }
}
