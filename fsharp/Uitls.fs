module Utils

open System.IO

module Seq =
    let from_reader (text_reader: TextReader) = seq {
        let mutable line = text_reader.ReadLine ()
        while line <> null do
            yield line
            line <- text_reader.ReadLine ()
    }
    
    let split<'a when 'a: equality> (separator: 'a) (sequence: 'a seq) =
        let add_key (key, prev_item) item =
            if key = 0 then (1, item)
            else if item = separator then (key + 1, item)
            else (key, item)
        
        sequence |> Seq.scan add_key (0, Unchecked.defaultof<'a>)
                 |> Seq.filter (fun (key, item) -> key <> 0 && item <> separator)
                 |> Seq.groupBy fst
                 |> Seq.map snd
                 |> Seq.map (Seq.map snd)
