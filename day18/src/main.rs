use std::fs::read_to_string;
use timer::time;

mod parser {
    use nom::{
        IResult, branch::alt, 
        bytes::complete::tag, 
        character::complete::{digit1, space0}, 
        combinator::map_res, 
        multi::fold_many0, 
        sequence::{delimited, pair}
    };

    fn brackets<F>(i: &str, expr_kind: F) -> IResult<&str, usize> 
        where F: Fn(&str) -> IResult<&str, usize>
    {
        delimited(
            space0,
            delimited(tag("("), expr_kind, tag(")")),
            space0
        )(i)
    }

    fn term<F>(i: &str, expr_kind: F) -> IResult<&str, usize> 
        where F: Fn(&str) -> IResult<&str, usize>
    {
        alt((
            map_res(
                delimited(space0, digit1, space0),
                |x: &str| x.parse::<usize>()
            ),
            |x| brackets(x, &expr_kind)
        ))(i)
    }

    pub fn expression(i: &str) -> IResult<&str, usize> {
        let (rest, first) = term(i, &expression)?;
        fold_many0(
            pair(
                alt((tag("*"), tag("+"))),
                |x| term(x, &expression)
            ),
            first,
            |acc, (op, val)| match op {
                "*" => acc * val,
                "+" => acc + val,
                _ => acc
            }
        )(rest)
    }

    fn addition(i: &str) -> IResult<&str, usize> {
        let (rest, first) = term(i, &expression_with_precedence)?;
        fold_many0(
            pair(tag("+"), |x| term(x, &expression_with_precedence)),
            first,
            |acc, (_, val)| acc + val
        )(rest)
    }

    pub fn expression_with_precedence(expr: &str) -> IResult<&str, usize> {
        let (rest, first) = addition(expr).unwrap();

        fold_many0(
            pair(tag("*"), addition),
            first,
            |acc, (_, val)| acc * val
        )(rest)
    }
}

fn part_1(input: &str) -> usize {
    input
    .lines()
    .map(|e| parser::expression(e).unwrap().1)
    .sum()
}

fn part_2(input: &str) -> usize {
    input
    .lines()
    .map(|e| parser::expression_with_precedence(e).unwrap().1)
    .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = time!(part_1(&input));
    let p2_answer = time!(part_2(&input));

    println!("Sum of all expressions: {}", p1_answer);
    println!("Sum of all expressions with addition having precedence: {}", p2_answer);
}
