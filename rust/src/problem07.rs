use text_io::scan;

struct Directory {
    size: u64,
    files: Vec<u64>,
    directories: Vec<usize>,
}

fn build_tree(lines: &mut dyn Iterator<Item=String>) -> Vec<Directory> {
    let mut directories = vec![Directory {
        size: 0,
        files: Vec::new(),
        directories: Vec::new(),
    }];
    let mut stack = vec![0];
    for line in lines {
        match line.as_str() {
            "$ cd /" => (),
            "$ ls" => (),
            "$ cd .." => {
                stack.pop();
            },
            cd_dir if cd_dir.starts_with("$ cd ") => {
                let sub_directory = Directory {
                    size: 0,
                    files: Vec::new(),
                    directories: Vec::new(),
                };
                directories.push(sub_directory);
                let sub_directory_index = directories.len() - 1;
                
                let current_directory_index = stack.pop().unwrap();
                directories[current_directory_index].directories.push(sub_directory_index);
                stack.push(current_directory_index);
                stack.push(sub_directory_index);
            },
            dir if dir.starts_with("dir ") => (),
            size_file => {
                let file_size: u64;
                scan!(size_file.bytes() => "{} ", file_size);
                
                let current_directory_index = stack.pop().unwrap();
                directories[current_directory_index].files.push(file_size);
                stack.push(current_directory_index);
            }
        }
    }
    
    directories
}

fn update_directory_sizes(tree: &mut [Directory]) {
    fn update_directory_size(tree: &mut [Directory], index: usize) {
        let files_size: u64 = tree[index].files.iter().sum();
        let mut sub_directories_size = 0u64;
        let sub_directory_indexes = tree[index].directories.clone();
        for sub_directory_index in sub_directory_indexes {
            update_directory_size(tree, sub_directory_index);
            sub_directories_size += &tree[sub_directory_index].size;
        }
        
        tree[index].size = files_size + sub_directories_size;
    }
    
    update_directory_size(tree, 0);
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> u64 {
    let mut tree = build_tree(lines);
    update_directory_sizes(&mut tree);
    
    tree.iter()
        .filter(|directory| directory.size < 100_000u64)
        .map(|directory| directory.size)
        .sum()
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
    let mut lines = crate::utils::str_to_iter(sample);
    
    let actual = solve_a(&mut lines);
    
    assert_eq!(95_437u64, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> u64 {
    let mut tree = build_tree(lines);
    update_directory_sizes(&mut tree);
    
    let used = tree[0].size;
    let unused = 70_000_000u64 - used;
    let need_free = 30_000_000u64 - unused;
    
    tree.sort_by_key(|directory| directory.size);

    tree.iter()
        .find(|directory| directory.size > need_free)
        .unwrap()
        .size
}

#[test]
fn solve_b_with_sample_data_returns_24933642() {
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
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(24_933_642u64, actual);
}

