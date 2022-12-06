open Core
module CharSet = Set.Make (Char)

let find_first_unique_marker (input_stream : char list) (unique_seq_len : int) =
  let found_at, _ =
    List.foldi input_stream ~init:(0, []) ~f:(fun inx (found_at, acc) next ->
        if found_at <> 0 then (found_at, acc)
        else
          let new_list = next :: List.take acc (unique_seq_len - 1) in
          let num_unique = CharSet.length (CharSet.of_list new_list) in
          if num_unique = unique_seq_len then (inx + 1, new_list)
          else (0, new_list))
  in
  Int.to_string found_at

let solve : Day_solution.solver =
 fun input ->
  let input_stream = List.hd_exn input in
  let part1 = find_first_unique_marker input_stream 4 in
  let part2 = find_first_unique_marker input_stream 14 in
  { part1; part2 }

let%test_unit "Day6 outputs match" =
  Test_helpers.solutions_match ~day:6 ~solver:solve ~expected_part1:"1640"
    ~expected_part2:"3613"
