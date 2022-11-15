type puzzle_input = char list list

val read : input_file_dir:string -> day:int -> puzzle_input
(** Read in a day's input text file *)

val display : puzzle_input -> unit
(** Print out an input file's contents *)

val to_int_grid : puzzle_input -> int array array
(** Util for parsing integer grids *)
