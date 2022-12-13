module Problem13

open System.Diagnostics
open FParsec
open Utils

type Item =
    | IntItem of int
    | ListItem of Item list
    
let make_list n = ListItem [IntItem n]
    
let rec compare a b =
    match a, b with
    | IntItem a, IntItem b -> sign (a - b)
    | IntItem a, b -> compare (make_list a) b
    | a, IntItem b -> compare a (make_list b)
    | ListItem [], ListItem [] -> 0
    | ListItem [], _ -> -1
    | _, ListItem [] -> +1
    | ListItem (h1::t1), ListItem (h2::t2) -> let result = compare h1 h2
                                              if result = 0
                                              then compare (ListItem t1) (ListItem t2)
                                              else result


let parser_item, parser_item_ref = createParserForwardedToRef ()
let parser_number: Parser<Item, unit> = pint32 |>> IntItem
let parser_list = between (pstring "[") (pstring "]") (sepBy parser_item (pstring ",")) |>> ListItem
do parser_item_ref.Value <- parser_list <|> parser_number


let parse_line line =
    match run parser_item line with
    | Success (item, _, _) -> item
    | _ -> failwith "Unrecognized item"


let parse_lines (lines: string array) =
    Debug.Assert (lines.Length = 2)
    
    (parse_line lines[0], parse_line lines[1])


let solve_a (lines: string seq) =
    lines |> Seq.split ""
          |> Seq.map Array.ofSeq
          |> Seq.map parse_lines
          |> Seq.mapi (fun i (a, b) -> (i + 1, compare a b))
          |> Seq.sumBy (fun (i, result) -> if result = -1 then i else 0)


let solve_b (lines: string seq) =
    let packets = lines |> Seq.filter ((<>) "")
                        |> Seq.append (seq { "[[2]]"; "[[6]]" })
                        |> Seq.map parse_line
                        |> Seq.sortWith compare
                        |> Array.ofSeq
                        
    let index2 = 1 + Array.findIndex (compare (ListItem [make_list 2]) >> ((=) 0)) packets 
    let index6 = 1 + Array.findIndex (compare (ListItem [make_list 6]) >> ((=) 0)) packets 
    
    index2 * index6
