module Problem05

open System.Collections.Generic
open System.Text.RegularExpressions

let solve_a (lines: string seq) =
    let lines = Array.ofSeq lines
    let mutable row = 0
    while lines[row] <> "" do
        row <- row + 1
    
    let stack_count = (lines[0].Length + 1)/4
    let stacks = Seq.replicate stack_count ()
              |> Seq.map (fun () -> Stack<char> ())
              |> Seq.toArray
    for i in (row - 2) .. -1 .. 0 do
        for j in 0 .. (stack_count - 1) do 
            let c = lines[i].[j * 4 + 1]
            if c <> ' ' then stacks[j].Push c
            
    let regex = Regex(@"(\d+)\D+(\d+)\D+(\d+)")
    for i in (row + 1) .. (lines.Length - 1) do
        let m = regex.Match(lines[i])
        let count = int m.Groups[1].Value
        let first = int m.Groups[2].Value
        let second = int m.Groups[3].Value
        
        for k in 1..count do
            stacks[second - 1].Push (stacks[first - 1].Pop ())
        
    stacks |> Array.map (fun stack -> stack.Peek().ToString())
           |> String.concat ""
        

let solve_b (lines: string seq) =
    let lines = Array.ofSeq lines
    let mutable row = 0
    while lines[row] <> "" do
        row <- row + 1
    
    let stack_count = (lines[0].Length + 1)/4
    let stacks = Seq.replicate stack_count ()
              |> Seq.map (fun () -> Stack<char> ())
              |> Seq.toArray
    for i in (row - 2) .. -1 .. 0 do
        for j in 0 .. (stack_count - 1) do 
            let c = lines[i].[j * 4 + 1]
            if c <> ' ' then stacks[j].Push c
            
    let regex = Regex(@"(\d+)\D+(\d+)\D+(\d+)")
    let swap_stack = Stack<char>()
    for i in (row + 1) .. (lines.Length - 1) do
        let m = regex.Match(lines[i])
        let count = int m.Groups[1].Value
        let first = int m.Groups[2].Value
        let second = int m.Groups[3].Value
        
        for k in 1..count do
            swap_stack.Push (stacks[first - 1].Pop ())
        
        for k in 1..count do
            stacks[second - 1].Push (swap_stack.Pop ())
        
    stacks |> Array.map (fun stack -> stack.Peek().ToString())
           |> String.concat ""
