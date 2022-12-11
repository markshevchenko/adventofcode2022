module Problem11Tests

open System.Collections.Generic
open Xunit
open Problem11

[<Fact>]
let ``execute should return valid values`` () =
    Assert.Equal (7UL, execute 5UL (Add 2UL))
    Assert.Equal (10UL, execute 5UL (Multiply 2UL))
    Assert.Equal (25UL, execute 5UL Square)
    
    
[<Fact>]
let ``try_parse_suffix checks for prefix`` () =
    Assert.Equal (Some 5, try_parse_suffix "prefix: " int "prefix: 5")
    Assert.Equal (None, try_parse_suffix "prefix: " int "not a valid prefix: 5")
    
    
[<Fact>]
let ``parse_operation makes valid operation`` () =
    Assert.Equal (Add 5UL, parse_operation "old + 5")
    Assert.Equal (Multiply 3UL, parse_operation "old * 3")
    Assert.Equal (Square, parse_operation "old * old")
    
    
[<Fact>]
let ``gcd should be valid`` () =
    Assert.Equal (1UL, gcd 2UL 3UL)
    Assert.Equal (1UL, gcd 3UL 2UL)
    Assert.Equal (20UL, gcd 100UL 80UL)
    Assert.Equal (20UL, gcd 60UL 80UL)


[<Fact>]
let ``lcm should be valid`` () =
    Assert.Equal (6UL, lcm 2UL 3UL)
    Assert.Equal (6UL, lcm 3UL 2UL)
    Assert.Equal (80UL, lcm 16UL 20UL)
    Assert.Equal (20UL, lcm 5UL 20UL)


[<Fact>]
let ``parse_monkey makes valid Monkey`` () =
    let description = [|
        "Monkey 2:"
        "  Starting items: 79, 60, 97"
        "  Operation: new = old * old"
        "  Test: divisible by 13"
        "    If true: throw to monkey 1"
        "    If false: throw to monkey 3" |]
    let monkey = parse_monkey description
    
    Assert.Equal<IEnumerable<uint64>> (seq {79UL; 60UL; 97UL}, monkey.Items)
    Assert.Equal (Square, monkey.Operation)
    Assert.Equal (13I, monkey.DivisibleBy)
    Assert.Equal (1, monkey.IfTrue)
    Assert.Equal (3, monkey.IfFalse)
    

let sample = [
    "Monkey 0:"
    "  Starting items: 79, 98"
    "  Operation: new = old * 19"
    "  Test: divisible by 23"
    "    If true: throw to monkey 2"
    "    If false: throw to monkey 3"
    ""
    "Monkey 1:"
    "  Starting items: 54, 65, 75, 74"
    "  Operation: new = old + 6"
    "  Test: divisible by 19"
    "    If true: throw to monkey 2"
    "    If false: throw to monkey 0"
    ""
    "Monkey 2:"
    "  Starting items: 79, 60, 97"
    "  Operation: new = old * old"
    "  Test: divisible by 13"
    "    If true: throw to monkey 1"
    "    If false: throw to monkey 3"
    ""
    "Monkey 3:"
    "  Starting items: 74"
    "  Operation: new = old + 3"
    "  Test: divisible by 17"
    "    If true: throw to monkey 0"
    "    If false: throw to monkey 1" ]


[<Fact>]
let ``solve_a with sample returns 10605`` () =
    let actual = solve_a sample
                  
    Assert.Equal (10605UL, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 2713310158`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (2_713_310_158UL, actual)
