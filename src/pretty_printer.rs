use ast::*;
use std::fmt::Write;
use std::io::Result;

pub trait PrettyPrinter {
    fn pretty_print(&self, indent_level: i8) -> Result<String>;
}

impl PrettyPrinter for SelectStruct {
    fn pretty_print(&self, indent_level: i8) -> Result<String> {
        let mut buf = String::new();
        write!(buf, " SELECT {}\n", &self.columns.first().unwrap().pretty_print(indent_level)?);
        for col in &self.columns[1..] {
            write!(buf, "      , {}\n", &col.pretty_print(indent_level)?);
        }
        write!(buf, "   FROM {}\n", self.table.pretty_print(indent_level)?);
        for join in &self.joins {
            write!(buf, "   {}\n", join.pretty_print(indent_level)?);
        }
        if let Some(ref e) = self.filter {
            write!(buf, "  WHERE {}", e.pretty_print(indent_level)?);
        }
        Ok(buf)
    }
}

impl PrettyPrinter for AliasedIdentifierT {
    fn pretty_print(&self, indent_level: i8) -> Result<String> {
        Ok(match self.alias {
            Some(ref s) => format!("{} {}", self.name.pretty_print(indent_level)?, s),
            None => format!("{}", &self.name.pretty_print(indent_level)?),
        })
    }
}

impl PrettyPrinter for QualifiedIdentifierT {
    fn pretty_print(&self, indent_level: i8) -> Result<String> {
        Ok(match self.qualifier {
            Some(ref s) => format!("{}.{}", s, self.name),
            None => format!("{}", self.name),
        })
    }
}

impl PrettyPrinter for JoinSpecificationT {
    fn pretty_print(&self, indent_level: i8) -> Result<String> {
        let j = match self.typ {
            JoinType::INNER => "JOIN",
            _ => "<unknown>"
        };
        Ok(format!("{} {} ON {}", j, self.table.pretty_print(indent_level)?, self.on.pretty_print(indent_level)?))
    }
}

impl PrettyPrinter for TwoSidedExpressionT {
    fn pretty_print(&self, indent_level: i8) -> Result<String> {
        let j = match self.operator {
            Operator::EQ => "=",
            _ => "<unknown>"
        };
        Ok(format!("{} {} {}", self.v1.pretty_print(indent_level)?, j, self.v2.pretty_print(indent_level)?))
    }
}