module Problem07Tests

open Xunit
open Problem07

let sample = [
    "$ cd /"
    "$ ls"
    "dir a"
    "14848514 b.txt"
    "8504156 c.dat"
    "dir d"
    "$ cd a"
    "$ ls"
    "dir e"
    "29116 f"
    "2557 g"
    "62596 h.lst"
    "$ cd e"
    "$ ls"
    "584 i"
    "$ cd .."
    "$ cd .."
    "$ cd d"
    "$ ls"
    "4060174 j"
    "8033020 d.log"
    "5626152 d.ext"
    "7214296 k" ]|> Seq.ofList


[<Fact>]
let ``solve_a with sample returns 95437`` () =
    
    let actual = solve_a sample
                  
    Assert.Equal(95_437UL, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 24933642`` () =
    
    let actual = solve_b sample 
                  
    Assert.Equal(24_933_642UL, actual)
