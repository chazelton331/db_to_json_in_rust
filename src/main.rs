extern crate postgres;
extern crate chrono;

use postgres::{Connection, SslMode};
//use postgres::types::Timestamp;

use chrono::offset::utc::*;
//use self::chrono::{TimeZone, NaiveDate, NaiveTime, NaiveDateTime, DateTime, UTC};

struct User {
    id:             i32,
    email:          String,
    first_name:     String,
    last_name:      String,
    age:            i32,
    signed_up_at:   chrono::DateTime<UTC>,
    referred:       bool,
    bio:            String,
}

fn main() {
    let user = "cliff";
    let pass = "";
    let host = "localhost";
    let port = 5432;
    let db   = "testing_rust";

    let mut conn_str = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);
    let     conn     = Connection::connect(conn_str, SslMode::None).unwrap();

    for row in &conn.query("select id, email, first_name, last_name, age, signed_up_at, referred, bio from users", &[]).unwrap() {
        let user = User {
            id:             row.get(0), //fields in a row can be accessed either by their indices or their column names ... access by index is more efficient
            email:          row.get(1),
            first_name:     row.get(2),
            last_name:      row.get(3),
            age:            row.get(4),
            signed_up_at:   row.get(5),
            referred:       row.get(6),
            bio:            row.get(7),
        };

        println!("found user id={} email={} fn={} ln={} age={} signed_up_at={} ref={} bio={}", user.id, user.email, user.first_name, user.last_name, user.age, user.signed_up_at, user.referred, user.bio);
    }
}
