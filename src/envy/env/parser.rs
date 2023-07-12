use super::{Env, Line, Meta};
use combine::parser::char::{digit, string, upper};
use combine::parser::repeat::take_until;
use combine::EasyParser;
use combine::Stream;
use combine::{choice, eof, many, token};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

parser! {
    fn key[I]()(I) -> String
        where [
            I: Stream<Token = char>
        ]
    {
        upper()
        .and(many::<Vec<_>, _, _>(
            upper().or(digit()).or(token('_'))
        ))
        .map(|(head,tail)|
            tail.into_iter().fold(head.to_string(), |sum, next| sum + &next.to_string())
        )
    }
}

parser! {
    fn value[I]()(I) -> String
        where [
            I: Stream<Token = char>
        ]
    {
        take_until::<Vec<char>, _, _>(eof())
            .map(|values| values.into_iter().fold(String::new(), |sum, next| sum + &next.to_string()))
    }
}

parser! {
    fn env[I]()(I) -> Env
        where [
            I: Stream<Token = char>
        ]
    {
        key().and(token('=')).and(value()).map(|((key, _), value)| Env::new(key, value))
    }
}

parser! {
    fn meta[I]()(I) -> Meta
        where [
            I: Stream<Token = char>
        ]
    {
        token('%').and(
            choice(( string("SECRET").map(|_| Meta::Secret),))
            ).and(eof()).map(|((_, meta), _)| meta)
    }
}

parser! {
    fn line[I]()(I) -> Line
        where [
            I: Stream<Token = char>
        ]
    {
        choice((
                meta().map(|v| Line::Meta(v)),
                env().map(|v| Line::Env(v))
               ))
    }
}

pub fn parser<P: AsRef<Path>>(path: P) -> Result<Vec<Line>, anyhow::Error> {
    let mut result = vec![];
    for v in BufReader::new(File::open(path)?).lines() {
        if let Ok(ref v) = v {
            if let Ok(v) = line().easy_parse(v as &str).map(|v| v.0) {
                result.push(v);
            }
        }
    }
    Ok(result)
}

pub fn parser_ignore_meta<P: AsRef<Path>>(path: P) -> Result<Vec<Env>, anyhow::Error> {
    let mut result = vec![];
    for v in BufReader::new(File::open(path)?).lines() {
        if let Ok(ref v) = v {
            if let Ok(Line::Env(env)) = line().easy_parse(v as &str).map(|v| v.0) {
                result.push(env);
            }
        }
    }
    Ok(result)
}

#[test]
fn it_key() {
    assert_eq!(
        key().easy_parse("KEY=VALUE"),
        Ok((String::from("KEY"), "=VALUE"))
    );
    assert_eq!(
        key().easy_parse("KEY_KEY=VALUE"),
        Ok((String::from("KEY_KEY"), "=VALUE"))
    );
    assert!(key().easy_parse("key=VALUE").is_err());
    assert!(key().easy_parse("1KEY=VALUE").is_err());
}

#[test]
fn it_value() {
    assert_eq!(value().easy_parse("VALUE"), Ok((String::from("VALUE"), "")));
    assert_eq!(
        value().easy_parse("{\"aaa: name\nbbbb\"}"),
        Ok((String::from("{\"aaa: name\nbbbb\"}"), ""))
    );
}

#[test]
fn it_environment() {
    assert_eq!(
        env().easy_parse("KEY=VALUE"),
        Ok((Env::new("KEY", "VALUE"), ""))
    );
}

#[test]
fn it_meta() {
    assert_eq!(meta().easy_parse("%SECRET"), Ok((Meta::Secret, "")));
    assert!(meta().easy_parse("%SECRET_").is_err());
}

#[test]
fn it_line() {
    assert_eq!(
        line().easy_parse("KEY=VALUE"),
        Ok((Line::Env(Env::new("KEY", "VALUE")), ""))
    );
    assert_eq!(
        line().easy_parse("%SECRET"),
        Ok((Line::Meta(Meta::Secret), ""))
    );
}
