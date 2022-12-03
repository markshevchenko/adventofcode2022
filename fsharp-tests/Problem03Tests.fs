module Problem03Tests

open Xunit
open Utils
open Problem03


[<Fact>]
let ``priority returns valid values`` () =
    Assert.Equal (1, priority 'a')
    Assert.Equal (26, priority 'z')
    Assert.Equal (27, priority 'A')
    Assert.Equal (52, priority 'Z')


[<Fact>]
let ``solve_a with sample returns 157`` () =
    let sample = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                  PmmdzqPrVvPwwTWBwg\n\
                  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                  ttgJtRGJQctTZtZT\n\
                  CrZsJsPPZsGzwwsLwLmpwMDw\n"
    
    let actual = solve_a (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(157, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 70`` () =
    let sample = "vJrwpWtwJgWrhcsFMMfFFhFp\n\
                  jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n\
                  PmmdzqPrVvPwwTWBwg\n\
                  wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n\
                  ttgJtRGJQctTZtZT\n\
                  CrZsJsPPZsGzwwsLwLmpwMDw\n"
    
    let actual = solve_b (sample |> String.to_reader |> Seq.from_reader) 
                  
    Assert.Equal(70, actual)