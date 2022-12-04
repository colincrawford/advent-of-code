open Core

type range = { start : int; stop : int }

exception InvalidInputLine of string
exception InvalidRange of string

let parse_range range_str =
  match String.split ~on:'-' range_str with
  | [ start; stop ] ->
      { start = Int.of_string start; stop = Int.of_string stop }
  | _ -> raise (InvalidRange range_str)

let to_ranges (line : string) : range * range =
  match String.split ~on:',' line with
  | [ left; right ] -> (parse_range left, parse_range right)
  | _ -> raise (InvalidInputLine line)

let left_fully_overlaps r1 r2 = r1.start <= r2.start && r1.stop >= r2.stop

let ranges_fully_overlap r1 r2 =
  left_fully_overlaps r1 r2 || left_fully_overlaps r2 r1

let part1 ranges =
  ranges
  |> List.filter ~f:(fun (r1, r2) -> ranges_fully_overlap r1 r2)
  |> List.length |> Int.to_string

let left_overlaps r1 r2 =
  (r1.start >= r2.start && r1.start <= r2.stop)
  || (r1.stop >= r2.start && r1.stop <= r2.stop)

let ranges_overlap r1 r2 = left_overlaps r1 r2 || left_overlaps r2 r1

let part2 ranges =
  ranges
  |> List.filter ~f:(fun (r1, r2) -> ranges_overlap r1 r2)
  |> List.length |> Int.to_string

let solve : Day_solution.solver =
 fun input ->
  let input_lines = List.map ~f:String.of_char_list input in
  let ranges = List.map ~f:to_ranges input_lines in
  { part1 = part1 ranges; part2 = part2 ranges }

let%test_unit "Day4 outputs match" =
  Test_helpers.solutions_match ~day:4 ~solver:solve ~expected_part1:"503"
    ~expected_part2:"827"
