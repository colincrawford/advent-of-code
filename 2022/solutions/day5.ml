open Core

type stacks = char list list
type instruction = { from_stack : int; to_stack : int; num_to_move : int }

let add_line_to_stacks (stacks : stacks) (line : string) =
  let map_stack inx lst =
    match String.nget line ((inx * 4) + 1) with ' ' -> lst | c -> c :: lst
  in
  List.mapi ~f:map_stack stacks

let rec run_instruction stacks instruction =
  let { from_stack; to_stack; num_to_move } = instruction in
  let moved_cargo = List.nth_exn stacks (from_stack - 1) |> List.hd_exn in
  let new_stacks =
    List.mapi
      ~f:(fun i stack ->
        if i = from_stack - 1 then List.tl_exn stack
        else if i = to_stack - 1 then moved_cargo :: stack
        else stack)
      stacks
  in
  if num_to_move = 1 then new_stacks
  else
    run_instruction new_stacks
      { from_stack; to_stack; num_to_move = num_to_move - 1 }

let parse_instruction line =
  let pieces = String.split ~on:' ' line in
  let num_to_move = List.nth_exn pieces 1 |> Int.of_string in
  let from_stack = List.nth_exn pieces 3 |> Int.of_string in
  let to_stack = List.nth_exn pieces 5 |> Int.of_string in
  { num_to_move; from_stack; to_stack }

let parse_input (input : char list list) =
  let lines = List.map ~f:String.of_char_list input in
  let cargo_lines =
    List.take_while ~f:(fun line -> not (String.equal "" line)) lines
  in
  let instruction_lines =
    List.tl_exn
      (List.drop_while ~f:(fun line -> not (String.equal "" line)) lines)
  in
  let instructions = List.map ~f:parse_instruction instruction_lines in
  let stack_num_line = List.last_exn cargo_lines in
  let cargo_stack_lines = List.take cargo_lines (List.length cargo_lines - 1) in
  let num_stacks = (String.length stack_num_line + 1) / 4 in
  let empty_stacks = List.init num_stacks ~f:(Fn.const []) in
  let stacks =
    List.fold ~init:empty_stacks ~f:add_line_to_stacks
      (* Reverse so that the top line is processed last & the cargo char is a the head of its lane's list *)
      (List.rev cargo_stack_lines)
  in
  (stacks, instructions)

let run_instruction_bulk stacks instruction =
  let { from_stack; to_stack; num_to_move } = instruction in
  let moved_cargo_stack = List.nth_exn stacks (from_stack - 1) in
  let moved_cargo = List.take moved_cargo_stack num_to_move in
  List.mapi
    ~f:(fun i stack ->
      if i = from_stack - 1 then List.drop stack num_to_move
      else if i = to_stack - 1 then List.append moved_cargo stack
      else stack)
    stacks

let part1 stacks instructions =
  let final_stacks = List.fold ~init:stacks ~f:run_instruction instructions in
  List.map ~f:List.hd_exn final_stacks |> String.of_char_list

let part2 stacks instructions =
  let final_stacks =
    List.fold ~init:stacks ~f:run_instruction_bulk instructions
  in
  List.map ~f:List.hd_exn final_stacks |> String.of_char_list

let solve : Day_solution.solver =
 fun input ->
  let stacks, instructions = parse_input input in
  { part1 = part1 stacks instructions; part2 = part2 stacks instructions }

let%test_unit "Day5 outputs match" =
  Test_helpers.solutions_match ~day:5 ~solver:solve ~expected_part1:"WSFTMRHPP"
    ~expected_part2:"GSLCMFBRP"
