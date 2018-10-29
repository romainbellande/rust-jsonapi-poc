#![allow(proc_macro_derive_resolution_fallback)]
extern crate r2d2;
extern crate r2d2_diesel;

use diesel;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use rocket_contrib::{Json, Value};
use jsonapi::jsonapi_model;
use jsonapi::model::*;

use super::schema::heroes;
use super::db;
use japi::api::{DocumentDto, ResourceDto};
use japi::model::JApiModel;
use japi::japi_model;

#[table_name = "heroes"]
#[derive(Insertable, Deserialize, Debug)]
pub struct HeroDto {
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32,
}

#[table_name = "heroes"]
#[derive(Serialize, Deserialize, Queryable, Insertable, AsChangeset)]
pub struct Hero {
  pub id: i32,
  pub name: String,
  pub identity: String,
  pub hometown: String,
  pub age: i32,
}
japi_model!(Hero; "hero");

impl Hero {
  pub fn create(hero: Hero, connection: &PgConnection) -> Hero {
    diesel::insert_into(heroes::table)
      .values(hero)
      .execute(connection)
      .expect("Error creating new hero");

    heroes::table.order(heroes::id.desc()).first(connection).unwrap()
  }

  pub fn read(connection: &PgConnection) -> Vec<Hero> {
    heroes::table.order(heroes::id).load::<Hero>(connection).unwrap()
  }

  pub fn read_one(id: i32, connection: &PgConnection) -> Hero {
    heroes::table.filter(heroes::id.eq(id)).first(connection).unwrap()
  }

  pub fn update(id: i32, hero: Hero, connection: &PgConnection) -> bool {
    diesel::update(heroes::table.find(id)).set(&hero).execute(connection).is_ok()
  }

  pub fn delete(id: i32, connection: &PgConnection) -> bool {
    diesel::delete(heroes::table.find(id)).execute(connection).is_ok()
  }
}

#[post("/", data="<hero>")]
pub fn create(hero: Json<DocumentDto<HeroDto>>, connection: db::Connection) {
  print!("{:?}", hero.deserialize());
  // Json(Hero::create(Hero::from_jsonapi_document(&hero).unwrap(), &connection))
}

#[get("/")]
fn read(connection: db::Connection) -> Json<Value> {
  Json(json!(Hero::read(&connection)))
}

#[get("/<id>")]
fn read_one(id: i32, connection: db::Connection) -> Json<Value> {
  Json(json!(Hero::read_one(id, &connection).serialize()))
}

#[put("/<id>", data = "<hero>")]
fn update(id: i32, hero: Json<Hero>, connection: db::Connection) -> Json<Value> {
  Json(json!({
    "success": Hero::update(id, hero.into_inner(), &connection)
  }))
}

#[delete("/<id>")]
fn delete(id: i32, connection: db::Connection) -> Json<Value> {
  Json(json!({
    "success": Hero::delete(id, &connection)
  }))
}
