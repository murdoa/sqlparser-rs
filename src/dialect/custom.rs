use crate::{dialect::Dialect, parser::{Parser, ParserError}, ast::{Statement, SelectItem}, keywords::Keyword};


#[derive(Debug)]
pub struct CustomDialect {}

impl Dialect for CustomDialect {
    fn is_identifier_start(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase()
    }

    fn is_identifier_part(&self, ch: char) -> bool {
        ch.is_ascii_lowercase() || ch.is_ascii_uppercase() || ch.is_ascii_digit() || ch == '_'
    }

    fn parse_statement(&self, parser: &mut Parser) -> Option<Result<Statement, ParserError>> {
        if parser.parse_keyword(Keyword::FOREACH) {
            Some(parse_foreach(parser))
        } else {
            None
        }
    }
}

pub fn parse_foreach(parser: &mut Parser) -> Result<Statement, ParserError> {
    let select_item = parser.parse_select_item()?;

    let return_items : Option<Vec<SelectItem>> = if parser.expect_keyword(Keyword::RETURN).is_ok() {
        Some(parser.parse_projection()?)
    } else {
        None
    };

    let when_expr = if parser.expect_keyword(Keyword::WHEN).is_ok() {
        Some(parser.parse_expr()?)
    } else {
        None
    };

    parser.expect_keyword(Keyword::FROM)?;
    let from_table = parser.parse_table_and_joins()?;

    let where_expr = if parser.expect_keyword(Keyword::WHERE).is_ok() {
        Some(parser.parse_expr()?)
    } else {
        None
    };

    Ok(Statement::Foreach { select_item,
                            return_items: return_items,
                            when_expr: when_expr,
                            from_table: from_table,
                            where_expr: where_expr })
}

