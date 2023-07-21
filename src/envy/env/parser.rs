use super::{Env, Line, Meta};
use combine::parser::char::{digit, spaces, string, upper};
use combine::parser::repeat::take_until;
use combine::EasyParser;
use combine::Stream;
use combine::{attempt, choice, eof, many, token};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

parser! {
    fn comment[I]()(I) -> char
        where [ I: Stream<Token = char> ]
    {
        token('#')
    }
}

#[test]
fn it_comment() {
    assert_eq!(comment().easy_parse("# aaa"), Ok(('#', " aaa")));
    assert!(comment().easy_parse("aaa").is_err());
}

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

#[test]
fn it_value() {
    assert_eq!(value().easy_parse("VALUE"), Ok((String::from("VALUE"), "")));
    assert_eq!(
        value().easy_parse("{\"aaa: name\nbbbb\"}"),
        Ok((String::from("{\"aaa: name\nbbbb\"}"), ""))
    );
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

#[test]
fn it_env() {
    assert_eq!(
        env().easy_parse("KEY=VALUE"),
        Ok((Env::new("KEY", "VALUE"), ""))
    );
}

parser! {
    fn meta_encrypt[I]()(I) -> Meta where [ I: Stream<Token = char> ]
    {
        attempt(string("%ENCRYPT")).map(|_| Meta::Encrypt)
    }
}
parser! {
    fn meta_encrypted[I]()(I) -> Meta where [ I: Stream<Token = char> ]
    {
        attempt(
            string("%ENCRYPTED")
        ).with(
            token('(')
            .with(take_until::<Vec<char>, _, _>(token(')')))
            .skip(token(')'))
        ).map(|values| Meta::Encrypted(values.into_iter().fold(String::new(), |sum, next| sum + &next.to_string())))
    }
}

parser! {
    fn meta_comment[I]()(I) -> Meta where [ I: Stream<Token = char> ]
    {
        take_until(eof()).map(|v| Meta::Comment(v))
    }
}

parser! {
    fn meta_whitespaces[I]()(I) -> Meta where [ I: Stream<Token = char> ]
    {
        spaces().with(eof()).map(|_| Meta::WhiteSpaces)
    }
}

parser! {
    fn meta[I]()(I) -> Meta
        where [
            I: Stream<Token = char>
        ]
    {
        choice((
            meta_whitespaces(),
            comment().and(choice((
                        meta_encrypted(),
                        meta_encrypt(),
                        meta_comment(),
                        ))).map(|(_, meta)| meta)
       )).skip(eof())
    }
}
#[test]
fn it_meta() {
    assert_eq!(meta().easy_parse("#%ENCRYPT"), Ok((Meta::Encrypt, "")));
    assert!(meta().easy_parse("#%ENCRYPT_").is_err());
    assert_eq!(
        meta().easy_parse("#%ENCRYPTED(helloworld)"),
        Ok((Meta::Encrypted(String::from("helloworld")), ""))
    );
    assert_eq!(meta().easy_parse("   "), Ok((Meta::WhiteSpaces, "")));
    assert_eq!(
        meta().easy_parse("# ENCRYPT"),
        Ok((Meta::Comment(String::from(" ENCRYPT")), ""))
    );
}

parser! {
    fn line[I]()(I) -> Line
        where [
            I: Stream<Token = char>
        ]
    {
        spaces().with(choice((
                    meta().map(|v| Line::Meta(v)),
                    env().map(|v| Line::Env(v))
                    )))

    }
}

#[test]
fn it_line() {
    assert_eq!(
        line().easy_parse("KEY=VALUE"),
        Ok((Line::Env(Env::new("KEY", "VALUE")), ""))
    );
    assert_eq!(
        line().easy_parse("#%ENCRYPT"),
        Ok((Line::Meta(Meta::Encrypt), ""))
    );
    assert_eq!(
        line().easy_parse("#%SECRET"),
        Ok((Line::Meta(Meta::Comment(String::from("%SECRET"))), ""))
    );
    assert_eq!(
        line().easy_parse("# ENCRYPT"),
        Ok((Line::Meta(Meta::Comment(String::from(" ENCRYPT"))), ""))
    );
}

pub fn parser<P: AsRef<Path>>(path: P) -> Result<Vec<Line>, anyhow::Error> {
    let mut result = vec![];
    for v in BufReader::new(File::open(path)?).lines() {
        let ref v = v.map_err(|_| anyhow::Error::msg("Unexpected"))?;
        let token = line()
            .easy_parse(v as &str)
            .map(|v| v.0)
            .map_err(|_| anyhow::Error::msg(format!("parse error: {}", v)))?;
        result.push(token);
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
