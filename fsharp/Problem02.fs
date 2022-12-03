module Problem02

type Shape = Rock | Paper | Scissors

type Outcome = Win | Draw | Defeat

let opponent_choice = function
    | 'A' -> Rock
    | 'B' -> Paper
    | 'C' -> Scissors
    | _ -> failwith "Invalid letter for opponent's choice"


let your_choice = function
    | 'X' -> Rock
    | 'Y' -> Paper
    | 'Z' -> Scissors
    | _ -> failwith "Invalid letter for yours choice"
    

let player2_outcome = function
    | (Rock, Rock) -> Draw
    | (Rock, Paper) -> Win
    | (Rock, Scissors) -> Defeat
    | (Paper, Rock) -> Defeat
    | (Paper, Paper) -> Draw
    | (Paper, Scissors) -> Win
    | (Scissors, Rock) -> Win
    | (Scissors, Paper) -> Defeat
    | (Scissors, Scissors) -> Draw


let shape_score = function
    | Rock -> 1
    | Paper -> 2
    | Scissors -> 3
    
    
let outcome_score = function
    | Defeat -> 0
    | Draw -> 3
    | Win -> 6


let solve_a (lines: string seq) =
    lines |> Seq.map (fun line -> (opponent_choice line.[0], your_choice line.[2]))
          |> Seq.map (fun (opponent, your) -> (player2_outcome (opponent, your), your))
          |> Seq.map (fun (outcome, shape) -> outcome_score outcome + shape_score shape)
          |> Seq.sum


let your_outcome = function
    | 'X' -> Defeat
    | 'Y' -> Draw
    | 'Z' -> Win
    | _ -> failwith "Invalid letter for yours outcome"


let player2_shape = function
    | (Rock, Defeat) -> Scissors
    | (Rock, Draw) -> Rock
    | (Rock, Win) -> Paper
    | (Paper, Defeat) -> Rock
    | (Paper, Draw) -> Paper
    | (Paper, Win) -> Scissors
    | (Scissors, Defeat) -> Paper
    | (Scissors, Draw) -> Scissors
    | (Scissors, Win) -> Rock


let solve_b (lines: string seq) =
    lines |> Seq.map (fun line -> (opponent_choice line.[0], your_outcome line.[2]))
          |> Seq.map (fun (opponent, outcome) -> (outcome, player2_shape (opponent, outcome)))
          |> Seq.map (fun (outcome, shape) -> outcome_score outcome + shape_score shape)
          |> Seq.sum
