module Problem01Tests

open Xunit
open Utils
open Problem01


[<Fact>]
let ``solve_a with sample data returns 24000`` () =
    let sample = "1000\n\
                  2000\n\
                  3000\n\
                  \n\
                  4000\n\
                  \n\
                  5000\n\
                  6000\n\
                  \n\
                  7000\n\
                  8000\n\
                  9000\n\
                  \n\
                  10000"
                  
    let actual = solve_a (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(24000, actual) 

[<Fact>]
let ``solve_b with sample data returns 45000`` () =
    let sample = "1000\n\
                  2000\n\
                  3000\n\
                  \n\
                  4000\n\
                  \n\
                  5000\n\
                  6000\n\
                  \n\
                  7000\n\
                  8000\n\
                  9000\n\
                  \n\
                  10000"
                  
    let actual = solve_b (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(45000, actual)