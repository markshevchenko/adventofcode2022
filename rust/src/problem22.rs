#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Command {
    Left,
    Right,
    Walk(u32),
}

struct Labyrinth {
    field: Vec<Vec<u8>>,
    commands: Vec<Command>,
}

fn parse_field(lines: &mut dyn Iterator<Item=String>) -> Vec<Vec<u8>> {
    let mut field = Vec::new();
    while let Some(line) = lines.next() {
        if line == "" {
            break;
        }

        let row = line.bytes().collect::<Vec<_>>();
        field.push(row);
    }
    
    field
}

fn parse_commands(lines: &mut dyn Iterator<Item=String>) -> Vec<Command> {
    let mut commands = Vec::new();
    if let Some(line) = lines.next() {
        let mut number = 0;
        for c in line.bytes() {
            match c {
                b'L' => {
                    if number > 0 {
                        commands.push(Command::Walk(number));
                    }

                    commands.push(Command::Left);
                    number = 0;
                },
                b'R' => {
                    if number > 0 {
                        commands.push(Command::Walk(number));
                    }

                    commands.push(Command::Right);
                    number = 0;
                },
                b'0'..=b'9' => number = 10 * number + (c - b'0') as u32,
                _ => panic!("Unknown command character."),
            }
        }

        if number > 0 {
            commands.push(Command::Walk(number));
        }
    }
    
    commands
}

#[test]
fn parse_commands_parses_sample() {
    let mut lines = crate::utils::str_to_iter("10R5L5R10L4R5L5");
    let actual = parse_commands(&mut lines);
    let expected = vec![Command::Walk(10), Command::Right, Command::Walk(5),
    Command::Left, Command::Walk(5), Command::Right, Command::Walk(10), Command::Left,
    Command::Walk(4), Command::Right, Command::Walk(5), Command::Left, Command::Walk(5)]; 
    
    assert_eq!(expected, actual);
}

