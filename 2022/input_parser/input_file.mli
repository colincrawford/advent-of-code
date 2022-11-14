type puzzle_input = char list list

(** Read in a day's input text file *)
val read : input_file_dir:string -> day:int -> puzzle_input

(** Print out an input file's contents *)
val display : puzzle_input -> unit

(** Util for parsing integer grids *)
val to_int_grid : puzzle_input -> int array array
