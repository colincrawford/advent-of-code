open Core

module CharSet = Set.Make(Char)

type sack = char list * char list

exception OddNumberOfItemsInSack

let sum_list lst = List.fold ~init:0 ~f:Int.( + ) lst

(* Split a list at the half point into a tuple with each half - (left, right) *)
let split_line_in_half (line : char list) : sack =
  let list_len = List.length line in
  let () = if list_len % 2 <> 0 then raise OddNumberOfItemsInSack else () in
  let half_inx = list_len / 2 in
  List.split_n line half_inx

exception InvalidInputChar of char

(*
   From the prompt
   'a'..'z' = 1..26
   'A'..'Z' = 27..52

   # Char.to_int 'a';;
   - : int = 97
   # Char.to_int 'A';;
   - : int = 65
*)
let char_score chr =
  let num = Char.to_int chr in
  let is_upper = num >= 65 && num <= 90 in
  let is_lower = num >= 97 && num <= 122 in
  let () = if not (is_upper || is_lower) then raise (InvalidInputChar chr) in
  if is_lower then num - 96 else num - 64 + 26

let find_intersection char_lists =
  char_lists
  |> List.map ~f:(CharSet.of_list)
  |> List.reduce ~f:(fun acc s -> CharSet.inter acc s)
  |> Option.value ~default:(CharSet.empty)
  |> Set.to_list

let score_for_items items =
  items |> List.map ~f:char_score |> sum_list |> Int.to_string

let part1 input =
  List.map ~f:split_line_in_half input
  |> List.bind ~f:(fun (left, right) -> find_intersection [ left; right ])
  |> score_for_items

exception InvalidNumberOfCommonItems of int
exception InvalidElfGroupSize of int

let chunks_of_three lines = List.groupi ~break:(fun i _ _ -> i % 3 = 0) lines

let part2 input =
  let find_badge elf1 elf2 elf3 =
    let common_items = find_intersection [ elf1; elf2; elf3 ] in
    let num_common_items = List.length common_items in
    if num_common_items <> 1 then
      raise (InvalidNumberOfCommonItems num_common_items)
    else List.nth_exn common_items 0
  in
  let find_id = function
    | [ elf1; elf2; elf3 ] -> find_badge elf1 elf2 elf3
    | _ as group -> raise (InvalidElfGroupSize (List.length group))
  in
  input |> chunks_of_three |> List.map ~f:find_id |> score_for_items

let solve : Day_solution.solver =
 fun input -> { part1 = part1 input; part2 = part2 input }

let%test_unit "Day2 outputs match" =
  Test_helpers.solutions_match ~day:3 ~solver:solve ~expected_part1:"8109"
    ~expected_part2:"2738"
