module Problem19Tests

open Xunit
open Problem19

let sample = [
    "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian."
    "Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian." ]

[<Fact>]
let ``solve_a with sample returns 33`` () =
    let actual = solve_a sample
                  
    Assert.Equal (33, actual)
    
    
[<Fact(Skip = "Too long")>]
let ``solve_b with sample returns 56 * 62`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (56 * 62, actual)
