use crate::comment;
use crate::parser::{
    ast::{Expr, RangeInterval},
    parse_scope::parse_scope, 
    parse_ident::parse_ident,
    parse_var_types::{parse_as_variable, parse_var_expr},
    tokens::{Span, FOREACH, IN, L_PAREN, R_PAREN, COMMA},
    tools::get_interval,
};
use nom::{alt, do_parse, named, opt, sep, tag, ws};

named!(pub parse_for<Span, Expr>, do_parse!(
    comment!(tag!(FOREACH)) >>
    start: get_interval >>

    comment!(tag!(L_PAREN)) >>
    ident: parse_ident >>
    opt: opt!(
        do_parse!(
            comment!(tag!(COMMA)) >>
            var :parse_ident >>
            (var)
        )
    ) >>
    comment!(tag!(R_PAREN)) >>

    comment!(tag!(IN)) >>
    expr: alt!(parse_as_variable | parse_var_expr) >>
    block: parse_scope >>
    end: get_interval >>

    (Expr::ForExpr(
        ident,
        opt,
        Box::new(expr),
        block,
        RangeInterval{start, end}
    ))
));