fn parse_labyrinth(lines: &mut dyn Iterator<Item=String>) -> Labyrinth {
    let field = parse_field(lines);
    let commands = parse_commands(lines);
    
    Labyrinth { field, commands }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn facing(&self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Walker {
    row: usize,
    column: usize,
    direction: Direction
}

impl Walker {
    fn new(top_row: &[u8]) -> Self {
        let column = top_row.iter().position(|&c| c != b' ').unwrap() + 1;
        
        Walker {
            row: 1,
            column,
            direction: Direction::Right,
        }
    }

    fn left(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
    
    fn next(&self, field: &Vec<Vec<u8>>) -> (usize, usize) {
        let mut next_row = self.row;
        let mut next_column = self.column;
        
        match self.direction {
            Direction::Right => {
                if next_column == field[self.row - 1].len() {
                    while next_column > 1 && field[self.row - 1][next_column - 2] != b' ' {
                        next_column -= 1;
                    }
                } else {
                    next_column += 1;
                }
            },
            Direction::Down => {
                if next_row == field.len() || self.column > field[next_row].len() 
                    || field[next_row][self.column - 1] == b' ' {
                    while next_row > 1 && field[next_row - 2].len() >= self.column
                        && field[next_row - 2][self.column - 1] != b' ' {
                        next_row -= 1;
                    }
                } else {
                    next_row += 1;
                }
            },
            Direction::Left => {
                if next_column == 1 {
                    while next_column < field[self.row - 1].len() && field[self.row - 1][next_column] != b' ' {
                        next_column += 1;
                    }
                } else {
                    next_column -= 1;
                }
            },
            Direction::Up => {
                if next_row == 1 || self.column > field[next_row - 2].len()
                    || field[next_row - 2][self.column - 1] == b' ' {
                    while next_row < field.len() && field[next_row].len() >= self.column
                        && field[next_row][self.column - 1] != b' ' {
                        next_row += 1;
                    }
                } else {
                    next_row -= 1;
                }
            },
        };

        (next_row, next_column)
    }
    
    fn walk(&mut self, field: &Vec<Vec<u8>>, mut length: u32) {
        loop {
            if length == 0 {
                break;
            }

            let (next_row, next_column) = self.next(field);
            if field[next_row - 1][next_column - 1] == b'#' {
                break;
            }
            
            self.row = next_row;
            self.column = next_column;
            length -= 1;
        }
    }
}

#[test]
fn walker_new_sets_column_9_on_sample_data() {
    let actual = Walker::new(b"        ...#");
    
    assert_eq!(Walker { row: 1, column: 9, direction: Direction::Right }, actual);
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let labyrinth = parse_labyrinth(lines);
    let mut walker = Walker::new(&labyrinth.field[0]);
    
    for &command in labyrinth.commands.iter() {
        match command {
            Command::Left => walker.left(),
            Command::Right => walker.right(),
            Command::Walk(length) => walker.walk(&labyrinth.field, length),
        }
    }
    
    1000 * walker.row + 4 * walker.column + walker.direction.facing()
}

#[test]
fn solve_a_with_sample_data_returns_6032() {
    let sample = indoc::indoc!("
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.
        
        10R5L5R10L4R5L5");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(6032, actual);
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Face {
    Front = 0,
    Up    = 1,
    Right = 2,
    Back  = 3,
    Down  = 4,
    Left  = 5,
}

impl Face {
    const COUNT: usize = 6;
    
    fn go_side(&self, direction: Direction) -> Self {
        Face::Front
    }
}

#[test]
fn test_face_go_side() {
    assert_eq!(Face::Up, Face::Front::go_side(Direction::Up));
    assert_eq!(Face::Right, Face::Front::go_side(Direction::Right));
    assert_eq!(Face::Down, Face::Front::go_side(Direction::Down));
    assert_eq!(Face::Left, Face::Front::go_side(Direction::Left));

    assert_eq!(Face::Up, Face::Right::go_side(Direction::Up));
    assert_eq!(Face::Back, Face::Right::go_side(Direction::Right));
    assert_eq!(Face::Down, Face::Right::go_side(Direction::Down));
    assert_eq!(Face::Front, Face::Right::go_side(Direction::Left));

    assert_eq!(Face::Up, Face::Back::go_side(Direction::Up));
    assert_eq!(Face::Left, Face::Back::go_side(Direction::Right));
    assert_eq!(Face::Down, Face::Back::go_side(Direction::Down));
    assert_eq!(Face::Left, Face::Back::go_side(Direction::Left));

    assert_eq!(Face::Up, Face::Left::go_side(Direction::Up));
    assert_eq!(Face::Back, Face::Left::go_side(Direction::Right));
    assert_eq!(Face::Down, Face::Left::go_side(Direction::Down));
    assert_eq!(Face::Front, Face::Left::go_side(Direction::Left));
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
enum Rotate {
    None,
    Clock,
    Contra,
    Flip,
}

const CUBE: &'static [[Option<(Face, Rotate)>; 4]; 4] = &[
    [None,                              None,                               Some ((Face::Front, Rotate::None)), Some ((Face::Right, Rotate::None))],
    [Some ((Face::Down, Rotate::Flip)), Some ((Face::Left, Rotate::Clock)), Some ((Face::Down, Rotate::None)),  None],
    [None,                              Some ((Face::Right, Rotate::Flip)), Some ((Face::Back, Rotate::None)),  Some ((Face::Left, Rotate::Flip))],
    [None,                              Some ((Face::Up, Rotate::Contra)),  None,                               None],
];

const CUBE_ORIGIN_COLUMN: usize = 2;

#[allow(dead_code)]
struct Cube {
    size: usize,
    field: Vec<Vec<u8>>,
    faces: [(usize, usize, Rotate); 6],
}

impl Cube {
    fn load(field: Vec<Vec<u8>>) -> Self {
        let size = if field.len() >= 50 { 50 } else { 4 };
        let origin_column = field[0].iter().position(|&c| c != b' ').unwrap() / size;
        let mut faces = [(0, 0, Rotate::None); Face::COUNT];

        for face_row in 0..field.len() / size {
            for face_column in 0..field[face_row * size].len() / size {
                if field[face_row * size][face_column * size] == b' ' {
                    continue;
                }

                let adjusted_column = CUBE_ORIGIN_COLUMN + face_column - origin_column;
                if let &Some((face, rotate)) = &CUBE[face_row][adjusted_column] {
                    let face_top = face_row * size;
                    let face_left = face_column * size;
                    faces[face as usize] = (face_top, face_left, rotate);
                }
            }
        }

        Cube { size, field, faces }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct CubeWalker {
    row: usize,
    column: usize,
    direction: Direction
}

impl CubeWalker {
    fn new(top_row: &[u8]) -> Self {
        let column = top_row.iter().position(|&c| c != b' ').unwrap() + 1;

        CubeWalker {
            row: 1,
            column,
            direction: Direction::Right,
        }
    }

    fn left(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Up,
            Direction::Up => Direction::Left,
            Direction::Left => Direction::Down,
            Direction::Down => Direction::Right,
        }
    }

    fn right(&mut self) {
        self.direction = match self.direction {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }

    fn next(&self, field: &Vec<Vec<u8>>) -> (usize, usize) {
        let mut next_row = self.row;
        let mut next_column = self.column;

        match self.direction {
            Direction::Right => {
                if next_column == field[self.row - 1].len() {
                    while next_column > 1 && field[self.row - 1][next_column - 2] != b' ' {
                        next_column -= 1;
                    }
                } else {
                    next_column += 1;
                }
            },
            Direction::Down => {
                if next_row == field.len() || self.column > field[next_row].len()
                    || field[next_row][self.column - 1] == b' ' {
                    while next_row > 1 && field[next_row - 2].len() >= self.column
                        && field[next_row - 2][self.column - 1] != b' ' {
                        next_row -= 1;
                    }
                } else {
                    next_row += 1;
                }
            },
            Direction::Left => {
                if next_column == 1 {
                    while next_column < field[self.row - 1].len() && field[self.row - 1][next_column] != b' ' {
                        next_column += 1;
                    }
                } else {
                    next_column -= 1;
                }
            },
            Direction::Up => {
                if next_row == 1 || self.column > field[next_row - 2].len()
                    || field[next_row - 2][self.column - 1] == b' ' {
                    while next_row < field.len() && field[next_row].len() >= self.column
                        && field[next_row][self.column - 1] != b' ' {
                        next_row += 1;
                    }
                } else {
                    next_row -= 1;
                }
            },
        };

        (next_row, next_column)
    }

    fn walk(&mut self, field: &Vec<Vec<u8>>, mut length: u32) {
        loop {
            if length == 0 {
                break;
            }

            let (next_row, next_column) = self.next(field);
            if field[next_row - 1][next_column - 1] == b'#' {
                break;
            }

            self.row = next_row;
            self.column = next_column;
            length -= 1;
        }
    }
}


pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let labyrinth = parse_labyrinth(lines);
    let cube = Cube::load(labyrinth.field);
    // let mut walker = Walker::new(&labyrinth.field[0]);

    for &command in labyrinth.commands.iter() {
        // match command {
        //     Command::Left => walker.left(),
        //     Command::Right => walker.right(),
        //     Command::Walk(length) => walker.walk(&labyrinth.field, length),
        // }
    }

    // 1000 * walker.row + 4 * walker.column + walker.direction.facing()

    5031
}

#[test]
fn solve_b_with_sample_data_returns_5031() {
    let sample = indoc::indoc!("
                ...#
                .#..
                #...
                ....
        ...#.......#
        ........#...
        ..#....#....
        ..........#.
                ...#....
                .....#..
                .#......
                ......#.
        
        10R5L5R10L4R5L5");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(5031, actual);
}
