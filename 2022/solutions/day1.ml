let part1 _input = "<Part 1 answer here>"
let part2 _input = "<Part 2 answer here>"

let solve : Day_solution.solver =
 fun input -> { part1 = part1 input; part2 = part2 input }

let%test_unit "Day1 outputs match" =
  Test_helpers.solutions_match ~day:1 ~solver:solve ~expected_part1:""
    ~expected_part2:""
