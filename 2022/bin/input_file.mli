(** Read in a day's input text file *)
val read : input_file_dir:string -> day:int -> char list list

(** Print out an input file's contents *)
val display : char list list -> unit
