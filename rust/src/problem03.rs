fn priority(letter: u8) -> u32 {
    if letter >= b'a' {
        u32::from(letter - b'a' + 1)
    } else {
        u32::from(letter - b'A' + 27)
    }
}

#[test]
fn priority_should_be_valid() {
    assert_eq!(1, priority(b'a'));
    assert_eq!(26, priority(b'z'));
    assert_eq!(27, priority(b'A'));
    assert_eq!(52, priority(b'Z'));
}

pub fn solve_a(lines: &mut dyn Iterator<Item = String>) -> u32 {
    use std::collections::HashSet;

    let mut result = 0;
    for line in lines {
        let line = line.as_bytes();
        let mut set = HashSet::new();
        for i in 0..line.len()/2 {
            set.insert(line[i]);
        }

        let mut i = line.len()/2;
        while !set.contains(&line[i]) {
            i += 1
        }

        result += priority(line[i])
    }

    result
}

#[test]
fn solve_a_with_sample_data_returns_157() {
    let sample = indoc::indoc!("
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(157, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item = String>) -> u32 {
    use std::collections::HashSet;

    let mut result = 0;
    loop {
        if let Some(line1) = lines.next() {
            if let Some(line2) = lines.next() {
                if let Some(line3) = lines.next() {
                    let set1 = line1.as_bytes().iter().collect::<HashSet<_>>();
                    let set2 = line2.as_bytes().iter().collect::<HashSet<_>>();
                    let set3 = line3.as_bytes().iter().collect::<HashSet<_>>();

                    let intersection = &(&set1 & &set2) & &set3;
                    let common_badge = **intersection.iter().next().unwrap();

                    result += priority(common_badge);

                    continue;
                }
            }
        }

        break;
    }

    result
}

#[test]
fn solve_b_with_sample_data_returns_70() {
    let sample = indoc::indoc!("
        vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(70, actual);
}
