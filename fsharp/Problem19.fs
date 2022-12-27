module Problem19

open System.Collections.Generic
open System.Diagnostics
open System.Text.RegularExpressions

type Id = Id of int
type Ore = Ore of int
type Clay = Clay of int
type Obsidian = Obsidian of int
type Geode = Geode of int


type Blueprint =
    { id: Id
      ore_bot_cost: Ore
      clay_bot_cost: Ore
      obsidian_bot_cost: Ore * Clay
      geode_bot_cost: Ore * Obsidian }
    
    member this.max_ore =
        let (Ore ore_ore) = this.ore_bot_cost
        let (Ore clay_ore) = this.clay_bot_cost
        let (Ore obsidian_ore, _) = this.obsidian_bot_cost
        let (Ore geode_ore, _) = this.geode_bot_cost
        
        Seq.max <| seq { ore_ore; clay_ore; obsidian_ore; geode_ore }
        
    static member regex = Regex("""Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.""") 
    static member parse(line: string) =
        let ``match`` = Blueprint.regex.Match line
        Debug.Assert ``match``.Success
        
        { id = ``match``.Groups[1].Value |> int |> Id
          ore_bot_cost = ``match``.Groups[2].Value |> int |> Ore
          clay_bot_cost = int ``match``.Groups[3].Value |> int |> Ore
          obsidian_bot_cost = (``match``.Groups[4].Value |> int |> Ore, ``match``.Groups[5].Value |> int |> Clay)
          geode_bot_cost = (``match``.Groups[6].Value |> int |> Ore, ``match``.Groups[7].Value |> int |> Obsidian) } 


type Frame =
    { ore_bots: int
      ore: Ore
      clay_bots: int
      clay: Clay
      obsidian_bots: int
      obsidian: Obsidian
      geode_bots: int
      geode: Geode
      minute: int }


let init_frame =
    { ore_bots = 1
      ore = Ore 0
      clay_bots = 0
      clay = Clay 0
      obsidian_bots = 0
      obsidian = Obsidian 0
      geode_bots = 0
      geode = Geode 0
      minute = 0 }


let bfs (max_minute: int) (blueprint: Blueprint) =
    let mutable queue = Queue ()
    let mutable seen = HashSet ()
    
    queue.Enqueue init_frame
    
    let rec loop max_geode =
        if queue.Count = 0
        then max_geode
        else let frame = queue.Dequeue ()
             let time_left = max_minute - frame.minute
             let (Geode geode) = frame.geode
             let max_geode = max max_geode geode
             let minute = frame.minute
        
             if time_left = 0
             then loop max_geode
             else let ore_bots = min frame.ore_bots blueprint.max_ore
                  let (Ore obsidian_ore, Clay obsidian_clay) = blueprint.obsidian_bot_cost
                  let clay_bots = min frame.clay_bots obsidian_clay
                  let (Ore geode_ore, Obsidian geode_obsidian) = blueprint.geode_bot_cost
                  let obsidian_bots = min frame.obsidian_bots geode_obsidian
                  let geode_bots = frame.geode_bots
                 
                  let (Ore ore) = frame.ore
                  let (Clay clay) = frame.clay
                  let (Obsidian obsidian) = frame.obsidian
                 
                  let ore = min ore (time_left * blueprint.max_ore - (time_left - 1) * ore_bots)
                  let clay = min clay (time_left * obsidian_clay - (time_left - 1) * clay_bots)
                  let obsidian = min obsidian (time_left * geode_obsidian - (time_left - 1) * obsidian_bots)
                 
                  let next_frame = { ore_bots = ore_bots
                                     ore = Ore ore
                                     clay_bots = clay_bots
                                     clay = Clay clay
                                     obsidian_bots = obsidian_bots
                                     obsidian = Obsidian obsidian
                                     geode_bots = frame.geode_bots
                                     geode = frame.geode
                                     minute = frame.minute }
                  if seen.Contains next_frame
                  then loop max_geode
                  else seen.Add next_frame |> ignore
                       queue.Enqueue { frame with
                                             ore = Ore (ore + ore_bots)
                                             clay = Clay (clay + clay_bots)
                                             obsidian = Obsidian (obsidian + obsidian_bots)
                                             geode = Geode (geode + geode_bots)
                                             minute = minute + 1 }

                       let (Ore ore_ore) = blueprint.ore_bot_cost
                       if ore >= ore_ore
                       then queue.Enqueue { frame with
                                                  ore = Ore (ore + ore_bots - ore_ore)
                                                  clay = Clay (clay + clay_bots)
                                                  obsidian = Obsidian (obsidian + obsidian_bots)
                                                  geode = Geode (geode + geode_bots)
                                                  ore_bots = ore_bots + 1
                                                  minute = minute + 1 }
                      
                       let (Ore clay_ore) = blueprint.clay_bot_cost
                       if ore >= clay_ore
                       then queue.Enqueue { frame with
                                                  ore = Ore (ore + ore_bots - clay_ore)
                                                  clay = Clay (clay + clay_bots)
                                                  obsidian = Obsidian (obsidian + obsidian_bots)
                                                  geode = Geode (geode + geode_bots)
                                                  clay_bots = clay_bots + 1
                                                  minute = minute + 1 }

                       if ore >= obsidian_ore && clay >= obsidian_clay 
                       then queue.Enqueue { frame with
                                                  ore = Ore (ore + ore_bots - obsidian_ore)
                                                  clay = Clay (clay + clay_bots - obsidian_clay)
                                                  obsidian = Obsidian (obsidian + obsidian_bots)
                                                  geode = Geode (geode + geode_bots)
                                                  obsidian_bots = obsidian_bots + 1
                                                  minute = minute + 1 }
                       
                       if ore >= geode_ore && obsidian >= geode_obsidian 
                       then queue.Enqueue { frame with
                                                  ore = Ore (ore + ore_bots - geode_ore)
                                                  clay = Clay (clay + clay_bots)
                                                  obsidian = Obsidian (obsidian + obsidian_bots - geode_obsidian)
                                                  geode = Geode (geode + geode_bots)
                                                  geode_bots = geode_bots + 1
                                                  minute = minute + 1 }

                       loop max_geode

    loop 0


let solve_a (lines: string seq) =
    lines |> Seq.map Blueprint.parse
          |> Seq.map (fun it -> let (Id id) = it.id in id * bfs 24 it)
          |> Seq.sum


let solve_b (lines: string seq) =
    lines |> Seq.map Blueprint.parse
          |> Seq.truncate 3
          |> Seq.map (bfs 32)
          |> Seq.fold (*) 1
