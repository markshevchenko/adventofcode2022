#[derive(Clone, Copy, PartialEq, Debug)]
enum Op {
    Noop,
    AddX(i32),
}

impl Op {
    fn cycles(self) -> usize {
        match self {
            Op::Noop => 1,
            Op::AddX(_) => 2,
        }
    }
}

fn parse(line: &str) -> Op {
    if line.starts_with("noop") {
        return Op::Noop;
    }

    if line.starts_with("addx") {
        let parameter = line[5..].parse::<i32>().unwrap();
        return Op::AddX(parameter);
    }

    panic!("Unrecognized operation");
}

#[test]
fn parse_should_return_valid_values() {
    assert_eq!(Op::Noop, parse("noop"));
    assert_eq!(Op::AddX(-3), parse("addx -3"));
    assert_eq!(Op::AddX(10), parse("addx 10"));
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> i32 {
    let mut cycle = 1;
    let mut x = 1;
    let mut accumulator = 0;
    for line in lines {
        let op = parse(&line);

        for _ in 0..op.cycles() {
            if (cycle - 20) % 40 == 0 {
                accumulator += cycle * x
            }

            cycle += 1
        }

        if let Op::AddX(addendum) = op {
            x += addendum
        }
    }

    accumulator
}

#[test]
fn solve_a_with_sample_data_returns_13140() {
    let sample = indoc::indoc!("
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(13140, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> String {
    let mut crt = vec![vec![' '; 40]; 6];
    let mut row = 0;
    let mut column = 0;
    let mut cycle = 1;
    let mut x: isize = 1;
    for line in lines {
        let op = parse(&line);

        for _ in 0..op.cycles() {
            if column as isize >= x - 1 && column as isize <= x + 1 {
                crt[row][column] = '#';
            } else {
                crt[row][column] = '.';
            }

            cycle += 1;
            column += 1;
            if column == 40 {
                column = 0;
                row += 1;
            }
        }

        if let Op::AddX(addendum) = op {
            x += addendum as isize
        }
    }

    crt.iter()
       .map(|v| v.into_iter().collect::<String>())
       .collect::<Vec<_>>()
       .join("\n")
}

#[test]
fn solve_b_with_sample_data_returns_1() {
    let sample = indoc::indoc!("
        addx 15
        addx -11
        addx 6
        addx -3
        addx 5
        addx -1
        addx -8
        addx 13
        addx 4
        noop
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx 5
        addx -1
        addx -35
        addx 1
        addx 24
        addx -19
        addx 1
        addx 16
        addx -11
        noop
        noop
        addx 21
        addx -15
        noop
        noop
        addx -3
        addx 9
        addx 1
        addx -3
        addx 8
        addx 1
        addx 5
        noop
        noop
        noop
        noop
        noop
        addx -36
        noop
        addx 1
        addx 7
        noop
        noop
        noop
        addx 2
        addx 6
        noop
        noop
        noop
        noop
        noop
        addx 1
        noop
        noop
        addx 7
        addx 1
        noop
        addx -13
        addx 13
        addx 7
        noop
        addx 1
        addx -33
        noop
        noop
        noop
        addx 2
        noop
        noop
        noop
        addx 8
        noop
        addx -1
        addx 2
        addx 1
        noop
        addx 17
        addx -9
        addx 1
        addx 1
        addx -3
        addx 11
        noop
        noop
        addx 1
        noop
        addx 1
        noop
        noop
        addx -13
        addx -19
        addx 1
        addx 3
        addx 26
        addx -30
        addx 12
        addx -1
        addx 3
        addx 1
        noop
        noop
        noop
        addx -9
        addx 18
        addx 1
        addx 2
        noop
        noop
        addx 9
        noop
        noop
        noop
        addx -1
        addx 2
        addx -37
        addx 1
        addx 3
        noop
        addx 15
        addx -21
        addx 22
        addx -6
        addx 1
        noop
        addx 2
        addx 1
        noop
        addx -10
        noop
        noop
        addx 20
        addx 1
        addx 2
        addx 2
        addx -6
        addx -11
        noop
        noop
        noop");
    let mut lines = crate::utils::str_to_iter(sample);
    let expected = indoc::indoc!("
        ##..##..##..##..##..##..##..##..##..##..
        ###...###...###...###...###...###...###.
        ####....####....####....####....####....
        #####.....#####.....#####.....#####.....
        ######......######......######......####
        #######.......#######.......#######.....");

    let actual = solve_b(&mut lines);

    assert_eq!(expected, actual);
}
