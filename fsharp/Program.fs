open System
open Utils

let arg = Environment.GetCommandLineArgs() |> Seq.skip 1 |> Seq.tryHead
match arg with
| Some "1a" -> Console.In |> Seq.from_reader |> Problem01.solve_a |> printfn "%d"
| Some "1b" -> Console.In |> Seq.from_reader |> Problem01.solve_b |> printfn "%d"
| _ -> printfn "Advent of Code 2022 (https://adventofcode.com)"
       printfn "Run: adventofcode2022 problem"
       printfn "     problem is one of 1a, 1b"
       printfn "The utility reads input from stdin and prints result to stdout."
