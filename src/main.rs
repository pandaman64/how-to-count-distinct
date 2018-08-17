#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

mod schema {
    table! {
        t (id) {
            id -> Int4,
            labels -> Array<Text>,
        }
    }

    #[derive(Insertable)]
    #[table_name = "t"]
    pub struct NewRow<'a> {
        pub labels: &'a [String],
    }
}

fn count_distinct_labels(conn: &PgConnection) -> i64 {
    // SELECT COUNT(*) FROM (SELECT DISTINCT unnest(labels) FROM t) AS label
    unimplemented!()
}

fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let conn = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));

    diesel::insert_into(schema::t::dsl::t)
        .values(&vec![
            schema::NewRow {
                labels: &["foo".to_string(), "bar".to_string()],
            },
            schema::NewRow {
                labels: &["bar".to_string(), "baz".to_string()],
            },
        ]).execute(&conn)
        .unwrap();

    // how to implement?
    assert_eq!(count_distinct_labels(&conn), 3);
}
