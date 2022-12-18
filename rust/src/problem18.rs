use std::collections::HashSet;
use text_io::scan;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct Cube {
    x: usize,
    y: usize,
    z: usize,
}

fn parse_line(line: &String) -> Cube {
    let x: usize;
    let y: usize;
    let z: usize;
    
    scan!(line.bytes() => "{},{},{}", x, y, z);
    
    Cube { x: x + 1, y: y + 1, z: z + 1 }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> u32 {
    let mut cubes = HashSet::new();
    
    for line in lines {
        cubes.insert(parse_line(&line));
    }
    
    let mut total_count = 0;
    
    for cube in &cubes {
        let mut count = 6;
        if cube.x > 0 && cubes.contains(&Cube { x: cube.x - 1, y: cube.y, z: cube.z }) {
            count -= 1;
        }
        if cubes.contains(&Cube { x: cube.x + 1, y: cube.y, z: cube.z }) {
            count -= 1;
        }
        if cube.y > 0 && cubes.contains(&Cube { x: cube.x, y: cube.y - 1, z: cube.z }) {
            count -= 1;
        }
        if cubes.contains(&Cube { x: cube.x, y: cube.y + 1, z: cube.z }) {
            count -= 1;
        }
        if cube.z > 0 && cubes.contains(&Cube { x: cube.x, y: cube.y, z: cube.z - 1 }) {
            count -= 1;
        }
        if cubes.contains(&Cube { x: cube.x, y: cube.y, z: cube.z + 1 }) {
            count -= 1;
        }
        
        total_count += count;
    }
    
    total_count
}

#[test]
fn solve_a_with_sample_data_returns_64() {
    let sample = indoc::indoc!("
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(64, actual);
}

#[derive(PartialEq)]
enum Content {
    Empty,
    Lava,
    Fill,
}

fn parse_volume(lines: &mut dyn Iterator<Item=String>) -> Vec<Vec<Vec<Content>>> {
    let mut cubes = Vec::new();

    for line in lines {
        cubes.push(parse_line(&line));
    }

    let mut min = cubes[0];
    let mut max = cubes[0];

    for i in 1..cubes.len() {
        if min.x > cubes[i].x {
            min.x = cubes[i].x;
        }

        if max.x < cubes[i].x {
            max.x = cubes[i].x;
        }

        if min.y > cubes[i].y {
            min.y = cubes[i].y;
        }

        if max.y < cubes[i].y {
            max.y = cubes[i].y;
        }

        if min.z > cubes[i].z {
            min.z = cubes[i].z;
        }

        if max.z < cubes[i].z {
            max.z = cubes[i].z;
        }
    }
    
    min.x -= 1;
    min.y -= 1;
    min.z -= 1;
    max.x += 1;
    max.y += 1;
    max.z += 1;

    let mut volume = Vec::new();

    for x in min.x..=max.x {
        volume.push(Vec::new());

        for y in min.y..=max.y {
            volume[x - min.x].push(Vec::new());

            for _z in min.z..=max.z {
                volume[x - min.x][y - min.y].push(Content::Empty);
            }
        }
    }

    for cube in cubes {
        volume[cube.x - min.x][cube.y - min.y][cube.z - min.z] = Content::Lava;
    }
    
    volume
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> u32 {
    let mut volume = parse_volume(lines);
    let mut stack = Vec::new();
    stack.push(Cube { x: 0, y: 0, z: 0 });

    let mut total_count = 0;
    while let Some(current) = stack.pop() {
        if volume[current.x][current.y][current.z] == Content::Fill {
            continue;
        }
        volume[current.x][current.y][current.z] = Content::Fill;
        
        if current.x > 0 {
            if volume[current.x - 1][current.y][current.z] == Content::Empty {
                stack.push(Cube { x: current.x - 1, y: current.y, z: current.z });
            } else if volume[current.x - 1][current.y][current.z] == Content::Lava {
                total_count += 1;
            }
        }

        if current.x < volume.len() - 1 {
            if volume[current.x + 1][current.y][current.z] == Content::Empty {
                stack.push(Cube { x: current.x + 1, y: current.y, z: current.z });
            } else if volume[current.x + 1][current.y][current.z] == Content::Lava {
                total_count += 1;
            }
        }

        if current.y > 0 {
            if volume[current.x][current.y - 1][current.z] == Content::Empty {
                stack.push(Cube { x: current.x, y: current.y - 1, z: current.z });
            } else if volume[current.x][current.y - 1][current.z] == Content::Lava {
                total_count += 1;
            }
        }

        if current.y < volume[0].len() - 1 {
            if volume[current.x][current.y + 1][current.z] == Content::Empty {
                stack.push(Cube { x: current.x, y: current.y + 1, z: current.z });
            } else if volume[current.x][current.y + 1][current.z] == Content::Lava {
                total_count += 1;
            }
        }

        if current.z > 0 {
            if volume[current.x][current.y][current.z - 1] == Content::Empty {
                stack.push(Cube { x: current.x, y: current.y, z: current.z - 1 });
            } else if volume[current.x][current.y][current.z - 1] == Content::Lava {
                total_count += 1;
            }
        }

        if current.z < volume[0][0].len() - 1 {
            if volume[current.x][current.y][current.z + 1] == Content::Empty {
                stack.push(Cube { x: current.x, y: current.y, z: current.z + 1 });
            } else if volume[current.x][current.y][current.z + 1] == Content::Lava {
                total_count += 1;
            }
        }
    }
    
    total_count
}

#[test]
fn solve_b_with_sample_data_returns_58() {
    let sample = indoc::indoc!("
        2,2,2
        1,2,2
        3,2,2
        2,1,2
        2,3,2
        2,2,1
        2,2,3
        2,2,4
        2,2,6
        1,2,5
        3,2,5
        2,1,5
        2,3,5");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(58, actual);
}
