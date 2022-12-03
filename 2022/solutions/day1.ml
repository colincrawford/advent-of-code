open Core

let sum_list = List.fold ~init:0 ~f:Int.( + )

(* Get the sum of each blank line separated list of integers in the input lines
   For example:
   ```
   1
   2

   4
   ```
   Would ouput: (3, 4) *)
let get_sum_list input =
  List.map ~f:String.of_char_list input
  (* Split the list on blank lines *)
  |> List.group ~break:(fun _ b -> String.equal b "")
  |> List.map ~f:(fun l ->
         l
         (* Remove blank lines from groups of numbers *)
         |> List.filter ~f:(fun str -> not (String.equal str ""))
         (* Convert remaining lines to integers*)
         |> List.map ~f:Int.of_string)
  |> List.map ~f:sum_list

(* Sort in descending order *)
let get_sorted_calories input =
  get_sum_list input |> List.sort ~compare:(fun a b -> b - a)

let sum_first lst n = List.take lst n |> sum_list |> Int.to_string

let solve : Day_solution.solver =
 fun input ->
  let sorted_calories = get_sorted_calories input in
  let part1 = sum_first sorted_calories 1 in
  let part2 = sum_first sorted_calories 3 in
  { part1; part2 }

let%test_unit "Day1 outputs match" =
  Test_helpers.solutions_match ~day:1 ~solver:solve ~expected_part1:"65912"
    ~expected_part2:"195625"
