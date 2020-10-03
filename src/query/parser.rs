use http::Request;
use nom::branch::alt;
use nom::bytes::complete::{escaped, tag, tag_no_case, take_while, take_while1};
use nom::character::complete::{
    alphanumeric1, anychar, char, digit1, multispace0, none_of, one_of,
};
use nom::character::{is_alphanumeric, is_space};
use nom::combinator::{map, map_res};
use nom::error::ErrorKind;
use nom::multi::{many0, many1, separated_list0};
use nom::sequence::{delimited, preceded};
use nom::{IResult, InputTakeAtPosition};

#[derive(Debug, PartialEq)]
pub struct Selector<'a> {
    pub segments: Vec<Segment<'a>>,
}

#[derive(Debug, PartialEq)]
pub enum Segment<'a> {
    Field(&'a str),
    List(ListSegment),
}

#[derive(Debug, PartialEq)]
pub enum ListSegment {
    All,
    Index(u32),
}

pub fn parse_preload(i: &str) -> IResult<&str, Vec<Selector>> {
    let parse_item = delimited(char('"'), parse_selector, char('"'));
    let parse_separator = delimited(multispace0, tag(","), multispace0);

    separated_list0(parse_separator, parse_item)(i)
}

fn parse_selector(i: &str) -> IResult<&str, Selector> {
    let parse_selector = many0(alt((parse_list_segment, parse_field_segment)));

    map(parse_selector, |items| Selector { segments: items })(i)
}

fn parse_field_segment(i: &str) -> IResult<&str, Segment> {
    let is_field_name = |c: char| c.is_alphanumeric() || c == '_';
    let field_name = take_while1(is_field_name);

    map(
        preceded(char('/'), escaped(field_name, '\\', one_of("\"n\\"))),
        |result| Segment::Field(result),
    )(i)
}

fn parse_list_segment(i: &str) -> IResult<&str, Segment> {
    map(
        alt((parse_list_segment_all, parse_list_segment_index)),
        |result| Segment::List(result),
    )(i)
}

fn parse_list_segment_all(i: &str) -> IResult<&str, ListSegment> {
    map(tag("/*"), |_| ListSegment::All)(i)
}

fn parse_list_segment_index(i: &str) -> IResult<&str, ListSegment> {
    map_res(preceded(char('/'), digit1), |result| {
        u32::from_str_radix(result, 10).map(|number| ListSegment::Index(number))
    })(i)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse_preload() {
        let result = parse_preload("\"/some_field/1234/*\", \"/other_field/*/1234\"");
        let excepted = vec![
            Selector {
                segments: vec![
                    Segment::Field("some_field"),
                    Segment::List(ListSegment::Index(1234)),
                    Segment::List(ListSegment::All),
                ],
            },
            Selector {
                segments: vec![
                    Segment::Field("other_field"),
                    Segment::List(ListSegment::All),
                    Segment::List(ListSegment::Index(1234)),
                ],
            },
        ];

        assert_eq!(result, Ok(("", excepted)))
    }

    #[test]
    fn test_parse_selector() {
        let result = parse_selector("/some_field/1234/*");
        let excepted = Selector {
            segments: vec![
                Segment::Field("some_field"),
                Segment::List(ListSegment::Index(1234)),
                Segment::List(ListSegment::All),
            ],
        };

        assert_eq!(result, Ok(("", excepted)))
    }

    #[test]
    fn test_parse_field_segment() {
        let result = parse_field_segment("/some_field");
        let excepted = Segment::Field("some_field");

        assert_eq!(result, Ok(("", excepted)))
    }

    #[test]
    fn test_parse_list_segment_all() {
        let result = parse_list_segment_all("/*");
        let excepted = ListSegment::All;

        assert_eq!(result, Ok(("", excepted)))
    }

    #[test]
    fn test_parse_list_segment_index() {
        let result = parse_list_segment_index("/123456789");
        let excepted = ListSegment::Index(123456789);

        assert_eq!(result, Ok(("", excepted)))
    }
}
