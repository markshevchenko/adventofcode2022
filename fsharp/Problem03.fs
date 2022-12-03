module Problem03

open Microsoft.FSharp.Collections

let priority c =
    if c >= 'a'
    then int c - int 'a' + 1
    else int c - int 'A' + 27


let solve_a (lines: string seq) =
    let mutable result = 0
    for line in lines do
        let left_half = Set.ofSeq line[..line.Length/2 - 1] 
        let right_half = Set.ofSeq line[line.Length/2..]
        let share_item = Set.intersect left_half right_half |> Seq.exactlyOne
        
        result <- result + priority share_item
        
    result


let solve_b (lines: string seq) =
    let mutable result = 0
    for triple in Seq.chunkBySize 3 lines do
        let share_item = triple |> Seq.map Set.ofSeq
                                |> Set.intersectMany
                                |> Seq.exactlyOne
        
        result <- result + priority share_item
        
    result
