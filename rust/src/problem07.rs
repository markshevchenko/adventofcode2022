use std::iter::Peekable;
use text_io::scan;

enum Item {
    File (String, usize),
    Directory (String, Vec<Box<Item>>),
}

fn build_file_system(lines: &mut dyn Iterator<Item=String>) -> Item {
    let line = lines.next().unwrap();
    
    if line != "$ cd /" {
        panic!("First line should be `cd /`");
    }
    
    let mut peekable = lines.peekable();
    build_directory("/".to_string(), &mut peekable)
}

fn build_directory(name: String, lines: &mut Peekable<&mut dyn Iterator<Item=String>>) -> Item {
    let mut line = lines.next().unwrap();

    if line != "$ ls" {
        panic!("First line in directory should be `ls`");
    }
    
    let mut items = Vec::new();
    while let Some(line) = lines.peek() {
        if line.starts_with("$") {
            break;
        }
        
        if line.starts_with("dir") {
            lines.next();
        }
        
        let line = lines.next().unwrap();
        let mut name = "".to_string();
        let mut size = 0usize;
        scan!(line.bytes() => "{} {}", size, name);
        
        items.push(Box::new(Item::File(name, size)));
    }
    
    Item::Directory(name, items)
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    0
}

#[test]
fn solve_a_with_sample_data_returns_95437() {
    let sample = indoc::indoc!("
        $ cd /
        $ ls
        dir a
        14848514 b.txt
        8504156 c.dat
        dir d
        $ cd a
        $ ls
        dir e
        29116 f
        2557 g
        62596 h.lst
        $ cd e
        $ ls
        584 i
        $ cd ..
        $ cd ..
        $ cd d
        $ ls
        4060174 j
        8033020 d.log
        5626152 d.ext
        7214296 k");
    
    
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    0
}
