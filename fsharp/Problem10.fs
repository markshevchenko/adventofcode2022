module Problem10

open System

type Op =
    | Noop
    | AddX of int
    
let cycles = function
    | Noop -> 1
    | AddX _ -> 2

let parse line =
    if line = "noop" then Noop
    else if line.StartsWith "addx" then AddX (line.Substring 5 |> int)
    else failwith "Unrecognized operation"

let solve_a (lines: string seq) =
    let mutable cycle = 1
    let mutable x = 1
    let mutable accumulator = 0
    for line in lines do
        let op = parse line
        
        for _ in 1..cycles op do
            if (cycle - 20) % 40 = 0 then
                accumulator <- accumulator + cycle * x
                
            cycle <- cycle + 1
            
        match op with
        | Noop -> ()
        | AddX addendum -> x <- x + addendum
    
    accumulator
    
    
let solve_b (lines: string seq) =
    let mutable crt = Array.init 6 (fun _ -> Array.replicate 40 ' ')
    let mutable row = 0
    let mutable column = 0
    let mutable x = 1
    for line in lines do
        let op = parse line
        
        for _ in 1..cycles op do
            if column >= x - 1 && column <= x + 1
            then crt[row][column] <- '#'
            else crt[row][column] <- '.'
            
            column <- column + 1
            if column = 40 then
                column <- 0
                row <- row + 1
            
        match op with
        | Noop -> ()
        | AddX addendum -> x <- x + addendum
    
    crt |> Seq.map String.Concat
        |> String.concat "\n"
