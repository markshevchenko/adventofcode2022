module Problem09

type Position = { row: int
                  column: int }

let up position = { position with row = position.row + 1 }
let right position = { position with column = position.column + 1 }
let down position = { position with row = position.row - 1 }
let left position = { position with column = position.column - 1 }

let are_near position1 position2 =
    abs (position1.row - position2.row) < 2 && abs (position1.column - position2.column) < 2

let solve_a (lines: string seq) =
    0

let solve_b (lines: string seq) =
    0