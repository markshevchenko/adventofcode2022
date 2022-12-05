module Problem02Tests

open Xunit
open Utils
open Problem02


[<Fact>]
let ``solve_a with sample returns 15`` () =
    let sample = [ "A Y"
                   "B X"
                   "C Z" ] |> Seq.ofList
    
    let actual = solve_a sample 
                  
    Assert.Equal(15, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 12`` () =
    let sample = [ "A Y"
                   "B X"
                   "C Z" ] |> Seq.ofList
    
    let actual = solve_b sample 
                  
    Assert.Equal(12, actual)