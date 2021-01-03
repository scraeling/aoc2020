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

    pub mod without_precedence {
        use super::*;

        pub fn evaluate(expr: &str) -> usize {
            expression(expr).unwrap().1
        }
    
        fn number(i: &str) -> IResult<&str, usize> {
            alt((
                map_res(
                    delimited(space0, digit1, space0),
                    |x: &str| x.parse::<usize>()
                ),
                brackets
            ))(i)
        }
    
        fn brackets(i: &str) -> IResult<&str, usize> {
            delimited(
                space0,
                delimited(tag("("), expression, tag(")")),
                space0
            )(i)
        }
    
        fn expression(i: &str) -> IResult<&str, usize> {
            let (rest, first) = number(i)?;
            fold_many0(
                pair(alt((tag("*"), tag("+"))), number),
                first,
                |acc, (op, val)| match op {
                    "*" => acc * val,
                    "+" => acc + val,
                    _ => acc
                }
            )(rest)
        }
    }

    pub mod addition_precedence {
        use super::*;

        fn number(i: &str) -> IResult<&str, usize> {
            alt((
                map_res(
                    delimited(space0, digit1, space0),
                    |x: &str| x.parse::<usize>()
                ),
                brackets
            ))(i)
        }

        fn addition(i: &str) -> IResult<&str, usize> {
            let (rest, first) = number(i)?;
            fold_many0(
                pair(tag("+"), number),
                first,
                |acc, (_, val)| acc + val
            )(rest)
        }

        fn brackets(i: &str) -> IResult<&str, usize> {
            delimited(
                space0,
                delimited(tag("("), expression, tag(")")),
                space0
            )(i)
        }

        fn expression(expr: &str) -> IResult<&str, usize> {
            let (rest, first) = addition(expr).unwrap();
    
            fold_many0(
                pair(tag("*"), addition),
                first,
                |acc, (_, val)| acc * val
            )(rest)
        }

        pub fn evaluate(expr: &str) -> usize {
            expression(expr).unwrap().1
        }
    }
   
}

fn part_1(input: &str) -> usize {
    input
    .lines()
    .map(parser::without_precedence::evaluate)
    .sum()
}

fn part_2(input: &str) -> usize {
    input
    .lines()
    .map(parser::addition_precedence::evaluate)
    .sum()
}

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let p1_answer = time!(part_1(&input));
    let p2_answer = time!(part_2(&input));

    println!("Sum of all expressions: {}", p1_answer);
    println!("Sum of all expressions with addition having precedence: {}", p2_answer);
}
