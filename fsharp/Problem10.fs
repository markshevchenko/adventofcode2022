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
    lines |> Seq.map parse
          |> Seq.collect (function
                          | Noop -> seq { Noop }
                          | AddX n -> seq { Noop; AddX n })
          |> Seq.mapi (fun i op -> (i + 1, op))
          |> Seq.scan (fun (x, accumulator) (cycle, op) ->
                       match op, (cycle - 20) % 40 = 0 with
                       | Noop, false -> (x, accumulator)
                       | Noop, true -> (x, accumulator + cycle * x)
                       | AddX addendum, false ->(x + addendum, accumulator)
                       | AddX addendum, true ->(x + addendum, accumulator + cycle * x))
                      (1, 0)
          |> Seq.last
          |> snd
    
    
let solve_b (lines: string seq) =
    lines |> Seq.map parse
          |> Seq.collect (function
                          | Noop -> seq { Noop }
                          | AddX n -> seq { Noop; AddX n })
          |> Seq.scan (fun (column, x) op ->
                       match op with
                       | Noop -> if column = 39
                                 then (0, x)
                                 else (column + 1, x)
                       | AddX addendum -> if column = 39
                                          then (0, x + addendum)
                                          else (column + 1, x + addendum))
                      (0, 1)
          |> Seq.map (fun (column, x) ->
                          if column >= x - 1 && column <= x + 1
                          then '#'
                          else '.')
          |> Seq.chunkBySize 40
          |> Seq.map String.Concat
          |> Seq.filter (fun line -> line.Length = 40)
          |> String.concat "\n" 
