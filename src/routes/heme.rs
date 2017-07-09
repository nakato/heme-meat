use diesel;
use diesel::prelude::*;
use rocket::request::Form;
use rocket_contrib::JSON;

use ::db;
use ::schema::attendees;
use ::schema::event;

use ::models::{Attendees, NewAttendee, Event, NewEvent};

#[get("/get_attendees", format = "application/json")]
pub fn attendee_list(conn: db::Connection) -> JSON<Vec<Attendees>> {
    let e: Vec<Attendees> = attendees::table.load(&*conn).expect("Error loading attendees");
    JSON(e)
}

#[post("/add_attendee", format = "application/json", data = "<attendee>")]
pub fn add_attendee(conn: db::Connection, attendee: Option<JSON<NewAttendee>>) -> String {
    attendee.map(|attendee| {
        println!("{:?}", *attendee);
        diesel::insert(&*attendee).into(attendees::table)
            .execute(&*conn).expect("Error adding attendee");
    });
    String::from("Didn't crash")
}

#[get("/get_description", format = "application/json")]
pub fn description(conn: db::Connection) -> JSON<Event> {
    use ::schema::event::dsl::*;
    let mut e = event.filter(id.eq(1)).limit(1).load::<Event>(&*conn).expect("Error loading event");
    let s = e.remove(0);
    return JSON(s)
}

#[post("/set_description", data = "<evente>")]
pub fn set_description(conn: db::Connection, evente: Option<Form<NewEvent>>) -> String {
    evente.map(|eventee| {
        println!("{:?}", eventee.get());
        diesel::insert(eventee.get()).into(event::table)
            .execute(&*conn).expect("Error creating event");
    });
    String::from("Didn't crash")
}
