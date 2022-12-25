module Problem25

open System

let from_snafu_digit = function
    | '=' -> -2L
    | '-' -> -1L
    | '0' -> 0L
    | '1' -> 1L
    | '2' -> 2L
    | _ -> failwith "Unknown SNAFU digit"


let from_snafu (chars: char seq) =
    chars |> Seq.map from_snafu_digit
          |> Seq.fold (fun a d -> 5L * a + d) 0L
          
          
let to_snafu_digit = function
    | -2L -> '='
    | -1L -> '-'
    | 0L -> '0'
    | 1L -> '1'
    | 2L -> '2'
    | _ -> failwith "Invalid remainder"
          
          
let to_snafu (number: int64) =
    number |> Seq.unfold (fun number -> if number = 0
                                        then None
                                        else let remainder = (number + 2L) % 5L - 2L in
                                             Some (to_snafu_digit remainder, (number - remainder) / 5L))
           |> Seq.rev
           |> String.Concat 


let solve_a (lines: string seq) =
    lines |> Seq.map from_snafu
          |> Seq.sum
          |> to_snafu
