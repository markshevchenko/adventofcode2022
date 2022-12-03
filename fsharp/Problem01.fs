module Problem01

open System
open Utils

let solve_a lines =
    lines |> Seq.split ""
          |> Seq.map (Seq.map (UInt32.TryParse >> snd))
          |> Seq.map Seq.sum
          |> Seq.max


let solve_b lines =
    lines |> Seq.split ""
          |> Seq.map (Seq.map (UInt32.TryParse >> snd))
          |> Seq.map Seq.sum
          |> Seq.sortDescending
          |> Seq.take 3
          |> Seq.sum