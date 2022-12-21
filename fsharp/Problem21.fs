module Problem21

open System
open System.Collections.Generic

type Name = Name of string
type Operation =
    | Add
    | Sub
    | Mul
    | Div
    
type Operand = Left | Right

type Expression =
    | Number of int64
    | Operation of Operation * Name * Name
    
type Reactive =
    | Result of int64
    | Operations of (Operation * Operand * int64) list
    
let parse (line: string) =
    let name = Name line[0..3]
    if Char.IsDigit line[6]
    then (name, Number (int64 line[6..]))
    else let left = Name line[6..9]
         let right = Name line[13..16]
         let operation = match line[11] with
                         | '+' -> Add
                         | '-' -> Sub
                         | '*' -> Mul
                         | '/' -> Div
                         | _ -> failwith $"Unknown operation {line[11]}"
         (name, Operation (operation, left, right))
         
let rec calculate (steps: Dictionary<Name, Expression>) (name: Name) =
    match steps[name] with
    | Number number -> number
    | Operation (operation, left, right) ->
        let left_value = calculate steps left
        let right_value = calculate steps right
        match operation with
        | Add -> left_value + right_value
        | Sub -> left_value - right_value
        | Mul -> left_value * right_value
        | Div -> left_value / right_value
        
let solve_a (lines: string seq) =
    let mutable steps = Dictionary<Name, Expression> ()
    for line in lines do
        let name, expression = parse line
        steps.Add (name, expression)
        
    calculate steps (Name "root")
    
let rec reactive_build (steps: Dictionary<Name, Expression>) (name: Name) =
    if name = Name "humn"
    then Operations []
    else match steps[name] with
         | Number number -> Result number
         | Operation (operation, left, right) ->
            let left_value = reactive_build steps left
            let right_value = reactive_build steps right
            match (operation, left_value, right_value) with
            | Add, Result a, Result b -> Result (a + b)
            | Sub, Result a, Result b -> Result (a - b)
            | Mul, Result a, Result b -> Result (a * b)
            | Div, Result a, Result b -> Result (a / b)
            | operation, Result a, Operations b -> Operations ((operation, Left, a)::b)
            | operation, Operations a, Result b -> Operations ((operation, Right, b)::a)
            | _ -> failwith "Too complicated to solve"
            
let rec reactive_calculate (expect: int64) (operations: (Operation * Operand * int64) list) =            
    match operations with
    | [] -> expect
    | (Add, _, value)::tail_operations -> reactive_calculate (expect - value) tail_operations
    | (Sub, Left, value)::tail_operations -> reactive_calculate (value - expect) tail_operations
    | (Sub, Right, value)::tail_operations -> reactive_calculate (expect + value) tail_operations
    | (Mul, _, value)::tail_operations -> reactive_calculate (expect / value) tail_operations
    | (Div, Left, value)::tail_operations -> reactive_calculate (value / expect) tail_operations
    | (Div, Right, value)::tail_operations -> reactive_calculate (expect * value) tail_operations
            

let solve_b (lines: string seq) =
    let mutable steps = Dictionary<Name, Expression> ()
    for line in lines do
        let name, expression = parse line
        steps.Add (name, expression)

    match reactive_build steps (Name "root") with
    | Operations ((_, _, expect)::operations) -> reactive_calculate expect operations
    | _ -> failwith "Invalid reactive expression"
