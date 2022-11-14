(* Solution for both parts of a day's problem *)
type solution =
    { part1: string
    ; part2: string }

(* Function signature for each day's solution *)
type solver = Input_parser.Input_file.puzzle_input -> solution
