module Problem03Tests

open Xunit
open Problem03


[<Fact>]
let ``priority returns valid values`` () =
    Assert.Equal (1, priority 'a')
    Assert.Equal (26, priority 'z')
    Assert.Equal (27, priority 'A')
    Assert.Equal (52, priority 'Z')


let sample = [|
    "vJrwpWtwJgWrhcsFMMfFFhFp"
    "jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL"
    "PmmdzqPrVvPwwTWBwg"
    "wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn"
    "ttgJtRGJQctTZtZT"
    "CrZsJsPPZsGzwwsLwLmpwMDw" |]


[<Fact>]
let ``solve_a with sample returns 157`` () =
    let actual = solve_a sample 
                  
    Assert.Equal (157, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 70`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (70, actual)