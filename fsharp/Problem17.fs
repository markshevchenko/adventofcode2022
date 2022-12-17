module Problem17

open System
open System.Collections.Generic

type Play (commands: char array) =
    let rocks = [|
        array2D [| "@@@@" |> Array.ofSeq |]
        array2D [| ".@." |> Array.ofSeq
                   "@@@" |> Array.ofSeq
                   ".@." |> Array.ofSeq |]
        array2D ([| "..@" |> Array.ofSeq
                    "..@" |> Array.ofSeq
                    "@@@" |> Array.ofSeq |] |> Array.rev)
        array2D [| "@" |> Array.ofSeq
                   "@" |> Array.ofSeq
                   "@" |> Array.ofSeq
                   "@" |> Array.ofSeq |]
        array2D [| "@@" |> Array.ofSeq
                   "@@" |> Array.ofSeq |] |]
   
    let height = Array2D.length1
    let width = Array2D.length2
   
    let mutable chamber = Array2D.create 10000 7 '.'
   
    let no_overlay rock rock_y rock_x =
        seq {
            for y in 0..height rock - 1 do
                for x in 0..width rock - 1 do
                    yield (y, x)
        } |> Seq.forall (fun (y, x) -> rock[y, x] = '.' || chamber[rock_y + y, rock_x + x] = '.')

    let can_move_left rock rock_y rock_x =
        rock_x > 0 && no_overlay rock rock_y (rock_x - 1)

    let can_move_right rock rock_y rock_x =
        rock_x < (width chamber - width rock) && no_overlay rock rock_y (rock_x + 1)
   
    let can_move_down rock rock_y rock_x =
        rock_y > 0 && no_overlay rock (rock_y - 1) rock_x
   
    let fix rock rock_y rock_x =
        for y in 0..height rock - 1 do
            for x in 0..width rock - 1 do
                if chamber[rock_y + y, rock_x + x] = '.'
                then chamber[rock_y + y, rock_x + x] <- if rock[y, x] = '@' then '#' else '.'

    let mutable tower_height = 0
   
    let mutable next_command = 0
    let mutable next_rock = 0
   
    member _.Chamber = chamber
    member _.Rocks = rocks
    member _.Height = tower_height
    
    member _.TopRow = String.Concat chamber[tower_height - 1, *]
    
    member _.IsRepeat previous_height =
        if previous_height < 16 || previous_height = tower_height
        then false
        else seq {
                for y in 1..16 do
                    for x in 0..width chamber - 1 do
                        yield (y, x)
             } |> Seq.forall (fun (y, x) -> chamber[tower_height - y, x] = chamber[previous_height - y, x])
   
    member _.Round () =
        let rock = rocks[next_rock]
        let mutable y = tower_height + 3
        let mutable x = 2
        let mutable can_fall = true
       
        while can_fall do
            let command = commands[next_command]
            match command with
            | '<' -> if can_move_left rock y x then x <- x - 1 
            | '>' -> if can_move_right rock y x then x <- x + 1
            | _ -> failwith "Invalid command"
          
            next_command <- next_command + 1
            if next_command = commands.Length then next_command <- 0
          
            if can_move_down rock y x
            then y <- y - 1
            else can_fall <- false

        fix rock y x
        tower_height <- max tower_height (y + height rock)
       
        next_rock <- next_rock + 1
        if next_rock = rocks.Length then next_rock <- 0

         
let solve_a (lines: string seq) =
    let commands = lines |> Seq.exactlyOne |> Array.ofSeq 
    let mutable play = Play (commands)
    
    for _ in 1..2022 do
        play.Round ()
       
    int64 play.Height
    
    
let solve_b (lines: string seq) =
    let commands = lines |> Seq.exactlyOne |> Array.ofSeq 
    let mutable play = Play (commands)
    let mutable rounds = Dictionary<string, int> ()
    let mutable heights = List<int> ()
    heights.Add(0)
    
    let mutable round = 0
    let mutable has_done = false
    
    while not has_done do
        play.Round ()
        round <- round + 1
        heights.Add(play.Height)
       
        if rounds.ContainsKey play.TopRow
        then if play.IsRepeat heights[rounds[play.TopRow]]
             then has_done <- true
             else rounds[play.TopRow] <- round
        else
            rounds.Add(play.TopRow, round)
    
    let pred_round = rounds[play.TopRow]
    let period = round - pred_round
    let period_height = heights[round] - heights[pred_round]
    
    let mod_round = int ((1000000000000L - int64 pred_round) % int64 period)
    let repeats = (1000000000000L - int64 pred_round) / int64 period
    
    int64 heights[pred_round + mod_round] + repeats * int64 period_height