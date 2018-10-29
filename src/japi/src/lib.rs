#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate serde;
extern crate serde_json;

#[macro_use] extern crate rocket_contrib;
#[macro_use] extern crate serde_derive;

pub mod api;
pub mod model;
pub mod core;
