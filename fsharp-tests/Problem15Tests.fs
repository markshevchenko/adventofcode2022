module Problem15Tests

open Xunit
open Problem15

[<Fact>]
let ``parse_line parses points`` () =
    let p1, p2 = parse_line "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    
    Assert.Equal ({x = 2; y = 18}, p1)
    Assert.Equal ({x = -2; y = 15}, p2)


let sample = [
    "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
    "Sensor at x=9, y=16: closest beacon is at x=10, y=16"
    "Sensor at x=13, y=2: closest beacon is at x=15, y=3"
    "Sensor at x=12, y=14: closest beacon is at x=10, y=16"
    "Sensor at x=10, y=20: closest beacon is at x=10, y=16"
    "Sensor at x=14, y=17: closest beacon is at x=10, y=16"
    "Sensor at x=8, y=7: closest beacon is at x=2, y=10"
    "Sensor at x=2, y=0: closest beacon is at x=2, y=10"
    "Sensor at x=0, y=11: closest beacon is at x=2, y=10"
    "Sensor at x=20, y=14: closest beacon is at x=25, y=17"
    "Sensor at x=17, y=20: closest beacon is at x=21, y=22"
    "Sensor at x=16, y=7: closest beacon is at x=15, y=3"
    "Sensor at x=14, y=3: closest beacon is at x=15, y=3"
    "Sensor at x=20, y=1: closest beacon is at x=15, y=3" ]


[<Fact>]
let ``solve_a with sample returns 26`` () =
    let actual = solve_a sample
                  
    Assert.Equal (26, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 0`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (0, actual)
