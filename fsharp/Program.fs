open System
open Utils

let arg = Environment.GetCommandLineArgs() |> Seq.skip 1 |> Seq.tryHead
match arg with
| Some "1a" -> Console.In |> Seq.from_reader |> Problem01.solve_a |> printfn "%d"
| Some "1b" -> Console.In |> Seq.from_reader |> Problem01.solve_b |> printfn "%d"
| Some "2a" -> Console.In |> Seq.from_reader |> Problem02.solve_a |> printfn "%d"
| Some "2b" -> Console.In |> Seq.from_reader |> Problem02.solve_b |> printfn "%d"
| Some "3a" -> Console.In |> Seq.from_reader |> Problem03.solve_a |> printfn "%d"
| Some "3b" -> Console.In |> Seq.from_reader |> Problem03.solve_b |> printfn "%d"
| Some "4a" -> Console.In |> Seq.from_reader |> Problem04.solve_a |> printfn "%d"
| Some "4b" -> Console.In |> Seq.from_reader |> Problem04.solve_b |> printfn "%d"
| Some "5a" -> Console.In |> Seq.from_reader |> Problem05.solve_a |> printfn "%s"
| Some "5b" -> Console.In |> Seq.from_reader |> Problem05.solve_b |> printfn "%s"
| _ -> printfn "Advent of Code 2022 (https://adventofcode.com)"
       printfn "Run: adventofcode2022 problem"
       printfn "     problem is one of 1a-4a, 1b-4b"
       printfn "The utility reads input from stdin and prints result to stdout."
