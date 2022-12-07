module Problem07

open System
open System.Collections.Generic
open System.Text.RegularExpressions

type File =
    { name: string
      size: uint64 }
    
type Directory =
    { name: string
      parent: Directory option
      files: List<File>
      directories: List<Directory> }

let (|Cd|_|) parent s =
    let m = Regex.Match(s, @"\$ cd (.+)")
    if m.Success
    then Some({ name = m.Groups[1].Value
                parent = Some parent
                files = List ()
                directories = List () })
    else None
    
let (|FileSize|_|) s =
    let m = Regex.Match(s, @"(\d+) (.+)")
    if m.Success
    then Some( { name = m.Groups[2].Value
                 size = UInt64.Parse m.Groups[1].Value })
    else None

let build_tree (lines: string seq) =
    let mutable root = { name = "/"
                         parent = None
                         files = List ()
                         directories = List() }
    let mutable current_node = root
    for line in lines do
        match line with
        | "$ cd /" -> ()
        | "$ cd .." -> match current_node.parent with
                       | Some parent -> current_node <- parent
                       | None -> failwith "Unpair cd .. command"
        | "$ ls" -> ()
        | Cd current_node directory -> 
                     current_node.directories.Add(directory)
                     current_node <- directory
        | FileSize file -> current_node.files.Add(file)
        | _ -> ()
        
    root
    
    
let get_directory_sizes root =
    let rec collect directory: uint64 * uint64 list =
        let sum_file_size = directory.files
                         |> Seq.sumBy (fun f -> f.size)

        let collects = directory.directories
                    |> List.ofSeq
                    |> List.map collect
                 
        let sum_dir_size = collects |> List.map fst |> List.sum
        let total = collects |> List.map snd |> List.concat
        let sum_size = sum_file_size + sum_dir_size
        
        (sum_size, sum_size::total)         
        
    collect root |> snd |> Seq.ofList

    
let solve_a (lines: string seq) =
    let root = build_tree lines
    let sizes = get_directory_sizes root
    sizes |> Seq.filter ((>=) 100_000UL) 
          |> Seq.sum


let solve_b (lines: string seq) =
    let root = build_tree lines
    let sizes = get_directory_sizes root
    let used = Seq.max sizes
    let unused = 70_000_000UL - used
    let need_free = 30_000_000UL - unused
    sizes |> Seq.sort
          |> Seq.find ((<) need_free)
