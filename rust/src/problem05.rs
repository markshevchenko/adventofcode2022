use text_io::scan;

fn parse(s: &str) -> (usize, usize, usize) {
    let count: usize;
    let from: usize;
    let to: usize;

    scan!(s.bytes() => "move {} from {} to {}", count, from, to);
    (count, from, to)
}

fn read_picture(lines: &mut dyn Iterator<Item = String>) -> Vec<Vec<u8>> {
    let mut result = Vec::new();
    while let Some(line) = lines.next() {
        if &line == "" {
            break;
        }

        let line = line
            .as_bytes()
            .iter()
            .map(|b| *b)
            .collect::<Vec::<u8>>();

        if line[1] == b'1' {
            continue;
        }

        result.push(line);
    }

    result
}

fn make_stacks(picture: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let stack_count = (picture[picture.len() - 1].len() + 3)/4;
    let mut result = Vec::new();
    for _ in 0..stack_count {
        result.push(Vec::new());
    }

    for i in (0..picture.len()).rev() {
        for j in 0..stack_count {
            let index = j * 4 + 1;
            if index >= picture[i].len() {
                continue;
            }

            let c = picture[i][index];
            if c == b' ' {
                continue;
            }

            result[j].push(c);
        }
    }

    result
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> String {
    let picture = read_picture(lines);
    let mut stacks = make_stacks(&picture);

    while let Some(line) = lines.next() {
        let (count, from, to) = parse(&line);

        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    String::from_utf8(stacks.iter().map(|s| s[s.len() - 1]).collect()).unwrap()
}

#[test]
fn solve_a_with_sample_data_returns_cmz() {
    let sample = indoc::indoc!("
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!("CMZ", actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> String {
    let picture = read_picture(lines);
    let mut stacks = make_stacks(&picture);

    while let Some(line) = lines.next() {
        let (count, from, to) = parse(&line);

        let mut tmp_stack = Vec::new();
        for _ in 0..count {
            let c = stacks[from - 1].pop().unwrap();
            tmp_stack.push(c);
        }
        for _ in 0..count {
            let c = tmp_stack.pop().unwrap();
            stacks[to - 1].push(c);
        }
    }

    String::from_utf8(stacks.iter().map(|s| s[s.len() - 1]).collect()).unwrap()
}

#[test]
fn solve_b_with_sample_data_returns_mcd() {
    let sample = indoc::indoc!("
            [D]
        [N] [C]
        [Z] [M] [P]
         1   2   3

        move 1 from 2 to 1
        move 3 from 1 to 3
        move 2 from 2 to 1
        move 1 from 1 to 2");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!("MCD", actual);
}
