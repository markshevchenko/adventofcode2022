open System
open Utils

let arg = Environment.GetCommandLineArgs ()
       |> Seq.skip 1
       |> Seq.tryHead
let input_lines = Console.In |> Seq.from_reader 
match arg with
| Some "1a" -> input_lines |> Problem01.solve_a |> printfn "%d"
| Some "1b" -> input_lines |> Problem01.solve_b |> printfn "%d"
| Some "2a" -> input_lines |> Problem02.solve_a |> printfn "%d"
| Some "2b" -> input_lines |> Problem02.solve_b |> printfn "%d"
| Some "3a" -> input_lines |> Problem03.solve_a |> printfn "%d"
| Some "3b" -> input_lines |> Problem03.solve_b |> printfn "%d"
| Some "4a" -> input_lines |> Problem04.solve_a |> printfn "%d"
| Some "4b" -> input_lines |> Problem04.solve_b |> printfn "%d"
| Some "5a" -> input_lines |> Problem05.solve_a |> printfn "%s"
| Some "5b" -> input_lines |> Problem05.solve_b |> printfn "%s"
| Some "6a" -> input_lines |> Problem06.solve_a |> printfn "%d"
| Some "6b" -> input_lines |> Problem06.solve_b |> printfn "%d"
| Some "7a" -> input_lines |> Problem07.solve_a |> printfn "%d"
| Some "7b" -> input_lines |> Problem07.solve_b |> printfn "%d"
| Some "8a" -> input_lines |> Problem08.solve_a |> printfn "%d"
| Some "8b" -> input_lines |> Problem08.solve_b |> printfn "%d"
| Some "9a" -> input_lines |> Problem09.solve_a |> printfn "%d"
| Some "9b" -> input_lines |> Problem09.solve_b |> printfn "%d"
| Some "10a" -> input_lines |> Problem10.solve_a |> printfn "%d"
| Some "10b" -> input_lines |> Problem10.solve_b |> printfn "%s"
| Some "11a" -> input_lines |> Problem11.solve_a |> printfn "%d"
| Some "11b" -> input_lines |> Problem11.solve_b |> printfn "%d"
| Some "13a" -> input_lines |> Problem13.solve_a |> printfn "%d"
| Some "13b" -> input_lines |> Problem13.solve_b |> printfn "%d"
| Some "15a" -> input_lines |> Problem15.solve_a 2000000 |> printfn "%d"
| Some "15b" -> input_lines |> Problem15.solve_b 4000000 |> printfn "%d"
| _ -> printfn "Advent of Code 2022 (https://adventofcode.com)"
       printfn "Run: adventofcode2022 problem"
       printfn "     problem is one of 1a-11a,13a,15a; 1b-11b,13b,15b"
       printfn "The utility reads input from stdin and prints result to stdout."
