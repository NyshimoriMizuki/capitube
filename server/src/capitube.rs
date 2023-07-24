/*
 * This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at https://mozilla.org/MPL/2.0/.
*/

use rocket::{
    fairing::{Fairing, Info, Kind},
    http::Header,
    serde::{Deserialize, Serialize},
    Request, Response,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct ModelState {
    model: String,
    pose: u8,
    state: u8,
    position: Vec<u8>,
    proportion: Vec<u8>,
}

impl ModelState {
    pub fn get_model(&self) -> String {
        self.model.clone()
    }

    pub fn unpack(&self) -> (u8, u8, Vec<u8>, Vec<u8>) {
        (
            self.pose,
            self.state,
            self.position.clone(),
            self.proportion.clone(),
        )
    }

    pub fn update(&mut self, pose: u8, position: Vec<u8>, proportion: Vec<u8>) {
        self.pose = pose;
        self.position = position;
        self.proportion = proportion;
    }

    pub fn update_state(&mut self, state: u8) {
        self.state = state;
    }

    pub fn blink(&mut self, is_to_blink: bool) -> ModelState {
        if is_to_blink {
            self.state += 1;
        }
        self.clone()
    }

    pub fn change_model(&mut self, model: &str) {
        self.model = model.to_string();
    }

    pub fn reset(&mut self) {
        self.pose = 0;
        self.state = 0;
        self.position = vec![0, 0];
        self.proportion = vec![0, 0];
    }

    pub fn fairing(port: u16) -> ModelFairing {
        ModelFairing { port }
    }
}

impl Default for ModelState {
    fn default() -> Self {
        Self {
            model: String::from("xisen"),
            pose: 0,
            state: 0,
            position: vec![0, 0],
            proportion: vec![0, 0],
        }
    }
}

pub struct ModelFairing {
    port: u16,
}

#[rocket::async_trait]
impl Fairing for ModelFairing {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS response headers to ModelState",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        let allowed_methods = vec!["GET", "POST"].join(", ");

        response.set_header(Header::new(
            "Access-Control-Allow-Origin",
            format!("http://localhost:{}", self.port),
        ));
        response.set_header(Header::new("Access-Control-Allow-Methods", allowed_methods));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
    }
}
