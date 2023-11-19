use nom::{
    self,
    branch::alt,
    bytes::complete::{is_not, tag},
    character::complete::multispace0,
    sequence::{delimited, tuple},
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

fn parse_json_obj(json: &str) -> nom::IResult<&str, &str> {
    let (i, _) = tag("{")(json)?;
    let (i, _) = multispace0(i)?;
    let (i, _) = nom::multi::separated_list1(alt((tag(","), multispace0)), parse_field)(i)?;
    let (i, _) = multispace0(i)?;
    tag("}")(i)
}

fn parse_field(i: &str) -> nom::IResult<&str, (&str, &str)> {
    // TODO: Handle escape for \"
    let (i, field) = delimited(tag("\""), is_not("\""), tag("\""))(i)?;
    let (i, _) = tuple((multispace0, tag(":"), multispace0))(i)?;
    let (i, val) = delimited(tag("\""), is_not("\""), tag("\""))(i)?;
    println!("{field}: {val}");
    Ok((i, (field, val)))
}
