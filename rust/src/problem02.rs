pub fn solve(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut sums = vec![];
    let mut sum = 0;

    while let Some(line) = lines.next() {
        if line.is_empty() {
            sums.push(sum);
            sum = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            sum += calories;
        }
    }

    sums.push(sum);
    sums.sort_by(|a, b| b.cmp(a));

    sums.into_iter().take(3).sum()
}

#[test]
fn solve_should_return_24000_with_sample_data() {
    let example = indoc::indoc!("
        1000
        2000
        3000

        4000

        5000
        6000

        7000
        8000
        9000

        10000
    ");
    let mut lines = crate::utils::str_to_iter(example);

    let actual = solve(&mut lines);

    assert_eq!(45000, actual);
}