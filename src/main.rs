extern crate postgres;

use postgres::{Connection, SslMode};

struct User {
    id:             i32,
    email:          String,
    first_name:     String,
    last_name:      String,
    age:            i32,
    referred:       bool,
    bio:            String,
}

fn main() {
    //let user = "cliff";
    //let pass = "";
    //let host = "localhost";
    //let port = 5432;
    //let db   = "testing_rust";
    //let conn_str = format!("postgres://{}:{}@{}:{}/{}", user, pass, host, port, db);
    //let () = conn_str;
    //let conn     = Connection::connect(conn_str.to_string(), SslMode::None).unwrap();
    let conn = Connection::connect("postgres://cliff:@localhost:5432/testing_rust", SslMode::None).unwrap();

    for row in &conn.query("select id, email, first_name, last_name, age, referred, bio from users", &[]).unwrap() {
        let user = User {
            id:             row.get(0), //fields in a row can be accessed either by their indices or their column names ... access by index is more efficient
            email:          row.get(1),
            first_name:     row.get(2),
            last_name:      row.get(3),
            age:            row.get(4),
            referred:       row.get(5),
            bio:            row.get(6),
        };

        println!("found user id={} email={} fn={} ln={} age={} ref={} bio={}\n", user.id, user.email, user.first_name, user.last_name, user.age, user.referred, user.bio);
    }
}
