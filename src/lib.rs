use nom::{
    branch::alt, bytes::complete::is_not, character::complete::char, sequence::delimited, IResult,
};

fn paren(input: &str) -> IResult<&str, &str> {
    delimited(char('('), is_not(")"), char(')'))(input)
}

pub fn parens(s: &str) -> IResult<&str, &str> {
    delimited(char('('), alt((paren, is_not(")"))), char(')'))(s)
}

#[test]
fn test_parens() {
    assert_eq!(paren("(glot)").unwrap().1, "glot");
    assert_eq!(parens("(glot)").unwrap().1, "glot");
    assert_eq!(parens("((glot))").unwrap().1, "glot");
}
