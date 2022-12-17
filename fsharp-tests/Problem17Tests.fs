module Problem17Tests

open Xunit
open Problem17

let sample = [
    ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>" ]


[<Fact>]
let ``solve_a with sample returns 3068`` () =
    let actual = solve_a sample
                  
    Assert.Equal (3068L, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 1514285714288`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (1514285714288L, actual)
