/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

#[macro_use]
extern crate rocket;

use futures::lock::Mutex;
use rocket::config::Config;
use rocket::fs::{relative, FileServer};
use rocket::{Build, Rocket};
use rocket_dyn_templates::Template;

mod capitube;
mod routes;

pub use rocket::main;

pub fn setup(address: &str, port: u16, client_port: u16) -> Rocket<Build> {
    let config = Config::figment()
        .merge(("address", address))
        .merge(("port", port));

    rocket::custom(config)
        .attach(capitube::ModelState::fairing(client_port))
        .attach(Template::fairing())
        .manage(Mutex::new(capitube::ModelState::default()))
        .mount("/", FileServer::from(relative!("../models")))
        .mount("/", routes![routes::to_model])
        .mount("/capitube", routes![routes::model, routes::events])
        .mount("/api", routes![routes::config_model, routes::update_model])
}
