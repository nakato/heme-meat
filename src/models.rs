use schema::attendees;
use schema::event;

#[derive(Queryable, Serialize, Debug)]
pub struct Attendees {
    pub id: i32,
    pub name: String,
    pub additional: i32,
}

#[derive(Queryable, Serialize, Debug)]
pub struct Event {
    pub id: i32,
    pub title: String,
    pub description: String,
}

#[derive(Insertable, Deserialize, FromForm, Debug)]
#[table_name="attendees"]
pub struct NewAttendee {
    name: String,
    additional: i32,
}

#[derive(Insertable, Deserialize, FromForm, Debug)]
#[table_name="event"]
pub struct NewEvent {
    title: String,
    description: String,
}
