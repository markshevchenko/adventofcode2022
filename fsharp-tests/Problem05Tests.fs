module Problem05Tests

open Xunit
open Problem05


[<Fact>]
let ``solve_a with sample returns CMZ`` () =
    let sample = [ "    [D]    "
                   "[N] [C]    "
                   "[Z] [M] [P]"
                   " 1   2   3 "
                   ""
                   "move 1 from 2 to 1"
                   "move 3 from 1 to 3"
                   "move 2 from 2 to 1"
                   "move 1 from 1 to 2" ]
    
    let actual = solve_a sample 
                  
    Assert.Equal("CMZ", actual)
    
    
[<Fact>]
let ``solve_b with sample returns MCD`` () =
    let sample = [ "    [D]    "
                   "[N] [C]    "
                   "[Z] [M] [P]"
                   " 1   2   3 "
                   ""
                   "move 1 from 2 to 1"
                   "move 3 from 1 to 3"
                   "move 2 from 2 to 1"
                   "move 1 from 1 to 2" ]
    
    let actual = solve_b sample 
                  
    Assert.Equal("MCD", actual)
