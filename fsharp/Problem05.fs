module Problem05

open System.Collections.Generic
open System.Diagnostics
open System.Text.RegularExpressions
open Utils

let split_lines lines =
    let parts = lines |> Seq.split "" |> Array.ofSeq
    Debug.Assert (parts.Length = 2)
    
    (parts[0] |> Array.ofSeq, parts[1])


let make_stacks (picture: string array) =
    let stack_count = (picture[0].Length + 1)/4
    let stacks = Seq.replicate stack_count ()
              |> Seq.map (fun () -> Stack<char> ())
              |> Seq.toArray
    for i in (picture.Length - 2) .. -1 .. 0 do
        for j in 0 .. (stack_count - 1) do 
            let c = picture[i][j * 4 + 1]
            if c <> ' ' then stacks[j].Push c
            
    stacks


let regex = Regex(@"^move (\d+) from (\d+) to (\d+)$", RegexOptions.Compiled)
let get_move_parameters move =
    let ``match`` = regex.Match(move)
    let count = int ``match``.Groups[1].Value
    let source = int ``match``.Groups[2].Value
    let ``to`` = int ``match``.Groups[3].Value
    
    (count, source, ``to``)


let solve_a (lines: string seq) =
    let (picture, moves) = split_lines lines
    let stacks = make_stacks picture
    
    for move in moves do
        let (count, from, ``to``) = get_move_parameters move
        for k in 1..count do
            stacks[``to`` - 1].Push (stacks[from - 1].Pop ())
        
    stacks |> Array.map (fun stack -> stack.Peek().ToString())
           |> String.concat ""
        

let solve_b (lines: string seq) =
    let (picture, moves) = split_lines lines
    let stacks = make_stacks picture
    let swap_stack = Stack<char>()
    
    for move in moves do
        let (count, from, ``to``) = get_move_parameters move

        
        for k in 1..count do
            swap_stack.Push (stacks[from - 1].Pop ())
        
        for k in 1..count do
            stacks[``to`` - 1].Push (swap_stack.Pop ())
        
    stacks |> Array.map (fun stack -> stack.Peek().ToString())
           |> String.concat ""
