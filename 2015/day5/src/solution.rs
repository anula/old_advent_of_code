use std::io::{BufRead, BufReader, Write};
//use std::cmp::max;
//use std::cmp::min;
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
//use std::collections::HashMap;
//use rand::Rng;
//use itertools::Itertools;
use std::collections::VecDeque;

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
struct AString(String);

impl AString {
    const VOWELS: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
    const FORBIDDEN: [&'static str; 4] = ["ab", "cd", "pq", "xy"];

    fn is_nice(&self) -> bool {
        let mut vowels = 0;
        let mut double = false;
        let mut forbidden = false;

        let mut prev = '?';
        for c in self.0.chars() {
            if Self::VOWELS.contains(&c) {
                vowels += 1;
            }
            if c == prev {
                double = true;
            }
            let wr = prev.to_string() + &c.to_string();
            if Self::FORBIDDEN.contains(&wr.as_str()) {
                forbidden = true;
                break;
            }
            prev = c;
        }

        !forbidden && (vowels >= 3) && double
    }

    fn is_nice2(&self) -> bool {
        let mut double_double = false;
        let mut one_between = false;

        let mut last_two = VecDeque::new();
        last_two.push_back('?');
        last_two.push_back('?');

        let mut doubles = HashSet::new();

        for c in self.0.chars() {
            if last_two[0] == c {
                one_between = true;
            }
            let d = [last_two[1], c].iter().collect::<String>();
            if doubles.contains(&d) {
                double_double = true;
            }
            let prev = last_two.iter().take(2).collect::<String>();
            doubles.insert(prev);

            last_two.push_back(c);
            last_two.pop_front();
        }

        double_double && one_between
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    strings: Vec<AString>,
}

impl Solution {
    fn from_input<'a, I>(lines: I) -> Self
    where
        I: Iterator<Item = &'a str>,
    {
        let mut strings = vec![];
        for l in lines {
            let line = l.trim();
            strings.push(AString(line.to_string()));
        }

        Solution { strings }
    }

    fn solve1(&self) -> usize {
        self.strings.iter().filter(|s| s.is_nice()).count()
    }

    fn solve2(&self) -> usize {
        self.strings.iter().filter(|s| s.is_nice2()).count()
    }
}

fn solve1<'a, I, W: Write>(input_lines: I, mut output: W)
where
    I: Iterator<Item = &'a str>,
{
    let solution = Solution::from_input(input_lines);

    writeln!(output, "{}", solution.solve1()).unwrap();
}

fn solve2<'a, I, W: Write>(input_lines: I, mut output: W)
where
    I: Iterator<Item = &'a str>,
{
    let solution = Solution::from_input(input_lines);

    writeln!(output, "{}", solution.solve2()).unwrap();
}

pub fn main() {
    let stdin = std::io::stdin();
    let stdout = std::io::stdout();

    let input_lines: Vec<String> = BufReader::new(stdin.lock())
        .lines()
        .map(|l| l.unwrap())
        .collect();
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
        test_star1("ugknbfddgicrmopn", "1");
        test_star1("jchzalrnumimnmhp", "0");
        test_star1("haegwjzuvuyypxyu", "0");
        test_star1("dvszwmarrgswjxmb", "0");
    }

    #[test]
    fn sample_star2() {
        test_star2("qjhvhtzxzqqjkmpb", "1");
        test_star2("xxyxx", "1");
        test_star2("uurcxstgmygtbstg", "0");
        test_star2("ieodomkazucvgmuy", "0");
    }
}
