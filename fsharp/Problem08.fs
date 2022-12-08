module Problem08

let make_trees lines =
    lines |> Seq.map Seq.toArray
          |> Seq.toArray


let is_visible (trees: char array array) row column =
    let tree_height = trees[row][column]
    let left_visible = seq { 0..column - 1 }
                    |> Seq.forall (fun i -> trees[row][i] < tree_height)
    let right_visible = seq { column + 1..trees[row].Length - 1 }
                     |> Seq.forall (fun i -> trees[row][i] < tree_height)
    let top_visible = seq { 0..row - 1 }
                   |> Seq.forall (fun i -> trees[i][column] < tree_height)
    let bottom_visible = seq { row + 1..trees.Length - 1 }
                      |> Seq.forall (fun i -> trees[i][column] < tree_height)
                      
    left_visible || top_visible || right_visible || bottom_visible


let solve_a (lines: string seq) =
    let trees = make_trees lines
    let width = trees[0].Length
    let height = trees.Length
    let mutable count = 2 * (height + width) - 4
    
    for i in 1..(height - 2) do
        for j in 1..(width - 2) do
            if is_visible trees i j
            then count <- count + 1
    
    count


let get_scenic_score (trees: char array array) row column =
    let tree_height = trees[row][column]
    let left_score = seq { column - 1..-1..1 }
                  |> Seq.takeWhile (fun i -> trees[row][i] < tree_height)
                  |> Seq.length
                  |> (+) 1
    let right_score = seq { column + 1..trees[row].Length - 2 }
                   |> Seq.takeWhile (fun i -> trees[row][i] < tree_height)
                   |> Seq.length
                   |> (+) 1
    let top_score = seq { row - 1..-1..1 }
                 |> Seq.takeWhile (fun i -> trees[i][column] < tree_height)
                 |> Seq.length
                 |> (+) 1
    let bottoms_score = seq { row + 1..trees.Length - 2 }
                     |> Seq.takeWhile (fun i -> trees[i][column] < tree_height)
                     |> Seq.length
                     |> (+) 1
                     
    left_score * top_score * right_score * bottoms_score


let solve_b (lines: string seq) =
    let trees = make_trees lines
    let width = trees[0].Length
    let height = trees.Length
    let mutable max_scenic_score = 0
    
    for i in 1..(height - 2) do
        for j in 1..(width - 2) do
            let scenic_score = get_scenic_score trees i j
            if max_scenic_score < scenic_score
            then max_scenic_score <- scenic_score
    
    max_scenic_score
