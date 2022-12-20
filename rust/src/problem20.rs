fn load_pairs(lines: &mut dyn Iterator<Item=String>) -> Vec<(isize, usize)> {
    let mut result = Vec::new();

    let mut index = 0;
    for line in lines {
        let shift = line.parse::<isize>().unwrap();
        result.push((shift, index));
        index += 1;
    }

    result
}

fn modulo(a: isize, b: isize) -> usize {
    (((a % b) + b) % b) as usize
}

#[test]
fn module_should_return_positive_remainders() {
    assert_eq!(-1, -21 % 4);
    assert_eq!(3, modulo(-21, 4));
}

fn source_target(pairs: &[(isize, usize)], index: usize) -> (usize, usize) {
    let source = pairs.iter().position(|&it| it.1 == index).unwrap();
    let shift = pairs[source].0;
    let target = modulo(source as isize + shift, (pairs.len() - 1) as isize);

    (source, target)
}

#[test]
fn source_target_should_process_positive_zero_and_negative_offsets() {
    let pairs = vec![(-3, 0), (-1, 1), (0, 2), (1, 3), (3, 4)];

    assert_eq!((0, 1), source_target(&pairs, 0));
    assert_eq!((1, 0), source_target(&pairs, 1));
    assert_eq!((2, 2), source_target(&pairs, 2));
    assert_eq!((3, 0), source_target(&pairs, 3));
    assert_eq!((4, 3), source_target(&pairs, 4));
}

fn round(pairs: &mut [(isize, usize)]) {
    for index in 0..pairs.len() {
        let (source, target) = source_target(&pairs, index);
        if (source == target) {
            continue;
        }

        let item = pairs[source];
        if (source < target) {
            for i in source..target {
                pairs[i] = pairs[i + 1];
            }
        } else {
            for i in (target+1..source+1).rev() {
                pairs[i] = pairs[i - 1];
            }
        }
        pairs[target] = item;
    }
}

fn get_value(pairs: &[(isize, usize)]) -> isize {
    let index_0 = pairs.iter().position(|&x| x.0 == 0).unwrap();
    let at_1000 = pairs[(index_0 + 1000) % pairs.len()].0;
    let at_2000 = pairs[(index_0 + 2000) % pairs.len()].0;
    let at_3000 = pairs[(index_0 + 3000) % pairs.len()].0;

    at_1000 + at_2000 + at_3000
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> isize {
    let mut pairs = load_pairs(lines);

    round(&mut pairs);

    get_value(&pairs)
}

#[test]
fn solve_a_with_sample_data_returns_3() {
    let sample = indoc::indoc!("
        1
        2
        -3
        3
        -2
        0
        4");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(3, actual);
}

fn mul_811589153(pairs: &mut [(isize, usize)]) {
    for i in 0..pairs.len() {
        let (offset, index) = pairs[i];
        pairs[i] = (offset * 811589153, index);
    }
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> isize {
    let mut pairs = load_pairs(lines);

    mul_811589153(&mut pairs);
    for _ in 0..10 {
        round(&mut pairs);
    }

    get_value(&pairs)
}

#[test]
fn solve_b_with_sample_data_returns_1623178306() {
    let sample = indoc::indoc!("
        1
        2
        -3
        3
        -2
        0
        4");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(1623178306, actual);
}
