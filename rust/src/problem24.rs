use std::collections::{HashSet};
use std::hash::{Hash, Hasher};

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Direction {
    Fore,
    Back
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
struct Blizzard {
    position: usize,
    direction: Direction,
}

impl Blizzard {
    fn new(position: usize, direction: Direction) -> Self {
        Blizzard { position, direction }
    }
    
    fn shift(&self, offset: usize, size: usize) -> usize {
        if self.direction == Direction::Fore {
            (self.position + offset) % size
        } else if self.position >= offset {
            self.position - offset
        } else {
            size - 1 - ((offset - self.position - 1) % size)
        }
    }
}

#[test]
fn test_blizzard_next_position() {
    let fore = Blizzard::new(3, Direction::Fore);
    let back = Blizzard::new(3, Direction::Back);

    assert_eq!(3, fore.shift(0, 5));
    assert_eq!(3, back.shift(0, 5));
    
    assert_eq!(4, fore.shift(1, 5));
    assert_eq!(2, back.shift(1, 5));
    
    assert_eq!(0, fore.shift(2, 5));
    assert_eq!(1, back.shift(2, 5));
    
    assert_eq!(1, fore.shift(3, 5));
    assert_eq!(0, back.shift(3, 5));
    
    assert_eq!(2, fore.shift(4, 5));
    assert_eq!(4, back.shift(4, 5));
    
    assert_eq!(3, fore.shift(5, 5));
    assert_eq!(3, back.shift(5, 5));
    
    assert_eq!(4, fore.shift(6, 5));
    assert_eq!(2, back.shift(6, 5));

    assert_eq!(0, fore.shift(7, 5));
    assert_eq!(1, back.shift(7, 5));

    assert_eq!(1, fore.shift(8, 5));
    assert_eq!(0, back.shift(8, 5));

    assert_eq!(2, fore.shift(9, 5));
    assert_eq!(4, back.shift(9, 5));

    assert_eq!(3, fore.shift(10, 5));
    assert_eq!(3, back.shift(10, 5));

    assert_eq!(4, fore.shift(11, 5));
    assert_eq!(2, back.shift(11, 5));
}

struct Map {
    width: usize,
    height: usize,
    blizzards_by_x: Vec<Vec<Blizzard>>,
    blizzards_by_y: Vec<Vec<Blizzard>>,
    start_x: usize,
    start_y: usize,
    end_x: usize,
    end_y: usize,
}

impl Map {
    fn load(lines: &mut dyn Iterator<Item=String>) -> Self {
        let chars = lines.map(|line| line.bytes().collect::<Vec<_>>())
            .collect::<Vec<_>>();

        let width = chars[0].len() - 2;
        let height = chars.len() - 2;

        let mut blizzards_by_x = Vec::new();
        for _ in 0..width {
            blizzards_by_x.push(Vec::new());
        }

        let mut blizzards_by_y = Vec::new();
        for _ in 0..height {
            blizzards_by_y.push(Vec::new());
        }
        
        for y in 0..height {
            for x in 0..width {
                match chars[y + 1][x + 1] {
                    b'>' => blizzards_by_y[y].push(Blizzard::new(x, Direction::Fore)),
                    b'v' => blizzards_by_x[x].push(Blizzard::new(y, Direction::Fore)),
                    b'<' => blizzards_by_y[y].push(Blizzard::new(x, Direction::Back)),
                    b'^' => blizzards_by_x[x].push(Blizzard::new(y, Direction::Back)),
                    _ => (),
                }
            }
        }

        let start_x = chars[0].iter().position(|&b| b == b'.').unwrap() - 1;
        let start_y = usize::MAX;
        let end_x = chars[chars.len() - 1].iter().position(|&b| b == b'.').unwrap() - 1;
        let end_y = height;
        
        Map { width, height, blizzards_by_x, blizzards_by_y, start_x, start_y, end_x, end_y }
    }
    
    fn out_of_blizzards(&self, frame: &Frame) -> bool {
        if frame.y == usize::MAX || frame.y == self.height {
            return true;
        }
        
        for blizzard in &self.blizzards_by_y[frame.y] {
            if blizzard.shift(frame.minute, self.width) == frame.x {
                return false;
            }
        }

        for blizzard in &self.blizzards_by_x[frame.x] {
            if blizzard.shift(frame.minute, self.height) == frame.y {
                return false;
            }
        }
        
        true
    }
}

#[test]
fn test_map_load_with_sample() {
    let sample = indoc::indoc!("
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#");
    let mut lines = crate::utils::str_to_iter(sample);
    let map = Map::load(&mut lines);
    
    assert_eq!(5, map.width);
    assert_eq!(5, map.height);
    assert_eq!(0, map.start_x);
    assert_eq!(4, map.end_x);
    assert_eq!(vec![Blizzard { position: 3, direction: Direction::Fore }], map.blizzards_by_x[3]);
    assert_eq!(vec![Blizzard { position: 0, direction: Direction::Fore }], map.blizzards_by_y[1]);
}

#[derive(Clone, PartialEq, Eq, Debug)]
struct Frame {
    x: usize,
    y: usize,
    minute: usize,
}

impl Frame {
    fn new(x: usize, y: usize, minute: usize) -> Self {
        Frame { x, y, minute }
    }

    fn distance_from(&self, x: usize, y: usize) -> usize {
        let x_diff = if x >= self.x { x - self.x} else { self.x - x };
        let y_diff = if y == usize::MAX {
            if self.y == usize::MAX { 0 } else { self.y + 1 }
        } else if y >= self.y {
            y - self.y
        } else {
            if self.y == usize::MAX { y + 1 } else { self.y - y }
        };
        
        x_diff + y_diff
    }

    fn try_left(&self, map: &Map) -> Option<Self> {
        if self.x == map.start_x && self.y == map.start_y
            || self.x == map.end_x && self.y == map.end_y
            || self.x == 0 {
            None
        } else {
            Some(Frame { x: self.x - 1, minute: self.minute + 1, ..*self })
        }
    }
    
    fn try_right(&self, map: &Map) -> Option<Self> {
        if self.x == map.start_x && self.y == map.start_y
            || self.x == map.end_x && self.y == map.end_y
            || self.x == map.width - 1 {
            None
        } else  {
            Some(Frame { x: self.x + 1, minute: self.minute + 1, ..*self })
        }
    }

    fn try_up(&self, map: &Map) -> Option<Self> {
        if self.y == 0 {
            if self.x == map.start_x {
                Some(Frame { y: map.start_y, minute: self.minute + 1, ..*self })
            } else {
                None
            }
        } else if self.y == map.start_y {
            None
        } else {
            Some(Frame { y: self.y - 1, minute: self.minute + 1, ..*self })
        }
    }
    
    fn try_down(&self, map: &Map) -> Option<Self> {
        if self.y == map.height - 1 {
            if self.x == map.end_x {
                Some(Frame { y: map.end_y, minute: self.minute + 1, ..*self })
            } else {
                None
            }
        } else if self.y == map.end_y {
            None
        } else if self.y == map.start_y {
            Some(Frame { y: 0, minute: self.minute + 1, ..*self })
        } else {
            Some(Frame { y: self.y + 1, minute: self.minute + 1, ..*self })
        }
    }
    
    fn after(&self) -> Self {
        Frame { minute: self.minute + 1, ..*self }
    }
}

impl Hash for Frame {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.x.hash(state);
        self.y.hash(state);
        self.minute.hash(state);
    }
}

#[test]
fn test_frame_trys_with_sample() {
    let sample = indoc::indoc!("
        #.#####
        #.....#
        #>....#
        #.....#
        #...v.#
        #.....#
        #####.#");
    let mut lines = crate::utils::str_to_iter(sample);
    let map = Map::load(&mut lines);
    let frame1 = Frame::new(map.start_x, map.start_y, 0);
    assert_eq!(Frame { x: 0, y: usize::MAX, minute: 0 }, frame1);
    assert_eq!(None, frame1.try_left(&map));
    assert_eq!(Some(Frame { x: 0, y: 0, minute: 1 }), frame1.try_down(&map));
    assert_eq!(None, frame1.try_right(&map));
    assert_eq!(None, frame1.try_up(&map));
    
    let left_frame = Frame { x: 0, y: 2, minute: 0 };
    assert_eq!(None, left_frame.try_left(&map));
    assert_eq!(Some(Frame { x: 0, y: 3, minute: 1 }), left_frame.try_down(&map));
    assert_eq!(Some(Frame { x: 1, y: 2, minute: 1 }), left_frame.try_right(&map));
    assert_eq!(Some(Frame { x: 0, y: 1, minute: 1 }), left_frame.try_up(&map));
    
    let down_frame = Frame { x: 3, y: map.height - 1, minute: 0};
    assert_eq!(Some(Frame { x: 2, y: 4, minute: 1 }), down_frame.try_left(&map));
    assert_eq!(None, down_frame.try_down(&map));
    assert_eq!(Some(Frame { x: 4, y: 4, minute: 1 }), down_frame.try_right(&map));
    assert_eq!(Some(Frame { x: 3, y: 3, minute: 1 }), down_frame.try_up(&map));
    
    let above_end = Frame { x: 4, y: map.height - 1, minute: 0};
    assert_eq!(Some(Frame { x: 4, y: 5, minute: 1 }), above_end.try_down(&map));
}

fn enqueue_if_out_of_blizzards(map: &Map, frame: Frame, queue: &mut HashSet<Frame>) {
    if map.out_of_blizzards(&frame) {
        queue.insert(frame);
    }
}

fn accelerated_bfs(map: &Map, start_x: usize, start_y: usize, end_x: usize, end_y: usize, minute: usize) -> usize {
    let mut queue = vec![Frame::new(start_x, start_y, minute)];
    
    loop {
        let mut new_queue = HashSet::new();
        
        for frame in queue {
            if frame.x == end_x && frame.y == end_y {
                return frame.minute;
            }

            if let Some(next_frame) = frame.try_left(&map) {
                enqueue_if_out_of_blizzards(&map, next_frame, &mut new_queue);
            }

            if let Some(next_frame) = frame.try_down(&map) {
                enqueue_if_out_of_blizzards(&map, next_frame, &mut new_queue);
            }

            if let Some(next_frame) = frame.try_right(&map) {
                enqueue_if_out_of_blizzards(&map, next_frame, &mut new_queue);
            }

            if let Some(next_frame) = frame.try_up(&map) {
                enqueue_if_out_of_blizzards(&map, next_frame, &mut new_queue);
            }

            enqueue_if_out_of_blizzards(&map, frame.after(), &mut new_queue);
        }
        
        if new_queue.is_empty() {
            panic!("Path not found.")
        }
        
        let mut new_queue = new_queue.into_iter().collect::<Vec<_>>();
        new_queue.sort_by(|frame1, frame2| {
            let distance1 = frame1.distance_from(end_x, end_y);
            let distance2 = frame2.distance_from(end_x, end_y);
            
            distance1.cmp(&distance2)
        });
        
        new_queue.truncate(300);
        
        queue = new_queue;
    }
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let map = Map::load(lines);
    
    accelerated_bfs(&map, map.start_x, map.start_y, map.end_x, map.end_y, 0)
}

#[test]
fn solve_a_with_sample_data_returns_18() {
    let sample = indoc::indoc!("
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(18, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let map = Map::load(lines);

    let minute1 = accelerated_bfs(&map, map.start_x, map.start_y, map.end_x, map.end_y, 0);
    let minute2 = accelerated_bfs(&map, map.end_x, map.end_y, map.start_x, map.start_y, minute1);
    
    accelerated_bfs(&map, map.start_x, map.start_y, map.end_x, map.end_y, minute2)
}

#[test]
fn solve_b_with_sample_data_returns_54() {
    let sample = indoc::indoc!("
        #.######
        #>>.<^<#
        #.<..<<#
        #>v.><>#
        #<^v^^>#
        ######.#");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(54, actual);
}
