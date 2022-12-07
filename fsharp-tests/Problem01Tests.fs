module Problem01Tests

open Xunit
open Problem01

let sample = [|
    "1000"
    "2000"
    "3000"
    ""
    "4000"
    ""
    "5000"
    "6000"
    ""
    "7000"
    "8000"
    "9000"
    ""
    "10000" |] 

[<Fact>]
let ``solve_a with sample data returns 24000`` () =
    let actual = solve_a sample 
                  
    Assert.Equal (24000, actual) 

[<Fact>]
let ``solve_b with sample data returns 45000`` () =
                  
    let actual = solve_b sample 
                  
    Assert.Equal (45000, actual)