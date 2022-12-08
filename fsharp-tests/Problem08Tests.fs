module Problem08Tests

open Xunit
open Problem08

let sample = [
    "30373"
    "25512"
    "65332"
    "33549"
    "35390" ]


[<Fact>]
let ``solve_a with sample returns 21`` () =
    let actual = solve_a sample
                  
    Assert.Equal (21, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 8`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (8, actual)
