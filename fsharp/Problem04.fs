module Problem04


let (|Split|_|) (c: char) (s: string) =
    match s.Split c with
    | [|a; b|] -> Some (a, b)
    | _ -> None


let parse = function
    | Split ',' (Split '-' (a, b), Split '-' (c, d)) -> ((int a, int b), (int c, int d))
    | _ -> failwith "Input string doesn't match with format a-b,c-d"
    
    
let inside (a, b) (c, d) =
    a >= c && b <= d


let overlap (a, b) (c, d) =
    a <= d && c <= b


let solve_a (lines: string seq) =
    lines |> Seq.map parse
          |> Seq.filter (fun (r1, r2) -> inside r1 r2 || inside r2 r1)
          |> Seq.length


let solve_b (lines: string seq) =
    lines |> Seq.map parse
          |> Seq.filter (fun (r1, r2) -> overlap r1 r2)
          |> Seq.length
