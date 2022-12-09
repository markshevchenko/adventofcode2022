module Problem09

type Position = { row: int
                  column: int }


type Direction = Up | Right | Down | Left


let move direction position =
    match direction with
    | Up -> { position with row = position.row + 1 }
    | Right -> { position with column = position.column + 1 }
    | Down -> { position with row = position.row - 1 }
    | Left -> { position with column = position.column - 1 }


let move_tail head tail =
    match tail.row - head.row, tail.column - head.column with
    | 2, 2 -> head |> move Up |> move Right
    | 2, -2 -> head |> move Up |> move Left
    | -2, 2 -> head |> move Down |> move Right
    | -2, -2 -> head |> move Down |> move Left
    | 2, _ -> move Up head
    | -2, _ -> move Down head
    | _, 2 -> move Right head
    | _, -2 -> move Left head
    | _ -> tail
    
    
let parse (line: string) =
    let direction =
        match line[0] with
        | 'U' -> Up
        | 'R' -> Right
        | 'D' -> Down
        | 'L' -> Left
        | c -> failwith $"Invalid direction character %c{c}"
    let n = line.Substring (2) |> int
    (direction, n)


let solve_a (lines: string seq) =
    let mutable rope = Array.replicate 2 { row = 0; column = 0 }
    let mutable tails = Set<_> (seq { yield Array.last rope })
    for line in lines do
        let (direction, n) = parse line
        for _ in 1..n do
            rope[0] <- move direction rope[0]
            for i in 1..rope.Length - 1 do
                rope[i] <- move_tail rope[i - 1] rope[i]
            tails <- tails.Add (Array.last rope)
            
    tails.Count

let solve_b (lines: string seq) =
    let mutable rope = Array.replicate 10 { row = 0; column = 0 }
    let mutable tails = Set<_> (seq { yield Array.last rope })
    for line in lines do
        let (direction, n) = parse line
        for _ in 1..n do
            rope[0] <- move direction rope[0]
            for i in 1..rope.Length - 1 do
                rope[i] <- move_tail rope[i - 1] rope[i]
            tails <- tails.Add (Array.last rope)
            
    tails.Count
