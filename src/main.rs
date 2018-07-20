extern crate colored;

pub mod ast;
pub mod sql;

use ast::SelectStruct;
use std::fs::File;
use std::io::Read;
use colored::*;

fn main() {
    let filename = "test.sql";
    let mut f = File::open(filename).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let res = sql::SelectStmtParser::new().parse(&contents);

    match res {
        Ok(res) => match res {
            SelectStruct {
                columns: cols,
                tables: tabs,
            } => {
                println!(" SELECT {}", cols.first().unwrap());
                for col in &cols[1..] {
                    println!("      , {}", col);
                }
                println!("   FROM {}", tabs)
            }
        },
        Err(e) => println!("{}: {:?}", "ERROR".red(), e)
    }
}

#[test]
fn testit() {
    let expr = sql::SelectStmtParser::new().parse("SELECT column FROM dual ;");
    assert_eq!(
        &format!("{:?}", expr),
        "Ok(SelectStruct { columns: [\"column\"], tables: \"dual\" })"
    );

    let expr = sql::SelectStmtParser::new().parse("SELECT column, column2 FROM dual ;");
    assert_eq!(
        &format!("{:?}", expr),
        "Ok(SelectStruct { columns: [\"column\", \"column2\"], tables: \"dual\" })"
    );
}
