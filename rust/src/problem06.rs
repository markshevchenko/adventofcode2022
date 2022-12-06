fn detect_packet_position(chars: &[u8]) -> usize {
    let mut c1 = chars[0];
    let mut c2 = chars[1];
    let mut c3 = chars[2];
    let mut c4 = chars[3];
    
    let mut result = 4;
    while c1 == c2 || c1 == c3 || c1 == c4 || c2 == c3 || c2 == c4 || c3 == c4 {
        c1 = c2;
        c2 = c3;
        c3 = c4;
        c4 = chars[result];
        
        result += 1;
    }

    result
}

#[test]
fn detect_packet_position_should_return_valid_values() {
    assert_eq!(7, detect_packet_position(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(5, detect_packet_position(b"bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(6, detect_packet_position(b"nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(10, detect_packet_position(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(11, detect_packet_position(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

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

fn detect_message_position(chars: &[u8]) -> usize {
    let mut buffer = Vec::<u8>::new();
    chars[0..14].clone_into(&mut buffer);

    let mut result = 14;
    while !all_different(&buffer) {
        for i in 0..buffer.len() - 1 {
            buffer[i] = buffer[i + 1]
        }
        buffer[13] = chars[result];

        result += 1;
    }

    result
}

#[test]
fn detect_message_position_should_return_valid_values() {
    assert_eq!(19, detect_message_position(b"mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
    assert_eq!(23, detect_message_position(b"bvwbjplbgvbhsrlpgdmjqwftvncz"));
    assert_eq!(23, detect_message_position(b"nppdvjthqldpwncqszvftbrmjlhg"));
    assert_eq!(29, detect_message_position(b"nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
    assert_eq!(26, detect_message_position(b"zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let line = lines.next().unwrap();
    
    detect_packet_position(line.as_bytes())
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let line = lines.next().unwrap();

    detect_message_position(line.as_bytes())
}
