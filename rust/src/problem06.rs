fn all_different(chars: &[u8]) -> bool {
    for i in 0..chars.len() - 1 {
        for j in (i + 1)..chars.len() {
            if chars[i] == chars[j] {
                return false;
            }
        }
    }
    
    return true;
}

fn get_marker_position(chars: &[u8], marker_size: usize) -> usize {
    for result in marker_size..chars.len() {
        let buffer = &chars[result - marker_size..result];

        if all_different(buffer) {
            return result;
        }
    }

    marker_size
}

#[test]
fn get_marker_position_should_return_valid_values() {
    assert_eq!(7, get_marker_position(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 4));
    assert_eq!(5, get_marker_position(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 4));
    assert_eq!(6, get_marker_position(b"nppdvjthqldpwncqszvftbrmjlhg", 4));
    assert_eq!(10, get_marker_position(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 4));
    assert_eq!(11, get_marker_position(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 4));

    assert_eq!(19, get_marker_position(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb", 14));
    assert_eq!(23, get_marker_position(b"bvwbjplbgvbhsrlpgdmjqwftvncz", 14));
    assert_eq!(23, get_marker_position(b"nppdvjthqldpwncqszvftbrmjlhg", 14));
    assert_eq!(29, get_marker_position(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 14));
    assert_eq!(26, get_marker_position(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 14));
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let line = lines.next().unwrap();

    get_marker_position(line.as_bytes(), 4)
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let line = lines.next().unwrap();

    get_marker_position(line.as_bytes(), 14)
}
