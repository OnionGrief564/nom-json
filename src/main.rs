use std::{collections::HashMap, error, str::FromStr};

use nom::{
    self,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    combinator::opt,
    sequence::{delimited, pair, tuple},
};

fn main() {
    let print = std::env::args().nth(1).expect("Need json as arg");
    println!("{print}");
    let json = "{             \
        \"field\": \"1\",   
        \"banana\": \"2\"    
        }";
    let a = parse_json_obj(json);
    println!("{a:?}")
}

#[derive(Debug)]
enum JsonVal {
    Str(String),
    Num(isize),
    Array(Vec<JsonVal>),
    Obj(HashMap<String, JsonVal>),
    Boolean(bool),
    Null,
}

// impl FromStr for JsonVal {
//     type Err = Box<dyn error::Error>;
//
//     fn from_str(s: &str) -> Result<Self, Self::Err> {
//         alt((
//             parse_json_obj,
//             parse_json_arr,
//             parse_json_string,
//             parse_json_num,
//         ))?
//     }
// }
//
// fn parse_json_arr(json: &str) -> nom::IResult<&str, JsonVal> {}
// fn parse_json_string(json: &str) -> nom::IResult<&str, JsonVal> {}
// fn parse_json_num(json: &str) -> nom::IResult<&str, JsonVal> {}

fn parse_json_obj(json: &str) -> nom::IResult<&str, JsonVal> {
    let (i, _) = tag("{")(json)?;
    let (i, _) = multispace0(i)?;
    let (i, list) = nom::multi::separated_list1(pair(opt(tag(",")), multispace0), parse_field)(i)?;
    let (i, _) = multispace0(i)?;
    tag("}")(i)?;
    Ok((
        i,
        JsonVal::Obj(
            list.into_iter()
                .map(|(f, v)| (f.to_string(), JsonVal::Str(v.to_string())))
                .into_iter()
                .collect(),
        ),
    ))
}

fn parse_field(i: &str) -> nom::IResult<&str, (&str, &str)> {
    // TODO: Handle escape for \"
    let (i, field) = delimited(tag("\""), is_not("\""), tag("\""))(i)?;
    let (i, _) = tuple((multispace0, tag(":"), multispace0))(i)?;
    let (i, val) = delimited(tag("\""), is_not("\""), tag("\""))(i)?;
    println!("{field}: {val}");
    Ok((i, (field, val)))
}
