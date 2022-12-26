module Problem19

open System.Collections.Generic
open System.Diagnostics
open System.Text.RegularExpressions
open Microsoft.FSharp.Collections

type Id = Id of int
type Ore = Ore of int
type Clay = Clay of int
type Obsidian = Obsidian of int
type Geode = Geode of int

type Blueprint = {
    id: Id
    ore_robot_cost: Ore
    clay_robot_cost: Ore
    obsidian_robot_cost: Ore * Clay
    geode_robot_cost: Ore * Obsidian
}

let blueprint_regex = Regex("""Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.""")
let parse_blueprint (line: string) =
    let ``match`` = blueprint_regex.Match line
    Debug.Assert ``match``.Success
    
    { id = ``match``.Groups[1].Value |> int |> Id
      ore_robot_cost = ``match``.Groups[2].Value |> int |> Ore
      clay_robot_cost = int ``match``.Groups[3].Value |> int |> Ore
      obsidian_robot_cost = (``match``.Groups[4].Value |> int |> Ore, ``match``.Groups[5].Value |> int |> Clay)
      geode_robot_cost = (``match``.Groups[6].Value |> int |> Ore, ``match``.Groups[7].Value |> int |> Obsidian) }


type Frame =
    { ore_robot: int
      ore: Ore
      clay_robot: int
      clay: Clay
      obsidian_robot: int
      obsidian: Obsidian
      geode_robot: int
      geode: Geode
      previous: Frame option }
    
    
let rec print_frame minute = function
    | Some frame ->
        let (Ore ore) = frame.ore
        let (Clay clay) = frame.clay
        let (Obsidian obsidian) = frame.obsidian
        let (Geode geode) = frame.geode
        printfn $"Minute {minute}: {ore}, {clay}, {obsidian}, {geode}"
        printfn $"Robots - O {frame.ore_robot}, C {frame.clay_robot}, O {frame.obsidian_robot}, G {frame.geode_robot}"
        
        print_frame (minute - 1) frame.previous
    | None -> ()


let init_frame =
    { ore_robot = 1
      ore = Ore 0
      clay_robot = 0
      clay = Clay 0
      obsidian_robot = 0
      obsidian = Obsidian 0
      geode_robot = 0
      geode = Geode 0
      previous = None }
    

let growth (previous: Frame) (next: Frame) =
    let (Ore ore) = next.ore
    let (Clay clay) = next.clay
    let (Obsidian obsidian) = next.obsidian
    let (Geode geode) = next.geode
    
    { next with
        ore = Ore (ore + previous.ore_robot)
        clay = Clay (clay + previous.clay_robot)
        obsidian = Obsidian (obsidian + previous.obsidian_robot)
        geode = Geode (geode + previous.geode_robot)
        previous = Some previous }


let or_collect_ore_robot (blueprint: Blueprint) (previous: Frame) (next_frames: Frame list) =
    let (Ore ore_cost) = blueprint.ore_robot_cost
    let (Ore ore) = previous.ore
    
    if ore >= ore_cost
    then let next = { previous with
                        ore = Ore (ore - ore_cost)
                        ore_robot = previous.ore_robot + 1 }
         next::next_frames
    else next_frames


let or_collect_clay_robot (blueprint: Blueprint) (previous: Frame) (next_frames: Frame list) =
    let (Ore ore_cost) = blueprint.clay_robot_cost
    let (Ore ore) = previous.ore
    
    if ore >= ore_cost
    then let next = { previous with
                        ore = Ore (ore - ore_cost)
                        clay_robot = previous.clay_robot + 1 }
         next::next_frames
    else next_frames
    

let or_collect_obsidian_robot (blueprint: Blueprint) (previous: Frame) (next_frames: Frame list) =
    let (Ore ore_cost, Clay clay_cost) = blueprint.obsidian_robot_cost
    let (Ore ore) = previous.ore
    let (Clay clay) = previous.clay
    
    if ore >= ore_cost && clay >= clay_cost
    then let next = { previous with
                        ore = Ore (ore - ore_cost)
                        clay = Clay (clay - clay_cost)
                        obsidian_robot = previous.obsidian_robot + 1 }
         next::next_frames
    else next_frames
    

let or_collect_geode_robot (blueprint: Blueprint) (previous: Frame) (next_frames: Frame list) =
    let (Ore ore_cost, Obsidian obsidian_cost) = blueprint.geode_robot_cost
    let (Ore ore) = previous.ore
    let (Obsidian obsidian) = previous.obsidian
    
    if ore >= ore_cost && obsidian >= obsidian_cost
    then let next = { previous with
                        ore = Ore (ore - ore_cost)
                        obsidian = Obsidian (obsidian - obsidian_cost)
                        geode_robot = previous.geode_robot + 1 }
         next::next_frames
    else next_frames
    
    
let make_next_frames (blueprint: Blueprint) (previous: Frame) =
    [previous] |> or_collect_ore_robot blueprint previous
               |> or_collect_clay_robot blueprint previous
               |> or_collect_obsidian_robot blueprint previous
               |> or_collect_geode_robot blueprint previous
               |> List.map (growth previous)
               
               
let accelerated_bfs (blueprint: Blueprint) =
    let rec loop (queue: Queue<Frame>) (minute: int) =
        let next_frames =
            seq { for frame in queue do
                      yield! make_next_frames blueprint frame |> Seq.ofList }
            |> Seq.sortByDescending (fun it -> (it.geode,
                                                it.geode_robot,
                                                it.obsidian_robot,
                                                it.clay_robot,
                                                it.ore_robot))
            |> Seq.truncate 100000
            |> Seq.toArray
            
        if next_frames.Length = 0 then failwith "Path not found."
                
        if minute = 23
        then let (Geode geode) = next_frames[0].geode
             // print_frame 24 (Some (next_frames[0]))
             let (Id id) = blueprint.id
             id * geode
        else loop (Queue (next_frames |> Array.toSeq)) (minute + 1)
    
    loop (Queue (seq { init_frame })) 0


let solve_a (lines: string seq) =
    let blueprints = lines |> Seq.map parse_blueprint |> Seq.toArray
    let results = blueprints |> Seq.map accelerated_bfs |> Seq.toArray
    results |> Seq.sum
    

let solve_b (lines: string seq) =
    0