#[macro_use]
extern crate diesel;
extern crate dotenv;

use diesel::pg::PgConnection;
use diesel::prelude::*;
use diesel::sql_types::{Array, Text};
use dotenv::dotenv;
use std::collections::HashSet;
use std::env;

mod schema;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

sql_function!(fn unnest(a: Array<Text>) -> Text);

fn select_distinct_labels(conn: &PgConnection) -> Vec<String> {
    use schema::t::dsl::*;

    t.select(unnest(labels))
        .distinct()
        .get_results::<String>(conn)
        .unwrap()
}

fn count_distinct_labels_not_work(conn: &PgConnection) -> i64 {
    use schema::t::dsl::*;

    t.select(unnest(labels))
        .distinct()
        .count()
        .get_result(conn)
        .unwrap()
}

fn count_distinct_labels(conn: &PgConnection) -> i64 {
    // SELECT COUNT(*) FROM (SELECT DISTINCT unnest(labels) FROM t) AS label
    unimplemented!()
}

fn main() {
    use schema::t::dsl;

    let conn = establish_connection();

    diesel::insert_into(dsl::t)
        .values(&vec![
            schema::NewRow {
                labels: &["foo".to_string(), "bar".to_string()],
            },
            schema::NewRow {
                labels: &["bar".to_string(), "baz".to_string()],
            },
        ]).execute(&conn)
        .unwrap();

    let mut labels = select_distinct_labels(&conn);
    labels.sort();

    assert_eq!(
        labels,
        ["bar".to_string(), "baz".to_string(), "foo".to_string()]
    );

    // how to implement?
    // assert_eq!(count_distinct_labels(&conn), 4);

    // thread 'main' panicked at 'assertion failed: `(left == right)`
    //   left: `2`,
    //   right: `4`', src/main.rs:75:5
    assert_eq!(count_distinct_labels_not_work(&conn), 4);
}
