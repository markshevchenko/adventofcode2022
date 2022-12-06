module UtilsTests

open Xunit
open Utils


[<Fact>]
let ``Seq.split with 0 [1 2 3 0 4 5 6 0 7 8 9] returns [[1 2 3] [4 5 6] [7 8 9]]`` () =
    let actual = Seq.split 0 [|1; 2; 3; 0; 4; 5; 6; 0; 7; 8; 9|]
    
    Assert.Equal([| Array.toSeq [|1; 2; 3|]; [|4; 5; 6|]; [|7; 8; 9|] |], actual)
