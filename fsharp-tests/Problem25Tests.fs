module Problem25Tests

open Xunit
open Problem25


[<Fact>]
let ``test_from_snafu`` () =
    Assert.Equal (1747L, from_snafu "1=-0-2")
    Assert.Equal (906L, from_snafu "12111")
    Assert.Equal (198L, from_snafu "2=0=")
    Assert.Equal (11L, from_snafu "21")
    Assert.Equal (201L, from_snafu "2=01")
    Assert.Equal (31L, from_snafu "111")
    Assert.Equal (1257L, from_snafu "20012")
    Assert.Equal (32L, from_snafu "112")
    Assert.Equal (353L, from_snafu "1=-1=")


[<Fact>]
let ``test_to_snafu`` () =
    Assert.Equal (to_snafu 1747L, "1=-0-2")
    Assert.Equal (to_snafu 906L, "12111")
    Assert.Equal (to_snafu 198L, "2=0=")
    Assert.Equal (to_snafu 11L, "21")
    Assert.Equal (to_snafu 201L, "2=01")
    Assert.Equal (to_snafu 31L, "111")
    Assert.Equal (to_snafu 1257L, "20012")
    Assert.Equal (to_snafu 32L, "112")
    Assert.Equal (to_snafu 353L, "1=-1=")

    
let sample = [
    "1=-0-2"
    "12111"
    "2=0="
    "21"
    "2=01"
    "111"
    "20012"
    "112"
    "1=-1="
    "1-12"
    "12"
    "1="
    "122" ]


[<Fact>]
let ``solve_a with sample returns "2=-1=0"`` () =
    let actual = solve_a sample
                  
    Assert.Equal ("2=-1=0", actual)
