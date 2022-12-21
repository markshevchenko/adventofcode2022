module Problem21Tests

open Xunit
open Problem21

[<Fact>]
let ``parse should parse lines`` () =
    Assert.Equal ((Name "cczh", Operation (Add, Name "sllz", Name "lgvd")), parse "cczh: sllz + lgvd")
    Assert.Equal ((Name "dvpt", Number 3), parse "dvpt: 3")

let sample = [
    "root: pppw + sjmn"
    "dbpl: 5"
    "cczh: sllz + lgvd"
    "zczc: 2"
    "ptdq: humn - dvpt"
    "dvpt: 3"
    "lfqf: 4"
    "humn: 5"
    "ljgn: 2"
    "sjmn: drzm * dbpl"
    "sllz: 4"
    "pppw: cczh / lfqf"
    "lgvd: ljgn * ptdq"
    "drzm: hmdt - zczc"
    "hmdt: 32" ]


[<Fact>]
let ``solve_a with sample returns 152`` () =
    let actual = solve_a sample
                  
    Assert.Equal (152L, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 301`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (301L, actual)
