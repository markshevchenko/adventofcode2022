module UtilsTests

open Xunit
open Utils


[<Fact>]
let ``Seq.from_reader with empty string returns empty seq`` () =
    let actual = "" |> String.to_reader |> Seq.from_reader 
    
    Assert.Empty actual
    
    
[<Fact>]
let ``Seq.from_reader with Foo\n\nBar\nBaz returns "Foo", "", "Bar", "Baz"`` () =
    let actual = "Foo\n\nBar\nBaz" |> String.to_reader |> Seq.from_reader
    
    Assert.Equal([|"Foo"; ""; "Bar"; "Baz"|], actual)


[<Fact>]
let ``Seq.split with 0 [1 2 3 0 4 5 6 0 7 8 9] returns [[1 2 3] [4 5 6] [7 8 9]]`` () =
    let actual = Seq.split 0 [|1; 2; 3; 0; 4; 5; 6; 0; 7; 8; 9|]
    
    Assert.Equal([| Array.toSeq [|1; 2; 3|]; [|4; 5; 6|]; [|7; 8; 9|] |], actual)
    