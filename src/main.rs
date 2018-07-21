extern crate clap;
extern crate colored;
use clap::{App, Arg};

pub mod ast;
pub mod pretty_printer;
pub mod sql;

use colored::*;
use pretty_printer::PrettyPrinter;
use std::fs::File;
use std::io;
use std::io::{BufReader, BufRead};
use std::io::Read;

fn main() {
    let matches = App::new("SQL formatter")
        .version("1.0")
        .author("Thorsten M. <thorsten@muerell.de>")
        .about("Formats a SQL statement from a file or stdin")
        .arg(
            Arg::with_name("FILE")
                .help("Input file to use")
                .required(true)
                .index(1),
        )
        .get_matches();

    let filename = matches.value_of("FILE").unwrap();

    let mut reader: Box<BufRead> = if "-" == filename {
        Box::new(BufReader::new(io::stdin()))
    } else {
        Box::new(BufReader::new(
            File::open(filename).expect("Error opening file"),
        ))
    };

    let mut contents = String::new();
    reader.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let res = sql::SelectStmtParser::new().parse(&contents);

    match res {
        Ok(res) => println!("{}", res.pretty_print(0).unwrap()),
        Err(e) => println!("{}: {:?}", "ERROR".red(), e),
    }
}

#[test]
fn testit() {
    let expr = sql::SelectStmtParser::new().parse("SELECT column FROM dual;");
    assert_eq!(
        &format!("{:?}", expr),
        "Ok(SelectStruct { columns: [QualifiedIdentifierT { name: \"column\", qualifier: None }], table: AliasedIdentifierT { name: QualifiedIdentifierT { name: \"dual\", qualifier: None }, alias: None }, joins: [], filter: None })"
    );

    let expr = sql::SelectStmtParser::new().parse("SELECT column, column2 FROM dual ;");
    assert_eq!(
        &format!("{:?}", expr),
        "Ok(SelectStruct { columns: [QualifiedIdentifierT { name: \"column\", qualifier: None }, QualifiedIdentifierT { name: \"column2\", qualifier: None }], table: AliasedIdentifierT { name: QualifiedIdentifierT { name: \"dual\", qualifier: None }, alias: None }, joins: [], filter: None })"
    );

    let expr = sql::SelectStmtParser::new().parse("SELECT t.column, t.column2 FROM table t ;");
    assert_eq!(
        &format!("{:?}", expr),
        "Ok(SelectStruct { columns: [QualifiedIdentifierT { name: \"column\", qualifier: Some(\"t\") }, QualifiedIdentifierT { name: \"column2\", qualifier: Some(\"t\") }], table: AliasedIdentifierT { name: QualifiedIdentifierT { name: \"table\", qualifier: None }, alias: Some(\"t\") }, joins: [], filter: None })"
    );
}
