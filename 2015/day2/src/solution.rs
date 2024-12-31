use std::io::{BufRead, BufReader, Write};
use std::cmp::max;
use std::cmp::min;
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

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Present {
    l: i64,
    w: i64,
    h: i64,
}

impl Present {
    fn from_str(line: &str) -> Self {
        let parts: Vec<&str> = line.split('x').collect();
        Self {
            l: parts[0].parse::<i64>().unwrap(),
            w: parts[1].parse::<i64>().unwrap(),
            h: parts[2].parse::<i64>().unwrap(),
        }
    }

    fn paper_required(&self) -> i64 {
        let side1 = self.l * self.w;
        let side2 = self.w * self.h;
        let side3 = self.h * self.l;

        let min_side = min(side1, min(side2, side3));

        2 * (side1 + side2 + side3) + min_side
    }

    fn ribbon_for_wrapping(&self) -> i64 {
        2 * ((self.l + self.w + self.h) - max(self.l, max(self.w, self.h)))
    }

    fn ribbon_for_bow(&self) -> i64 {
        self.l * self.w * self.h
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    presents: Vec<Present>,
}

impl Solution {
    fn from_input<'a, I>(lines: I) -> Self
        where I: Iterator<Item = &'a str>
    {
        let mut presents = vec![];
        for l in lines {
            let line = l.trim();
            presents.push(Present::from_str(line));
        }

        Solution {
            presents,
        }
    }

    fn solve1(&self) -> i64 {
        self.presents.iter().map(|p| p.paper_required()).sum()
    }

    fn solve2(&self) -> i64 {
        self.presents.iter().map(|p| p.ribbon_for_bow() + p.ribbon_for_wrapping()).sum()
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
            "2x3x4",
            "58",
        );
        test_star1(
            "1x1x10",
            "43",
        );
    }

    #[test]
    fn sample_star2() {
        test_star2(
            "2x3x4",
            "34",
        );
        test_star2(
            "1x1x10",
            "14",
        );
    }
}
