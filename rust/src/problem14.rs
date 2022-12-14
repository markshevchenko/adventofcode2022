use std::iter;
use std::ops::Sub;
use std::str;
use text_io::scan;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Point {
    x: usize,
    y: usize
}

impl Point {
    fn parse(s: &str) -> Self {
        let x: usize;
        let y: usize;
        scan!(s.bytes() => "{},{}", x, y);
        Point { x, y }
    }
    
    fn new(x: usize, y: usize) -> Self {
        Point { x, y }
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: self.x - rhs.x, y: self.y - rhs.y }
    }
} 

#[test]
fn point_parse_parses_498_4() {
    let actual = Point::parse("498,4");
    
    assert_eq!(Point::new(498, 4), actual);
}

fn parse_chain(line: &str) -> Vec<Point> {
    line.split(" -> ").map(Point::parse).collect()
}

#[test]
fn parse_chain_parses_points() {
    let expected = vec![Point::new(498, 4),
                                  Point::new(498, 6),
                                  Point::new(496, 6)];
    let actual = parse_chain("498,4 -> 498,6 -> 496,6");
    
    assert_eq!(expected, actual);
}

struct Field {
    points: Vec<Vec<u8>>,
    source: Point,
}

impl Field {
    fn detect_limits(chains: &Vec<Vec<Point>>) -> (Point, Point) {
        let mut min = Point::new(500, 0);
        let mut max = Point::new(500, 0);

        for chain in chains {
            for point in chain {
                if min.x > point.x {
                    min.x = point.x;
                } else if max.x < point.x {
                    max.x = point.x;
                }

                if min.y > point.y {
                    min.y = point.y;
                } else if max.y < point.y {
                    max.y = point.y;
                }
            }
        }
        
        min.x -= 1;
        max.x += 1;

        (min, max)
    }

    fn from_chains(chains: &Vec<Vec<Point>>) -> Self {
        let (min, max) = Field::detect_limits(chains);
        let width = max.x - min.x + 1;
        let height = max.y - min.y + 1;
        let source = Point::new(500, 0) - min;
        let mut points: Vec<Vec<u8>> =
            iter::repeat(iter::repeat(b'.').take(width).collect())
                .take(height).collect();
        
        for chain in chains {
            for i in 1..chain.len() {
                let from = chain[i - 1] - min;
                let to = chain[i] - min;
                
                if from.x == to.x {
                    let range =
                        if from.y < to.y { from.y..=to.y }
                        else { to.y..=from.y };
                    
                    for y in range {
                            points[y][from.x] = b'#';
                    }
                }
                else if from.y == to.y {
                    let range =
                        if from.x < to.x { from.x..=to.x }
                        else { to.x..=from.x };

                    for x in range {
                        points[from.y][x] = b'#';
                    }
                }
                else {
                    panic!("Diagonal line")
                }
            }
        }
        
        Field { points, source }
    }
    
    fn try_pass(&mut self) -> bool {
        let mut sand = self.source;
        
        loop {
            if sand.y + 1 == self.points.len() {
                return false;
            }
            
            if self.points[sand.y + 1][sand.x] == b'.' {
                sand.y += 1;
                continue;
            } else if self.points[sand.y + 1][sand.x - 1] == b'.' {
                sand.y += 1;
                sand.x -= 1;
                continue;
            } else if self.points[sand.y + 1][sand.x + 1] == b'.' {
                sand.y += 1;
                sand.x += 1;
                continue;
            }
            
            self.points[sand.y][sand.x] = b'o';
            return true;
        }
    }
    
    fn print(&self) {
        for row in &self.points {
            println!("{}", str::from_utf8(&row).unwrap());
        }
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let chains = lines.map(|s| parse_chain(&s)).collect::<Vec<_>>();
    let mut field = Field::from_chains(&chains);
    let mut count = 0;
    
    while field.try_pass() {
        count += 1
    }
    
    count
}

#[test]
fn solve_a_with_sample_data_returns_24() {
    let sample = indoc::indoc!("
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(24, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    0
}

#[test]
fn solve_b_with_sample_data_returns_0() {
    let sample = indoc::indoc!("
        498,4 -> 498,6 -> 496,6
        503,4 -> 502,4 -> 502,9 -> 494,9");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(0, actual);
}
