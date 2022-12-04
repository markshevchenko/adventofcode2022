use text_io::scan;

fn parse(s: &str) -> ((u32, u32), (u32, u32)) {
    let a: u32;
    let b: u32;
    let c: u32;
    let d: u32;

    scan!(s.bytes() => "{}-{},{}-{}", a, b, c, d);
    ((a, b), (c, d))
}

fn inside(r1: &(u32, u32), r2: &(u32, u32)) -> bool {
    r1.0 >= r2.0 && r1.1 <= r2.1
}

pub fn solve_a(lines: &mut dyn Iterator<Item = String>) -> usize {
    lines.map(|s| parse(&s))
         .filter(|(r1, r2)| inside(r1, r2) || inside(r2, r1))
         .count()
}

#[test]
fn solve_a_with_sample_data_returns_2() {
    let sample = indoc::indoc!("
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(2, actual);
}

fn overlap(r1: &(u32, u32), r2: &(u32, u32)) -> bool {
    r1.0 <= r2.1 && r2.0 <= r1.1
}

pub fn solve_b(lines: &mut dyn Iterator<Item = String>) -> usize {
    lines.map(|s| parse(&s))
         .filter(|(r1, r2)| overlap(r1, r2))
         .count()
}

#[test]
fn solve_b_with_sample_data_returns_4() {
    let sample = indoc::indoc!("
        2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(4, actual);
}
