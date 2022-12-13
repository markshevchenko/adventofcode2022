module Problem13Tests

open Xunit
open Problem13

module Assert =
    open FParsec
    
    let ParseEqual<'r>(expected: 'r, parser: Parser<'r, unit>, source: string) =
        match run parser source with
        | Success (result, _, _) -> Assert.Equal (expected, result)
        | Failure (message, _, _) -> Assert.True (false, message)
        
        
[<Fact>]
let ``parser_number parses int`` () =
    Assert.ParseEqual (IntItem 12, parser_number, "12")
    

[<Fact>]
let ``parser_list parses ints`` () =
    Assert.ParseEqual (ListItem [IntItem 12; IntItem 13], parser_list, "[12,13]")
    

[<Fact>]
let ``parser_list parses inner lists`` () =
    Assert.ParseEqual (ListItem [ListItem []], parser_list, "[[]]")


[<Fact>]
let ``parse_item parses int or list`` () =
    Assert.ParseEqual (IntItem 12, parser_item, "12")
    Assert.ParseEqual (ListItem [ListItem []], parser_item, "[[]]")
    
    
let sample = [
    "[1,1,3,1,1]"
    "[1,1,5,1,1]"
    ""
    "[[1],[2,3,4]]"
    "[[1],4]"
    ""
    "[9]"
    "[[8,7,6]]"
    ""
    "[[4,4],4,4]"
    "[[4,4],4,4,4]"
    ""
    "[7,7,7,7]"
    "[7,7,7]"
    ""
    "[]"
    "[3]"
    ""
    "[[[]]]"
    "[[]]"
    ""
    "[1,[2,[3,[4,[5,6,7]]]],8,9]"
    "[1,[2,[3,[4,[5,6,0]]]],8,9]" ]


[<Fact>]
let ``solve_a with sample returns 13`` () =
    let actual = solve_a sample
                  
    Assert.Equal (13, actual)
    
    
[<Fact>]
let ``solve_b with sample returns 140`` () =
    let actual = solve_b sample 
                  
    Assert.Equal (140, actual)
