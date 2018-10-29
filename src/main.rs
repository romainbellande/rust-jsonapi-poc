#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate jsonapi;
extern crate serde_json;
extern crate japi;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate diesel;

pub mod config;
pub mod hero;
pub mod schema;
pub mod db;

fn main() {
  config::load();
  rocket::ignite()
    .mount("/heroes", routes![hero::read, hero::read_one, hero::create, hero::delete, hero::update])
    .manage(db::connect())
    .launch();
}
