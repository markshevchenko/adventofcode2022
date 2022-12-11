module Problem11

open System.Collections.Generic
open System.Diagnostics
open Utils

type Operation =
    | Add of uint64
    | Multiply of uint64
    | Square


let execute a  = function
    | Add b -> a + b
    | Multiply b -> a * b 
    | Square -> a * a


let swap<'a> (f: 'a -> 'a -> 'a) (a: 'a) (b: 'a) =
    f b a


let try_parse_suffix<'a> (prefix: string) (f: string -> 'a) (s: string) =
    if s.StartsWith prefix then Some (f (s.Substring prefix.Length))
    else None


let parse_suffix<'a> (prefix: string) (f: string -> 'a) (s: string) =
    match try_parse_suffix prefix f s with
    | Some result -> result
    | None -> failwith "Unrecognized prefix"
    

let parse_operation description =
    if description = "old * old" then Square
    else match try_parse_suffix "old * " uint64 description with
         | Some n -> Multiply n
         | None -> parse_suffix "old + " (uint64 >> Add) description
         
         
let rec gcd a b =
    if a < b then gcd b a
    else if b = 0UL then a
    else gcd b (a % b)


let inline lcm a b = a * b / (gcd a b)


type Monkey(items: uint64 seq,
            operation: Operation,
            divisible_by: uint64,
            if_true: int,
            if_false: int) =

    let items = Queue<uint64> items

    let mutable total_inspects = 0
    
    member _.Items = items :> uint64 seq
    
    member _.Operation = operation
    
    member _.DivisibleBy = divisible_by
    
    member _.IfTrue = if_true
    
    member _.IfFalse = if_false
    
    member _.TotalInspects = total_inspects
    
    member _.Queue = items |> Seq.map string |> String.concat ", "
    
    member _.AddItem item = items.Enqueue item
    
    member _.InspectAndThrow (monkeys: Monkey array) (relax: uint64 -> uint64) =
        total_inspects <- total_inspects + items.Count
        while items.Count > 0 do
            let item = items.Dequeue ()
            let worry_level = relax (execute item operation)
            let whom = if worry_level % divisible_by = 0UL
                       then if_true
                       else if_false
            
            monkeys[whom].AddItem worry_level
            
            
type Monkeys(monkeys: Monkey seq) =
    let monkeys = Array.ofSeq monkeys
    let common_lcm = monkeys |> Seq.map (fun it -> it.DivisibleBy)
                             |> Seq.scan lcm 1UL
                             |> Seq.last

    let mutable round = 0
    
    member _.Round = round
    
    member _.Items = monkeys :> Monkey seq
    
    member _.Inspects = monkeys |> Seq.map (fun it -> it.TotalInspects)
                                |> Seq.map string
                                |> String.concat ", "
                                
    member _.Queues = monkeys |> Seq.map (fun it -> it.Queue)
                              |> String.concat "; "
                                
    member _.Round1 () =
        for i in 0..monkeys.Length - 1 do
          monkeys[i].InspectAndThrow monkeys ((swap (/)) 3UL)

        round <- round + 1

    member _.Round2 () =
        for i in 0..monkeys.Length - 1 do
          monkeys[i].InspectAndThrow monkeys ((swap (%)) common_lcm)
          
        round <- round + 1
        

let parse_monkey (lines: string array) =
    Debug.Assert (lines.Length = 6)
    let items = parse_suffix "  Starting items: " (fun s -> s.Split(", ") |> Seq.map uint64) lines[1]
    let operation = parse_suffix "  Operation: new = " parse_operation lines[2]
    let divisible_by = parse_suffix "  Test: divisible by " uint64 lines[3]
    let if_true = parse_suffix "    If true: throw to monkey " int lines[4]
    let if_false = parse_suffix "    If false: throw to monkey " int lines[5]
    Monkey(items, operation, divisible_by, if_true, if_false)


let parse (lines: string seq) =
    lines |> Seq.split ""
          |> Seq.map Seq.toArray
          |> Seq.map parse_monkey
          |> Seq.toArray
          |> Monkeys


let solve_a (lines: string seq) =
    let monkeys = parse lines
    for _ in 1..20 do
        monkeys.Round1 ()
    
    monkeys.Items |> Seq.map (fun it -> it.TotalInspects |> uint64)
                  |> Seq.sortDescending
                  |> Seq.take 2
                  |> Seq.fold (*) 1UL
            
            
let solve_b (lines: string seq) =
    let monkeys = parse lines
    for _ in 1..10000 do
        monkeys.Round2 ()
        
    monkeys.Items |> Seq.map (fun it -> it.TotalInspects |> uint64)
                  |> Seq.sortDescending
                  |> Seq.take 2
                  |> Seq.fold (*) 1UL
