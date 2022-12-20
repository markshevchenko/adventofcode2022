module Problem15

open System.Text.RegularExpressions

[<Struct>]
type Point =
    { x: int
      y: int }


type Interval =
    { start: int
      finish: int }

    static member create center half_width =
        if half_width < 0 then None
        else Some { start = center - half_width; finish = center + half_width }
        
    static member size interval = interval.finish - interval.start + 1
    
    static member join (intervals: Interval seq) =
        let combine list interval =
            match list with
            | [] -> [interval]
            | head::tail -> if interval.start > head.finish
                            then interval::head::tail
                            else { start = head.start
                                   finish = max interval.finish head.finish }::tail
        intervals |> Seq.sortBy (fun it -> it.start)
                  |> Seq.fold combine []


[<Struct>]
type Rhombus =
    { sensor: Point
      beacon: Point }

    static member private regex =
        Regex("""Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)""",
              RegexOptions.Compiled)
    static member parse s =
        let ``match`` = Rhombus.regex.Match s
        if ``match``.Success
        then let x1 = int ``match``.Groups[1].Value
             let y1 = int ``match``.Groups[2].Value
             let x2 = int ``match``.Groups[3].Value
             let y2 = int ``match``.Groups[4].Value
             
             { sensor = { x = x1; y = y1 }; beacon = { x = x2; y = y2 } }
        else failwith "Unrecognized rhombus definition"

    static member scan y rhombus =
        let half_diagonal = abs (rhombus.sensor.x - rhombus.beacon.x)
                          + abs (rhombus.sensor.y - rhombus.beacon.y)
        let y_diff = abs (y - rhombus.sensor.y)
        
        Interval.create rhombus.sensor.x (half_diagonal - y_diff)
        
    static member min_max_y rhombus =
        let half_diagonal = abs (rhombus.sensor.x - rhombus.beacon.x)
                          + abs (rhombus.sensor.y - rhombus.beacon.y)
        
        (rhombus.sensor.y - half_diagonal, rhombus.sensor.y + half_diagonal)


let solve_a (y: int) (lines: string seq) =
    let rhombuses = lines |> Seq.map Rhombus.parse
                          |> Array.ofSeq
    
    let coverages_size = rhombuses |> Seq.map (Rhombus.scan y)
                                   |> Seq.choose id
                                   |> Interval.join
                                   |> List.sumBy Interval.size
                               
    let sensors_count = rhombuses |> Seq.map (fun it -> it.sensor)
                                  |> Seq.filter (fun it -> it.y = y)
                                  |> Seq.distinct
                                  |> Seq.length
                                  
    let beacons_count = rhombuses |> Seq.map (fun it -> it.beacon)
                                  |> Seq.filter (fun it -> it.y = y)
                                  |> Seq.distinct
                                  |> Seq.length
    
    coverages_size - sensors_count - beacons_count


let solve_b (max_xy: int) (lines: string seq) =
    let rhombuses = lines |> Seq.map Rhombus.parse
                          |> Array.ofSeq

    let min_y = rhombuses |> Array.map (Rhombus.min_max_y >> fst) |> Array.min
    let max_y = rhombuses |> Array.map (Rhombus.min_max_y >> snd) |> Array.max

    let mutable y = (max min_y 0)
    let mutable x = None
    while x.IsNone && y <= (min max_y max_xy) do
        let intervals = rhombuses |> Seq.map (Rhombus.scan y)
                                  |> Seq.choose id
                                  |> Interval.join
                                  |> List.pairwise
                                  |> List.filter (fun (b, a) -> a.finish = b.start - 2)
                                  |> List.tryExactlyOne
                                  
        match intervals with
        | Some ({ start = result; finish = _ }, _) -> x <- Some (result - 1)
        | None -> y <- y + 1

    match x with
    | Some x -> (int64 x * 4000000L) + int64 y
    | _ -> failwith "No result"
    