use std::io::{BufRead, BufReader, Write};
//use std::cmp::max;
//use std::cmp::min;
//use regex::Regex;
//use lazy_static::lazy_static;
//use std::collections::HashSet;
//use std::collections::HashMap;
//use rand::Rng;
//use itertools::Itertools;
//use std::collections::VecDeque;

#[allow(unused_macros)]
macro_rules! dprintln {
    ( $( $x:expr ),* ) => {
        {
	    #[cfg(test)]
            println!($($x), *);
        }
    };
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
}

impl Solution {
    fn from_input<'a, I>(lines: I) -> Self
        where I: Iterator<Item = &'a str>
    {
        for (y, l) in lines.enumerate() {
            let line = l.trim();
        }

        Solution {
        }
    }

    fn solve1(&self) -> i64 {
        0
    }

    fn solve2(&self) -> i64 {
        0
    }
}

fn solve1<'a, I, W: Write>(input_lines: I, mut output: W)
        where I: Iterator<Item = &'a str>
{
    let solution = Solution::from_input(input_lines);

    writeln!(output, "{}", solution.solve1()).unwrap();
}

fn solve2<'a, I, W: Write>(input_lines: I, mut output: W)
        where I: Iterator<Item = &'a str>
{
    let solution = Solution::from_input(input_lines);

    writeln!(output, "{}", solution.solve2()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();
    
    let input_lines: Vec<String> = BufReader::new(stdin.lock())
        .lines().map(|l| l.unwrap()).collect();
    solve1(input_lines.iter().map(AsRef::as_ref), stdout.lock());
    solve2(input_lines.iter().map(AsRef::as_ref), stdout.lock());
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;

    fn assert_ignore_whitespaces(actual_out: Vec<u8>, output: &str) {
        let actual_out_str = String::from_utf8(actual_out).unwrap();
        let actual_outs = actual_out_str.split_whitespace().collect::<Vec<&str>>();
        let expected_outs = output.split_whitespace().collect::<Vec<&str>>();
        assert_eq!(actual_outs, expected_outs);
    }

    #[allow(dead_code)]
    fn test_star1(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve1(input.split('\n'), &mut actual_out);

        assert_ignore_whitespaces(actual_out, output);
    }

    #[allow(dead_code)]
    fn test_star2(input: &str, output: &str) {
        let mut actual_out: Vec<u8> = Vec::new();
        solve2(input.split('\n'), &mut actual_out);

        assert_ignore_whitespaces(actual_out, output);
    }

    #[allow(dead_code)]
    fn official_input() -> std::io::Lines<BufReader<File>> {
        let file = File::open("input").unwrap();
        BufReader::new(file).lines()
    }

    #[test]
    fn sample_star1() {
        test_star1(
            "",
            "0",
        );
    }
}
