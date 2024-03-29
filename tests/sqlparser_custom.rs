#![warn(clippy::all)]
extern crate core;

#[macro_use]
mod test_utils;

use test_utils::TestedDialects;
use sqlparser::dialect::{CustomDialect};

fn custom_dialect() -> TestedDialects {
    TestedDialects {
        dialects: vec![Box::new(CustomDialect {})],
        options: None,
    }
}

#[test]
fn parse_foreach() {
    let dialect = custom_dialect();

    assert!(dialect.parse_sql_statements(
        "FOREACH payload.arr AS ele FROM \"/topic/#\""
    ).is_ok());

    assert!(dialect.parse_sql_statements(
        "FOREACH payload.arr AS ele FROM \"/topic/#\" WHERE payload.version > 2"
    ).is_ok());

    assert!(dialect.parse_sql_statements(
        "FOREACH payload.arr AS ele WHEN ele.count = 1 FROM \"/topic/#\" WHERE payload.version > 2"
    ).is_ok());

    assert!(dialect.parse_sql_statements(
        "FOREACH payload.arr AS ele RETURN ele.name, ele WHEN ele.count = 1 FROM \"/topic/#\" WHERE payload.version > 2"
    ).is_ok());


    dialect.verified_stmt(
        "FOREACH payload.arr AS ele RETURN ele.name, ele.count WHEN ele.count = 1 FROM \"/topic/#\" WHERE payload.version > 2"
    );
}