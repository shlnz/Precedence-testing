#[macro_use]
extern crate lalrpop_util;

lalrpop_mod!(pub grammar);

fn main() {
    println!("Hello World");
}

fn assert_parse(input: &str, expect: i64) {
    let result = grammar::StartRuleParser::new()
        .parse(input)
        .unwrap();
    assert_eq!(result, expect)
}

#[test]
fn not_add() {
    assert_parse("Not 2 + 2", -5)
}

#[test]
fn add_not() {
    assert_parse("2 + Not 2", -1)
}

#[test]
fn add_paren_not() {
    assert_parse("2 + (Not 2)", -1)
}
