#[macro_use]
extern crate log;
extern crate packem_logger;

fn main() {
    packem_logger::init().unwrap();

    info!("This is an example message.");
    warn!("This is an example message.");
}
