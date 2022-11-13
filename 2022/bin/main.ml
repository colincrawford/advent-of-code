type solution_part = char list list -> string
type solutions = (solution_part * solution_part) array

let solutions: solutions =
  [| (Day1.part1, Day1.part2)
   ; (Day2.part1, Day2.part2)
   ; (Day3.part1, Day3.part2)
   ; (Day4.part1, Day4.part2)
   ; (Day5.part1, Day5.part2)
   ; (Day6.part1, Day6.part2)
   ; (Day7.part1, Day7.part2)
   ; (Day8.part1, Day8.part2)
   ; (Day9.part1, Day9.part2)
   ; (Day10.part1, Day10.part2)
   ; (Day11.part1, Day11.part2)
   ; (Day12.part1, Day12.part2)
   ; (Day13.part1, Day13.part2)
   ; (Day14.part1, Day14.part2)
   ; (Day15.part1, Day15.part2)
   ; (Day16.part1, Day16.part2)
   ; (Day17.part1, Day17.part2)
   ; (Day18.part1, Day18.part2)
   ; (Day19.part1, Day19.part2)
   ; (Day20.part1, Day20.part2)
   ; (Day21.part1, Day21.part2)
   ; (Day22.part1, Day22.part2)
   ; (Day23.part1, Day23.part2)
   ; (Day24.part1, Day24.part2)
   ; (Day25.part1, Day25.part2)
  |]

let print_day_solution day part1_solution part2_solution =
  [ ("Day " ^ (string_of_int day))
  ; ("  Part 1: " ^ part1_solution)
  ; ("  Part 2: " ^ part2_solution)
  ; ""
  ] |> List.iter print_endline

let solve (input_file_dir: string) (days: int list) =
  let solve_day (day, part1, part2) =
    let input_file = Input_file.read ~day ~input_file_dir in
    print_day_solution day (part1 input_file) (part2 input_file)
  in
  let with_solutions day =
    let (part1, part2) = Array.get solutions (day - 1) in
    (day, part1, part2)
  in
  days |> List.map with_solutions |> List.iter solve_day

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
  let input_file_directory = "./inputs" in
  (* Allow the user to pass in a list of days to run solutions for *)
  let input_days = parse_user_input_days () in
  (* Solutions will exist for 25 days *)
  let all_days = List.init 25 (Int.add 1) in
  (* Fall back to running solutions for all 25 days if no user input *)
  let days = if (List.length input_days) > 0 then input_days else all_days in
  solve input_file_directory days
