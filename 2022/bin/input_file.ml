open Core

let print_char = Out_channel.output_char Out_channel.stdout
let print_newline () = Out_channel.newline Out_channel.stdout

(** Ex: ../inputs/day8.txt *)
let get_file_name ~input_file_dir ~day =
  input_file_dir ^ "/day" ^ (string_of_int day) ^ ".txt"

let read ~input_file_dir ~day =
  get_file_name ~day ~input_file_dir
  |> In_channel.read_all
  |> String.split_lines
  |> List.map ~f:(String.to_list)

(* Util for debugging *)
let display contents =
  let print_line line =
    List.iter line ~f:print_char;
    print_newline ()
  in
  contents |> List.iter ~f:print_line
  
