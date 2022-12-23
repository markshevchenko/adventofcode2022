module Problem23Tests

open Xunit
open Problem23

let sample = [
    "....#.."
    "..###.#"
    "#...#.#"
    ".#...##"
    "#.###.."
    "##.#.##"
    ".#..#.." ]


[<Fact>]
let ``solve_a with sample returns 110`` () =
    let actual = solve_a sample
                  
    Assert.Equal (110, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 20`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (20, actual)
