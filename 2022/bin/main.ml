module All_solutions = Solutions.All_solutions
module Input_file = Input_parser.Input_file

let print_day_solution (day: int) (solution: Solutions.Day_solution.solution) =
  [ ("Day " ^ (string_of_int day))
  ; ("  Part 1: " ^ solution.part1)
  ; ("  Part 2: " ^ solution.part2)
  ; ""
  ] |> List.iter print_endline

let solve (input_file_dir: string) (days: int list) =
  let solve_day day =
    Input_file.read ~day ~input_file_dir
    |> All_solutions.solver_for_day ~day
    |> print_day_solution day
  in
  days |> List.iter solve_day

(* Allow the user to pass in specific day solutions to run as CLI args
   Ex: `./main.exe 1 2 23` to only run the solutions for days 1, 2 and 23 *)
let parse_user_input_days () =
  let input_days = ref [] in
  (* Fn to handle each anonymous CLI arg *)
  let handle_input_day day = input_days := (int_of_string day) :: !input_days in
  let () = Arg.parse [] handle_input_day "Pass days to run or nothing to run all days" in
  (* Sort days to run in ascending order; 1 2 3... *)
  List.sort Int.sub !input_days


let () =
  (* Directory with the AOC text prompts stored in a text file per day *)
  let input_file_directory = "./puzzle_inputs" in
  (* Allow the user to pass in a list of days to run solutions for *)
  let input_days = parse_user_input_days () in
  (* Solutions will exist for 25 days *)
  let all_days = List.init 25 (Int.add 1) in
  (* Fall back to running solutions for all 25 days if no user input *)
  let days = if (List.length input_days) > 0 then input_days else all_days in
  solve input_file_directory days
