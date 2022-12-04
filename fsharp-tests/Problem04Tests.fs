module Problem04Tests

open Xunit
open Utils
open Problem04

[<Fact>]
let ``parse "2-3,7-8" returns ((2, 3), (7, 8))`` () =
    Assert.Equal (((2, 3), (7, 8)), parse "2-3,7-8")
    

[<Fact>]
let ``solve_a with sample returns 2`` () =
    let sample = "2-4,6-8\n\
                  2-3,4-5\n\
                  5-7,7-9\n\
                  2-8,3-7\n\
                  6-6,4-6\n\
                  2-6,4-8"
    
    let actual = solve_a (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(2, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 4`` () =
    let sample = "2-4,6-8\n\
                  2-3,4-5\n\
                  5-7,7-9\n\
                  2-8,3-7\n\
                  6-6,4-6\n\
                  2-6,4-8"
    
    let actual = solve_b (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(4, actual)