use std::collections::HashMap;

use nom::{
    self,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::{digit1, multispace0},
    combinator::{map, map_res, opt},
    multi::separated_list1,
    number::complete::double,
    sequence::{delimited, pair, preceded, terminated, tuple},
};

fn main() {
    let print = std::env::args().nth(1).expect("Need json as arg");
    println!("{print}");
    // let json = "{             \
    //     \"field\": \"1\",
    //     \"banana\": \"2\"
    //     }";
    let json = "[\"1\", \"2\"]";
    let a = alt((
        parse_json_obj,
        parse_json_arr,
        parse_json_num,
        parse_json_string,
    ))(json);
    println!("{a:#?}")
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
//
fn parse_json_arr(json: &str) -> nom::IResult<&str, JsonVal> {
    let (i, fields) = preceded(
        pair(tag("["), multispace0),
        terminated(
            separated_list1(pair(opt(tag(",")), multispace0), parse_json_string),
            pair(multispace0, tag("]")),
        ),
    )(json)?;
    Ok((i, JsonVal::Array(fields.into_iter().collect())))
}

fn parse_json_string(json: &str) -> nom::IResult<&str, JsonVal> {
    map(delimited(tag("\""), is_not("\""), tag("\"")), |v: &str| {
        JsonVal::Str(v.to_string())
    })(json)
}

fn parse_json_num(json: &str) -> nom::IResult<&str, JsonVal> {
    map(double, |d| (JsonVal::Num(d)))(json)
}

fn parse_json_obj(json: &str) -> nom::IResult<&str, JsonVal> {
    let (i, fields) = preceded(
        pair(tag("{"), multispace0),
        terminated(
            separated_list1(pair(opt(tag(",")), multispace0), parse_field),
            pair(multispace0, tag("}")),
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
    let (i, field) = delimited(tag("\""), is_not("\""), tag("\""))(i)?;
    let (i, _) = tuple((multispace0, tag(":"), multispace0))(i)?;
    let (i, val) = parse_json_string(i)?;
    println!("{field}: {val:#?}");
    Ok((i, (field, val)))
}
