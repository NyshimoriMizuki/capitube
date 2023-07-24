/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[macro_use]
extern crate rocket;

use rocket::config::Config;
use rocket::{Build, Rocket};

mod capitube;
mod routes;

pub use rocket::main;

pub fn setup(address: &str, port: u16) -> Rocket<Build> {
    let config = Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    rocket::custom(config)
}
