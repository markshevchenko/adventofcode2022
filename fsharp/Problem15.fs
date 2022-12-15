module Problem15

open System.Diagnostics
open System.Text.RegularExpressions

type Points = {
    x: int
    y: int
}

let parse_regex = Regex("""Sensor at x=(\-?\d+), y=(\-?\d+): closest beacon is at x=(\-?\d+), y=(\-?\d+)""")
let parse_line line =
    let m = parse_regex.Match line
    Debug.Assert (m.Success)
    
    let x1, y1, x2, y2 = (int m.Groups[1].Value, int m.Groups[2].Value, int m.Groups[3].Value, int m.Groups[4].Value)
    
    ({x = x1; y = y1}, {x = x2; y = y2})


let solve_a (lines: string seq) =
    0

let solve_b (lines: string seq) =
    0