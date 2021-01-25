use diesel::{self, prelude::*};
use rocket_contrib::json::Json;

use crate::database::models::Person;
use crate::database::schema;
use crate::DbConn;


#[derive(Serialize)]
pub struct Test {
    firstname: String,
    email: String,
    active: bool
}


#[get("/persons")]
pub fn persons(conn: DbConn) -> Result<Json<Vec<Person>>, String> {

    use crate::database::schema::persons::dsl::*;

    return persons.load::<Person>(&*conn)
        .map_err(|err| -> String {
        println!("Error querying page views: {:?}", err);
        "Error querying page views from the database".into()
    }).map(Json);
}