module Problem04Tests

open Xunit
open Problem04

[<Fact>]
let ``parse "2-3,7-8" returns ((2, 3), (7, 8))`` () =
    Assert.Equal (((2, 3), (7, 8)), parse "2-3,7-8")


let sample = [|
    "2-4,6-8"
    "2-3,4-5"
    "5-7,7-9"
    "2-8,3-7"
    "6-6,4-6"
    "2-6,4-8" |]
    

[<Fact>]
let ``solve_a with sample returns 2`` () =
    let actual = solve_a sample 
                  
    Assert.Equal (2, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 4`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (4, actual)