module Problem09Tests

open Xunit
open Problem09

let sample = [
    "R 4"
    "U 4"
    "L 3"
    "D 1"
    "R 4"
    "D 1"
    "L 5"
    "R 2" ]


[<Fact>]
let ``solve_a with sample returns 13`` () =
    let actual = solve_a sample
                  
    Assert.Equal (13, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 8`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (8, actual)
