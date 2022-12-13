use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Vertex {
    row: usize,
    column: usize,
}

impl Vertex {
    fn new(row: usize, column: usize) -> Vertex {
        Vertex { row, column }
    }

    fn distance(&self, other: &Vertex) -> usize {
        self.row.abs_diff(other.row) + self.column.abs_diff(other.column)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Node {
    priority: usize,
    cost: usize,
    vertex: Vertex,
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority.cmp(&other.priority).reverse()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.priority.partial_cmp(&other.priority).map(|cmp| cmp.reverse())
    }
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Debug)]
struct Field {
    height: usize,
    width: usize,
    cells: Vec<Vec<u8>>,
    start: Vertex,
    end: Vertex,
}

impl Field {
    fn load(lines: &mut dyn Iterator<Item=String>) -> Self {
        let mut cells = read_cells(lines);
        let height = cells.len();
        let width= cells[0].len();
        let (start, end) = detect_start_end(&cells);
        cells[start.row][start.column] = b'a';
        cells[end.row][end.column] = b'z';

        Field { height, width, cells, start, end }
    }

    fn is_suitable1(&self, vertex: &Vertex, direction: Direction) -> bool {
        let next = match direction {
            Direction::Up => if vertex.row == 0 { None }
                             else { Some(Vertex::new(vertex.row - 1, vertex.column)) },
            Direction::Down => if vertex.row == self.height - 1 { None }
                               else { Some(Vertex::new(vertex.row + 1, vertex.column)) },
            Direction::Left => if vertex.column == 0 { None }
                               else { Some(Vertex::new(vertex.row, vertex.column - 1)) },
            Direction::Right => if vertex.column == self.width - 1 { None }
                                else { Some(Vertex::new(vertex.row, vertex.column + 1)) },
        };

        if let Some(next) = next {
            let current_value = self.cells[vertex.row][vertex.column];
            let next_value = self.cells[next.row][next.column];

            next_value <= current_value || next_value == current_value + 1
        } else {
            false
        }
    }

    fn neighbors1(&self, coord: Vertex) -> Vec<Vertex> {
        let mut result = Vec::new();
        if self.is_suitable1(&coord, Direction::Up) {
            result.push(Vertex::new(coord.row - 1, coord.column));
        }

        if self.is_suitable1(&coord, Direction::Down) {
            result.push(Vertex::new(coord.row + 1, coord.column));
        }

        if self.is_suitable1(&coord, Direction::Left) {
            result.push(Vertex::new(coord.row, coord.column - 1));
        }

        if self.is_suitable1(&coord, Direction::Right) {
            result.push(Vertex::new(coord.row, coord.column + 1));
        }

        result
    }

    fn find_path(&self) -> usize {
        self.a_star1(self.start, self.end)
    }

    fn a_star1(&self, from: Vertex, to: Vertex) -> usize {
        let mut costs = HashMap::<Vertex, usize>::new();
        let mut heap = BinaryHeap::new();

        costs.insert(from, 0usize);
        heap.push(Node { priority: to.distance(&from), cost: 0, vertex: from });

        while let Some(Node { priority: _, cost, vertex: current }) = heap.pop() {
            if current == to {
                return cost;
            }

            if cost > costs[&current] {
                continue;
            }

            for next in self.neighbors1(current) {
                let next_cost = cost + 1;
                if !costs.contains_key(&next) || next_cost < costs[&next] {
                    costs.insert(next, next_cost);

                    let priority = next_cost + to.distance(&next);
                    heap.push(Node { priority, cost: next_cost, vertex: next});
                }
            }
        }

        panic!("Path has not found")
    }
    
    fn find_nearest_a(&self) -> usize {
        self.a_star2(self.end)
    }

    fn is_suitable2(&self, vertex: &Vertex, direction: Direction) -> bool {
        let next = match direction {
            Direction::Up => if vertex.row == 0 { None }
            else { Some(Vertex::new(vertex.row - 1, vertex.column)) },
            Direction::Down => if vertex.row == self.height - 1 { None }
            else { Some(Vertex::new(vertex.row + 1, vertex.column)) },
            Direction::Left => if vertex.column == 0 { None }
            else { Some(Vertex::new(vertex.row, vertex.column - 1)) },
            Direction::Right => if vertex.column == self.width - 1 { None }
            else { Some(Vertex::new(vertex.row, vertex.column + 1)) },
        };

        if let Some(next) = next {
            let current_value = self.cells[vertex.row][vertex.column];
            let next_value = self.cells[next.row][next.column];

            next_value >= current_value || next_value == current_value - 1
        } else {
            false
        }
    }

    fn neighbors2(&self, coord: Vertex) -> Vec<Vertex> {
        let mut result = Vec::new();
        if self.is_suitable2(&coord, Direction::Up) {
            result.push(Vertex::new(coord.row - 1, coord.column));
        }

        if self.is_suitable2(&coord, Direction::Down) {
            result.push(Vertex::new(coord.row + 1, coord.column));
        }

        if self.is_suitable2(&coord, Direction::Left) {
            result.push(Vertex::new(coord.row, coord.column - 1));
        }

        if self.is_suitable2(&coord, Direction::Right) {
            result.push(Vertex::new(coord.row, coord.column + 1));
        }

        result
    }

    fn a_star2(&self, from: Vertex) -> usize {
        let mut costs = HashMap::<Vertex, usize>::new();
        let mut heap = BinaryHeap::new();

        costs.insert(from, 0usize);
        heap.push(Node { priority: 0, cost: 0, vertex: from });

        while let Some(Node { priority: _, cost, vertex: current }) = heap.pop() {
            if self.cells[current.row][current.column] == b'a' {
                return cost;
            }

            if cost > costs[&current] {
                continue;
            }

            for next in self.neighbors2(current) {
                let next_cost = cost + 1;
                if !costs.contains_key(&next) || next_cost < costs[&next] {
                    costs.insert(next, next_cost);

                    heap.push(Node { priority: next_cost, cost: next_cost, vertex: next});
                }
            }
        }

        panic!("Path has not found")
    }
}

fn read_cells(lines: &mut dyn Iterator<Item=String>) -> Vec<Vec<u8>> {
    lines.map(|line| line.bytes().collect::<Vec<_>>())
         .collect()
}

fn detect_start_end(cells: &Vec<Vec<u8>>) -> (Vertex, Vertex) {
    let mut start = None;
    let mut end = None;

    for row in 0..cells.len() {
        for column in 0..cells[row].len() {
            if let Some(start) = start {
                if let Some(end) = end {
                    return (start, end);
                }
            }

            if cells[row][column] == b'S' {
                start = Some(Vertex { row, column });
            } else if cells[row][column] == b'E' {
                end = Some(Vertex { row, column });
            }
        }
    }

    panic!("No start or end have found")
}

pub fn solve_a(lines: &mut dyn Iterator<Item=String>) -> usize {
    let field = Field::load(lines);

    field.find_path()
}

#[test]
fn solve_a_with_sample_data_returns_31() {
    let sample = indoc::indoc!("
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_a(&mut lines);

    assert_eq!(31, actual);
}

pub fn solve_b(lines: &mut dyn Iterator<Item=String>) -> usize {
    let field = Field::load(lines);

    field.find_nearest_a()
}

#[test]
fn solve_b_with_sample_data_returns_29() {
    let sample = indoc::indoc!("
        Sabqponm
        abcryxxl
        accszExk
        acctuvwj
        abdefghi");
    let mut lines = crate::utils::str_to_iter(sample);

    let actual = solve_b(&mut lines);

    assert_eq!(29, actual);
}
