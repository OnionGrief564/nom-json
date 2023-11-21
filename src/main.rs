use std::collections::HashMap;

use nom::{
    self,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{char, multispace0},
    combinator::map,
    error::context,
    multi::separated_list0,
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated, tuple},
};

fn main() {
    let json = std::env::args().nth(1).expect("Need json as arg");
    let (_, result) = parse_json(&json).unwrap();
    println!("{result:#?}")
}

#[derive(Debug)]
enum JsonVal {
    Str(String),
    Num(f64),
    Array(Vec<JsonVal>),
    Obj(HashMap<String, JsonVal>),
    Boolean(bool),
    Null,
}

fn parse_json(json: &str) -> nom::IResult<&str, JsonVal> {
    alt((
        parse_json_arr,
        parse_json_obj,
        parse_json_num,
        parse_json_string,
        parse_json_boolean,
        parse_json_null,
    ))(json)
}
fn parse_json_arr(json: &str) -> nom::IResult<&str, JsonVal> {
    let (i, vals) = context(
        "array",
        preceded(
            pair(char('['), multispace0),
            terminated(
                separated_list0(pair(char(','), multispace0), parse_json),
                pair(multispace0, char(']')),
            ),
        ),
    )(json)?;
    Ok((i, JsonVal::Array(vals.into_iter().collect())))
}

fn parse_json_string(json: &str) -> nom::IResult<&str, JsonVal> {
    map(delimited(char('"'), is_not("\""), char('"')), |v: &str| {
        JsonVal::Str(v.to_string())
    })(json)
}

fn parse_json_boolean(json: &str) -> nom::IResult<&str, JsonVal> {
    alt((
        map(tag("true"), |_| JsonVal::Boolean(true)),
        map(tag("false"), |_| JsonVal::Boolean(false)),
    ))(json)
}

fn parse_json_null(json: &str) -> nom::IResult<&str, JsonVal> {
    map(tag("null"), |_| JsonVal::Null)(json)
}

fn parse_json_num(json: &str) -> nom::IResult<&str, JsonVal> {
    map(double, |d| (JsonVal::Num(d)))(json)
}

fn parse_json_obj(json: &str) -> nom::IResult<&str, JsonVal> {
    let (i, fields) = preceded(
        pair(char('{'), multispace0),
        terminated(
            separated_list0(pair(char(','), multispace0), parse_field),
            pair(multispace0, char('}')),
        ),
    )(json)?;

    Ok((
        i,
        JsonVal::Obj(
            fields
                .into_iter()
                .map(|(f, v)| (f.to_string(), v))
                .into_iter()
                .collect(),
        ),
    ))
}

fn parse_field(i: &str) -> nom::IResult<&str, (&str, JsonVal)> {
    // TODO: Handle escape for \"
    let (i, field) = delimited(char('"'), is_not("\""), char('"'))(i)?;
    let (i, _) = tuple((multispace0, char(':'), multispace0))(i)?;
    let (i, val) = parse_json(i)?;
    Ok((i, (field, val)))
}
