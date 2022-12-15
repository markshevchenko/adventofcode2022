module Problem15Tests

open Xunit
open Problem15

[<Fact>]
let ``coverage with sensor at 8,7 and beacon at 2,10 should make valid intervals`` () =
    let sensor = { x = 8; y = 7 }
    let beacon = { x = 2; y = 10 }
    let rhombus = { sensor = sensor; beacon = beacon }
    
    Assert.Equal (None, Rhombus.scan -3 rhombus)
    Assert.Equal (Some { start = 8; finish = 8 }, Rhombus.scan -2 rhombus)
    Assert.Equal (Some { start = 5; finish = 11 }, Rhombus.scan 1 rhombus)
    Assert.Equal (Some { start = -1; finish = 17 }, Rhombus.scan 7 rhombus)
    Assert.Equal (Some { start = 1; finish = 15 }, Rhombus.scan 9 rhombus)
    Assert.Equal (Some { start = 8; finish = 8 }, Rhombus.scan 16 rhombus)
    Assert.Equal (None, Rhombus.scan 20 rhombus)


[<Fact>]
let ``Interval.size with start = finish equals to 1`` () =
    Assert.Equal (1, Interval.size { start = 100; finish = 100 })

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
    let actual = solve_a 10 sample
                  
    Assert.Equal (26, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 56000011`` () =
    let actual = solve_b 20 sample 
                  
    Assert.Equal (56000011L, actual)
