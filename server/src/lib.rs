#[macro_use]
extern crate rocket;

use rocket::config::Config;
use rocket::{Build, Rocket};

pub use rocket::main;

pub fn setup(address: &str, port: u16) -> Rocket<Build> {
    let config = Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    rocket::custom(config)
}
