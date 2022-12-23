module Problem23

open FSharp.Collections

type Direction =
    | North
    | East
    | South
    | West

type Point =
    { y: int
      x: int }
    
let go direction { y = y; x = x } =
    match direction with
    | North -> { y = y - 1; x = x }
    | East -> { y = y; x = x + 1 }
    | South -> { y = y + 1; x = x }
    | West -> { y = y; x = x - 1 }
    
let around point =
    seq {
        point |> go North |> go West
        point |> go North
        point |> go North |> go East
        point |> go East
        point |> go South |> go East
        point |> go South
        point |> go South |> go West
        point |> go West
    } |> Set.ofSeq
    
let from direction point =
    match direction with
    | North -> seq {
        point |> go North |> go West
        point |> go North
        point |> go North |> go East }
    | South -> seq {
        point |> go South |> go West
        point |> go South
        point |> go South |> go East }
    | East -> seq {
        point |> go East |> go North
        point |> go East
        point |> go East |> go South }
    | West -> seq {
        point |> go West |> go North
        point |> go West
        point |> go West |> go South }
    |> Set.ofSeq
    
let is_empty field area = Set.intersect field area |> Set.isEmpty
    
let can_move direction point =
    from direction point |> is_empty
                         
let try_move field (direction1, direction2, direction3, direction4) point =
    if is_empty field (around point)
    then None
    else if can_move direction1 point field
    then Some (point, go direction1 point)
    else if can_move direction2 point field
    then Some (point, go direction2 point)
    else if can_move direction3 point field
    then Some (point, go direction3 point)
    else if can_move direction4 point field
    then Some (point, go direction4 point)
    else None
    
let directions =
    let cycle = [| (North, South, West, East)
                   (South, West, East, North)
                   (West, East, North, South)
                   (East, North, South, West) |] in
    Seq.initInfinite (fun i -> cycle[i % cycle.Length])
    
let round field four_directions =
    let set_unzip sequence =
        let minus, plus = Seq.toArray sequence |> Array.unzip
        (Set.ofArray minus, Set.ofArray plus)
    
    let first_half field four_directions =
        field |> Seq.choose (try_move field four_directions)
              |> Seq.groupBy snd
              |> Seq.choose (snd >> Seq.tryExactlyOne)
              |> set_unzip
              
    let second_half field (minus, plus) =
        if Set.count minus = 0
        then None
        else Some (Set.difference field minus |> Set.union plus)
        
    field |> Option.bind (fun field -> first_half field four_directions |> second_half field)
    
let empty_count field =
    let field = field |> Option.defaultValue Set.empty |> Set.toArray
    if field.Length = 0
    then 0
    else let mutable x_min = field[0].x
         let mutable x_max = field[0].x
         let mutable y_min = field[0].y
         let mutable y_max = field[0].y
         
         for i in 1..field.Length - 1 do
             if x_min > field[i].x then x_min <- field[i].x
             if x_max < field[i].x then x_max <- field[i].x
             if y_min > field[i].y then y_min <- field[i].y
             if y_max < field[i].y then y_max <- field[i].y
         
         let width = x_max - x_min + 1
         let height = y_max - y_min + 1
        
         width * height - field.Length

    
let load_field(lines: string seq) =
    let collect_row y = Seq.mapi (fun x char -> (char, { y = y; x = x })) 
    lines |> Seq.mapi collect_row 
          |> Seq.concat
          |> Seq.choose (fun (char, point) -> if char = '#' then Some point else None)
          |> Set.ofSeq
    
let solve_a (lines: string seq) =
    let field = load_field lines
    
    directions |> Seq.scan round (Some field)
               |> Seq.item 10
               |> empty_count


let solve_b (lines: string seq) =
    let field = load_field lines
    
    directions |> Seq.scan round (Some field)
               |> Seq.takeWhile Option.isSome
               |> Seq.length
