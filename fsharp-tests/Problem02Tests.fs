module Problem02Tests

open Xunit
open Problem02

let sample = [|
    "A Y"
    "B X"
    "C Z" |]

[<Fact>]
let ``solve_a with sample returns 15`` () =
    let actual = solve_a sample 
                  
    Assert.Equal (15, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 12`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (12, actual)