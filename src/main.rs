#![feature(proc_macro_hygiene, decl_macro)]
#[macro_use]
extern crate rocket;

#[cfg(feature = "music")]
mod music;

mod media;
mod media_type;
mod tags;

use tags::*;

pub fn rocket() -> rocket::Rocket {
    rocket::ignite().mount("/v1", routes![get_tags])
}

fn main() {
    rocket().launch();
}
