module Problem06

let no_duplicates (chars: string) =
    seq { 0 .. chars.Length - 2 }
    |> Seq.collect (fun i -> seq { i + 1 .. chars.Length - 1 }
                          |> Seq.map (fun j -> (i, j)))
    |> Seq.forall (fun (i, j) -> chars[i] <> chars[j])

let detect_marker (size: int) (buffer: string) =
    seq { 0 .. buffer.Length - size }
    |> Seq.map (fun i -> buffer[i .. i + size - 1])
    |> Seq.findIndex no_duplicates
    |> (+) size


let solve_a (lines: string seq) =
    lines |> Seq.head |> detect_marker 4


let solve_b (lines: string seq) =
    lines |> Seq.head |> detect_marker 14
