module Problem19

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

type State = {
    ore_robot: int
    ore: Ore
    clay_robot: int
    clay: Clay
    obsidian_robot: int
    obsidian: Obsidian
    geode_robot: int
    geode: Geode
}

let add_ore (state: State) =
    let (Ore ore) = state.ore
    { state with ore = Ore (ore + 1) }
    
let add_clay_robot (blueprint: Blueprint) (state: State) =
    if state.ore >= blueprint.clay_robot_cost
    then let (Ore ore) = state.ore
         let (Ore clay_robot_cost) = blueprint.clay_robot_cost
         { state with ore = Ore (ore - clay_robot_cost); clay_robot = state.clay_robot + 1 }
    else state

let make_state () =
    { ore_robot = 1
      ore = Ore 0
      clay_robot = 0
      clay = Clay 0
      obsidian_robot = 0
      obsidian = Obsidian 0
      geode_robot = 0
      geode = Geode 0 }


let solve_a (lines: string seq) =
    0
    

let solve_b (lines: string seq) =
    0