module Problem03

open Microsoft.FSharp.Collections

let priority c =
    if c >= 'a'
    then int c - int 'a' + 1
    else int c - int 'A' + 27


let solve_a (lines: string seq) =
    lines |> Seq.map (fun line -> (line[..line.Length/2 - 1], line[line.Length/2..]))
          |> Seq.map (fun (left, right) -> (Set.ofSeq left, Set.ofSeq right))
          |> Seq.map (fun (left, right) -> Set.intersect left right)
          |> Seq.map Seq.exactlyOne
          |> Seq.map priority
          |> Seq.sum


let solve_b (lines: string seq) =
    lines |> Seq.chunkBySize 3
          |> Seq.map (fun triple -> triple |> Seq.map Set.ofSeq |> Set.intersectMany |> Seq.exactlyOne)
          |> Seq.map priority
          |> Seq.sum
