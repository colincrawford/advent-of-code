open Core

type choice = Rock | Paper | Scissors

exception InvalidInput of string

let move_to_choice (move : char) =
  match move with
  | 'A' | 'X' -> Rock
  | 'B' | 'Y' -> Paper
  | 'C' | 'Z' -> Scissors
  | _ -> raise (InvalidInput ("Invalid input for move: " ^ String.of_char move))

type outcome = Win | Lose | Draw

let input_to_outcome (input : char) =
  match input with
  | 'X' -> Lose
  | 'Y' -> Draw
  | 'Z' -> Win
  | _ ->
      raise
        (InvalidInput
           ("Invalid input for expected outcome: " ^ String.of_char input))

let move_for_outcome their_move outcome =
  match (their_move, outcome) with
  | _, Draw -> their_move
  | Rock, Lose | Paper, Win -> Scissors
  | Rock, Win | Scissors, Lose -> Paper
  | Paper, Lose | Scissors, Win -> Rock

let input_to_moves (input : char list list) :
    (choice * choice * outcome * choice) list =
  let line_to_moves line =
    let their_move = List.nth_exn line 0 |> move_to_choice in
    let my_move = List.nth_exn line 2 |> move_to_choice in
    let expected_outcome = List.nth_exn line 2 |> input_to_outcome in
    let expected_move = move_for_outcome their_move expected_outcome in
    (their_move, my_move, expected_outcome, expected_move)
  in
  List.map ~f:line_to_moves input

let score_choices my_choice their_choice =
  match (my_choice, their_choice) with
  | Rock, Paper -> 1 + 0
  | Rock, Rock -> 1 + 3
  | Rock, Scissors -> 1 + 6
  | Paper, Scissors -> 2 + 0
  | Paper, Paper -> 2 + 3
  | Paper, Rock -> 2 + 6
  | Scissors, Rock -> 3 + 0
  | Scissors, Scissors -> 3 + 3
  | Scissors, Paper -> 3 + 6

let sum_list lst = List.fold ~init:0 ~f:Int.( + ) lst

let part1 moves =
  moves
  |> List.map ~f:(fun (their_choice, my_choice, _, _) ->
         score_choices my_choice their_choice)
  |> sum_list |> Int.to_string

let part2 moves =
  moves
  |> List.map ~f:(fun (their_choice, _, _, my_choice) ->
         score_choices my_choice their_choice)
  |> sum_list |> Int.to_string

let solve : Day_solution.solver =
 fun input ->
  let parsed_input = input_to_moves input in
  { part1 = part1 parsed_input; part2 = part2 parsed_input }

let%test_unit "Day2 outputs match" =
  Test_helpers.solutions_match ~day:2 ~solver:solve ~expected_part1:"8392"
    ~expected_part2:"10116"
