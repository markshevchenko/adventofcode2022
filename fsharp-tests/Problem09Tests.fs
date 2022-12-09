module Problem09Tests

open Xunit
open Problem09

[<Fact>]
let ``move should return valid values`` () =
    Assert.Equal ({ row = 1; column = 0 }, move Up { row = 0; column = 0 })
    Assert.Equal ({ row = 0; column = 1 }, move Right { row = 0; column = 0 })
    Assert.Equal ({ row = -1; column = 0 }, move Down { row = 0; column = 0 })
    Assert.Equal ({ row = 0; column = -1 }, move Left { row = 0; column = 0 })

    
[<Fact>]
let ``move_tail should move tail valid`` () =
    let head = ({ row = 0; column = 0 })
    let offset row column position =
        { row = position.row + row; column = position.column + column }
    let check_offset row column = move_tail head (offset row column head)
    
    Assert.Equal ({ row = -1; column = -1 }, check_offset -1 -1)
    Assert.Equal ({ row = -1; column = 0 }, check_offset -1 0)
    Assert.Equal ({ row = -1; column = 1 }, check_offset -1 1)
    Assert.Equal ({ row = 0; column = -1 }, check_offset 0 -1)
    Assert.Equal ({ row = 0; column = 0 }, check_offset 0 0)
    Assert.Equal ({ row = 0; column = 1 }, check_offset 0 1)
    Assert.Equal ({ row = 1; column = -1 }, check_offset 1 -1)
    Assert.Equal ({ row = 1; column = 0 }, check_offset 1 0)
    Assert.Equal ({ row = 1; column = 1 }, check_offset 1 1)
    Assert.Equal ({ row = -1; column = -1 }, check_offset -2 -2)
    Assert.Equal ({ row = -1; column = 0 }, check_offset -2 -1)
    Assert.Equal ({ row = -1; column = 0 }, check_offset -2 0)
    Assert.Equal ({ row = -1; column = 0 }, check_offset -2 1)
    Assert.Equal ({ row = -1; column = 1 }, check_offset -2 2)
    Assert.Equal ({ row = 1; column = -1 }, check_offset 2 -2)
    Assert.Equal ({ row = 1; column = 0 }, check_offset 2 -1)
    Assert.Equal ({ row = 1; column = 0 }, check_offset 2 0)
    Assert.Equal ({ row = 1; column = 0 }, check_offset 2 1)
    Assert.Equal ({ row = 1; column = 1 }, check_offset 2 2)
    Assert.Equal ({ row = 0; column = -1 }, check_offset -1 -2)
    Assert.Equal ({ row = 0; column = -1 }, check_offset 0 -2)
    Assert.Equal ({ row = 0; column = -1 }, check_offset 1 -2)
    Assert.Equal ({ row = 0; column = 1 }, check_offset -1 2)
    Assert.Equal ({ row = 0; column = 1 }, check_offset 0 2)
    Assert.Equal ({ row = 0; column = 1 }, check_offset 1 2)


[<Fact>]
let ``parse should split line to direction and integer`` () =
    Assert.Equal ((Up, 4), parse "U 4")
    Assert.Equal ((Right, 12), parse "R 12")
    Assert.Equal ((Down, 5), parse "D 5")
    Assert.Equal ((Left, 1), parse "L 1")


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
let ``solve_b with sample returns 1`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (1, actual)


let sample2 = [
    "R 5"
    "U 8"
    "L 8"
    "D 3"
    "R 17"
    "D 10"
    "L 25"
    "U 20" ]


[<Fact>]
let ``solve_b with sample2 returns 36`` () =
    let actual = solve_b sample2 
                  
    Assert.Equal (36, actual)
