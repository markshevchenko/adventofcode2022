module Problem23

type Point =
    { y: int
      x: int }
    
type Direction =
    | North
    | East
    | South
    | West
    
type Field private (cells: char[,], min: Point, max: Point) =
    let mutable cells = cells
    let mutable min = min
    let mutable max = max
    let directions = [| North; South; West; East |]
    let mutable next_direction = 0
    
    let elves () =
        seq {
            for y in min.y..max.y do
                for x in min.x..max.x do
                    if cells[y, x] = '#'
                    then yield { y = y; x = x }
        }
        
    let can_move (elf: Point) = function
        | North -> cells[elf.y - 1, elf.x - 1] = '.'
                && cells[elf.y - 1, elf.x] = '.'
                && cells[elf.y - 1, elf.x + 1] = '.'
        | South -> cells[elf.y + 1, elf.x - 1] = '.'
                && cells[elf.y + 1, elf.x] = '.'
                && cells[elf.y + 1, elf.x + 1] = '.'
        | West -> cells[elf.y - 1, elf.x - 1] = '.'
               && cells[elf.y, elf.x - 1] = '.'
               && cells[elf.y + 1, elf.x - 1] = '.'
        | East -> cells[elf.y - 1, elf.x + 1] = '.'
               && cells[elf.y, elf.x + 1] = '.'
               && cells[elf.y + 1, elf.x + 1] = '.'
               
    let next_move (elf: Point) = function
        | North -> { elf with y = elf.y - 1 }
        | South -> { elf with y = elf.y + 1 }
        | West -> { elf with x = elf.x - 1 }
        | East -> { elf with x = elf.x + 1 }
               
    let next_point (elf: Point) =
        let direction1 = directions[next_direction]
        let direction2 = directions[(next_direction + 1) % directions.Length]
        let direction3 = directions[(next_direction + 2) % directions.Length]
        let direction4 = directions[(next_direction + 3) % directions.Length]
        
        if can_move elf direction1 then Some (elf, next_move elf direction1)
        else if can_move elf direction2 then Some (elf, next_move elf direction2)
        else if can_move elf direction3 then Some (elf, next_move elf direction3)
        else if can_move elf direction4 then Some (elf, next_move elf direction4)
        else None
        
    member _.prepare () =
        elves () |> Seq.choose next_point
                 |> Seq.groupBy snd
                 |> Seq.choose (snd >> Seq.tryExactlyOne)
                 
    member _.move (moves: (Point * Point) seq) =
        for (old_point, new_point) in moves do
            cells[old_point.y, old_point.x] <- '.'
            cells[new_point.y, new_point.x] <- '#'
            
            if min.y > new_point.y
            then min <- { min with y = new_point.y }
            if max.y < new_point.y
            then max <- { max with y = new_point.y }
            if min.x > new_point.x
            then min <- { min with x = new_point.x }
            if max.x < new_point.x
            then max <- { max with x = new_point.x }
            
            next_direction <- if next_direction = directions.Length - 1
                              then 0
                              else next_direction + 1
                              
    member _.empty_count () =
        let mutable count = 0
        for y in min.y..max.y do
            for x in min.x..max.x do
                if cells[y, x] = '.'
                then count <- count + 1
                
        count
        
    static member parse (lines: string seq) =
        let lines = Array.ofSeq lines
        let height = lines.Length
        let width = lines[0].Length
        let mutable cells = Array2D.create (height + 20) (width + 20) '.'
        for i in 0..height - 1 do
            for j in 0..width - 1 do
                cells[i + 10, j + 10] <- lines[i][j]

        Field (cells, { y = 10; x = 10 }, { y = height + 10 - 1; x = width + 10 - 1 })
    
    
let solve_a (lines: string seq) =
    let mutable field = Field.parse lines
    
    for _ in 1..10 do
        let moves = field.prepare ()
        field.move moves
        
    field.empty_count ()    


let solve_b (lines: string seq) =
    0