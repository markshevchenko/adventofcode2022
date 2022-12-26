module Problem19

open System.Collections.Generic
open System.Diagnostics
open System.Text.RegularExpressions

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


type Frame = {
    ore_robot: int
    ore: Ore
    clay_robot: int
    clay: Clay
    obsidian_robot: int
    obsidian: Obsidian
    geode_robot: int
    geode: Geode
    minute: int
}


let init_frame =
    { ore_robot = 0
      ore = Ore 0
      clay_robot = 0
      clay = Clay 0
      obsidian_robot = 0
      obsidian = Obsidian 0
      geode_robot = 0
      geode = Geode 0
      minute = 0 }
    

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
        minute = previous.minute + 1 }


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
    let rec loop (queue: Queue<Frame>) =
        let mutable next_frames = set []
        for frame in queue do
            for next_frame in make_next_frames blueprint frame do
                next_frames.Add (next_frame) |> ignore
                
        if next_frames.Count = 0 then failwith "Path not found."
                
        let next_frames = next_frames
                       |> Set.toSeq
                       |> Seq.sortByDescending (fun it -> it.geode)
                       |> Seq.take 10000
                       |> Seq.toArray
                       
        if next_frames[0].minute = 24
        then let (Geode geode) = next_frames[0].geode
             let (Id id) = blueprint.id
             id * geode
        else loop (Queue (next_frames |> Array.toSeq))
    
    loop (Queue (seq { init_frame }))


let solve_a (lines: string seq) =
    lines |> Seq.map parse_blueprint
          |> Seq.map accelerated_bfs
          |> Seq.sum
    

let solve_b (lines: string seq) =
    0