#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate rocket;
extern crate heme_meat;

use dotenv::dotenv;
use heme_meat::db;
use heme_meat::routes::*;
use std::env;

fn main() {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("env var DATABASE_URL needed");

    rocket::ignite()
        .mount("/api", routes![attendee_list, add_attendee, description, set_description])
        .mount("/", routes![index, files])
        .manage(db::init_pool(&database_url))
        .launch();
}
