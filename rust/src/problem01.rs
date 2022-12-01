pub fn solve(lines: &mut dyn Iterator<Item = String>) -> u32 {
    let mut total_max_calories = 0;
    let mut current_calories = 0;
    
    while let Some(line) = lines.next() {
        if line.is_empty() {
            if total_max_calories < current_calories {
                total_max_calories = current_calories
            }
            
            current_calories = 0;
        } else {
            let calories = line.parse::<u32>().unwrap();
            current_calories += calories;
        }
    }

    if total_max_calories < current_calories {
        total_max_calories = current_calories
    }

    total_max_calories
}

#[test]
fn solve_should_return_24000_with_sample_data() {
    let example = vec![
        "1000", "2000", "3000", "",
        "4000", "",
        "5000", "6000", "",
        "7000", "8000", "9000", "",
        "10000"];
    
    let actual = solve(&mut crate::utils::strvec_to_iter(&example));
    
    assert_eq!(24000, actual);
}