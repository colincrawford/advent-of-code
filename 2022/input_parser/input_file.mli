type puzzle_input = char list list

(** Read in a day's input text file *)
val read : input_file_dir:string -> day:int -> puzzle_input

(** Print out an input file's contents *)
val display : puzzle_input -> unit
