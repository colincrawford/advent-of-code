open Core

(* Helper function for validating a day's solutions *)
let solutions_match ~(day : int) ~(solver : Day_solution.solver)
    ~(expected_part1 : string) ~(expected_part2 : string) =
  let input =
    Input_parser.Input_file.read ~day ~input_file_dir:"../../../puzzle_inputs"
  in
  let solution = solver input in
  [%test_eq: string] expected_part1 solution.part1;
  [%test_eq: string] expected_part2 solution.part2
