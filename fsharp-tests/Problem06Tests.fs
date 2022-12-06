module Problem06Tests

open Xunit
open Problem06


[<Fact>]
let ``detect_marker should return sample values`` () =
    Assert.Equal(7, detect_marker 4 "mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    Assert.Equal(5, detect_marker 4 "bvwbjplbgvbhsrlpgdmjqwftvncz")
    Assert.Equal(6, detect_marker 4 "nppdvjthqldpwncqszvftbrmjlhg")
    Assert.Equal(10, detect_marker 4 "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    Assert.Equal(11, detect_marker 4 "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")

    Assert.Equal(19, detect_marker 14 "mjqjpqmgbljsphdztnvjfqwrcgsmlb")
    Assert.Equal(23, detect_marker 14 "bvwbjplbgvbhsrlpgdmjqwftvncz")
    Assert.Equal(23, detect_marker 14 "nppdvjthqldpwncqszvftbrmjlhg")
    Assert.Equal(29, detect_marker 14 "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg")
    Assert.Equal(26, detect_marker 14 "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw")
    