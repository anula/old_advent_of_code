use std::io::{BufRead, BufReader, Write};
//use std::cmp::max;
//use std::cmp::min;
//use regex::Regex;
//use lazy_static::lazy_static;
use std::collections::HashSet;
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

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
struct XY {
    x: i64,
    y: i64,
}

#[allow(dead_code)]
impl XY {
    const fn new(x: i64, y: i64) -> XY { XY {x, y} }
    const fn unew(x: usize, y: usize) -> XY { XY {x: x as i64, y: y as i64} }

    const fn add(&self, other: &XY) -> XY { XY { x: self.x + other.x, y: self.y + other.y } }
    const fn sub(&self, other: &XY) -> XY { XY { x: self.x - other.x, y: self.y - other.y } }
    const fn mul(&self, other: &XY) -> XY { XY { x: self.x * other.x, y: self.y * other.y } }

    const fn smul(&self, s: i64) -> XY { XY { x: self.x * s, y: self.y * s } }

    const fn ux(&self) -> usize { self.x as usize }
    const fn uy(&self) -> usize { self.y as usize }

    const fn step(&self, dir: &Direction) -> XY { self.add(&dir.as_coords()) }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone, Hash)]
enum Direction {
    UP,
    RIGHT,
    DOWN,
    LEFT,
}
use Direction::{UP, RIGHT, DOWN, LEFT};

#[allow(dead_code)]
impl Direction {
    const ALL: [Direction; 4] = [
        UP,
        RIGHT,
        DOWN,
        LEFT,
    ];

    const fn from_char(c: char) -> Self {
        match c {
            'v' => DOWN,
            '<' => LEFT,
            '^' => UP,
            '>' => RIGHT,
            _ => panic!("dont know that dir char"),
        }
    }

    const fn as_coords(&self) -> XY {
        match self {
            UP => XY::new(0, -1),
            RIGHT => XY::new(1, 0),
            DOWN => XY::new(0, 1),
            LEFT => XY::new(-1, 0),
        }
    }

    const fn reverse(&self) -> Self {
        match self {
            UP => DOWN,
            RIGHT => LEFT,
            DOWN => UP,
            LEFT => RIGHT,
        }
    }

    const fn from_to(from: &XY, to: &XY) -> Self {
        let diff = to.sub(from);
        match diff {
            XY { x, y } if x < 0 && y == 0 => LEFT,
            XY { x, y } if x > 0 && y == 0 => RIGHT,
            XY { x, y } if x == 0 && y > 0 => DOWN,
            XY { x, y } if x == 0 && y < 0 => UP,
            _ => panic!("Diagonal!"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Solution {
    dirs: Vec<Direction>,
}

impl Solution {
    fn from_input<'a, I>(mut lines: I) -> Self
        where I: Iterator<Item = &'a str>
    {
        let mut dirs = vec![];

        let l = lines.next().unwrap();
        let line = l.trim();

        for c in line.chars() {
            dirs.push(Direction::from_char(c));
        }

        Solution {
            dirs,
        }
    }

    fn solve1(&self) -> usize {
        let mut visited = HashSet::new();

        let mut pos = XY::new(0, 0);
        visited.insert(pos);

        for dir in &self.dirs {
            pos = pos.step(dir);
            visited.insert(pos);
        }

        visited.len()
    }

    fn solve2(&self) -> usize {
        let mut visited = HashSet::new();

        let mut santa = XY::new(0, 0);
        let mut robo = XY::new(0, 0);
        visited.insert(santa);

        for (i, dir) in self.dirs.iter().enumerate() {
            if i % 2 == 0 {
                santa = santa.step(dir);
                visited.insert(santa);
            } else {
                robo = robo.step(dir);
                visited.insert(robo);
            }
        }

        visited.len()
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
            "^v",
            "2",
        );
        test_star1(
            "^>v<",
            "4",
        );
        test_star1(
            "^v^v^v^v^v",
            "2",
        );
    }

    #[test]
    fn sample_star2() {
        test_star2(
            "^v",
            "3",
        );
        test_star2(
            "^>v<",
            "3",
        );
        test_star2(
            "^v^v^v^v^v",
            "11",
        );
    }
}
