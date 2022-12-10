open Core

type fs_node =
  | Dir of { name : string; children : fs_node list; size : int }
  | File of { name : string; size : int }

let size = function Dir { size; _ } -> size | File { size; _ } -> size
let is_dir = function Dir _ -> true | File _ -> false

exception InvalidCmd of string

let parse_file line =
  let pieces = String.split ~on:' ' line in
  let size = List.nth_exn pieces 0 |> Int.of_string in
  let name = List.nth_exn pieces 1 in
  File { name; size }

let rec parse_ls (arr : string array) (start : int) : fs_node list * int =
  let files = ref [] in
  let i = ref (start + 1) in
  let line inx = Array.get arr inx in
  while
    !i < Array.length arr && not (String.is_prefix ~prefix:"$ " (line !i))
  do
    if not (String.is_prefix ~prefix:"dir " (line !i)) then
      files := parse_file (line !i) :: !files;
    i := !i + 1
  done;
  (!files, !i)

and parse_cd (arr : string array) (start : int) : fs_node list * int =
  let line = Array.get arr start in
  let name = String.chop_prefix_exn ~prefix:"$ cd " line in
  let children = ref [] in
  let start_pos = ref (start + 1) in
  let line inx = Array.get arr inx in
  while
    !start_pos < Array.length arr
    && not (String.equal "$ cd .." (line !start_pos))
  do
    let children', end_inx = parse_cmd arr !start_pos in
    start_pos := end_inx;
    children := children' @ !children
  done;
  let size = List.fold ~init:0 ~f:(fun acc node -> acc + size node) !children in
  ([ Dir { name; children = !children; size } ], !start_pos + 1)

and parse_cmd (arr : string array) (pos : int) : fs_node list * int =
  let line = Array.get arr pos in
  if String.is_prefix ~prefix:"$ ls" line then parse_ls arr pos
  else if String.is_prefix ~prefix:"$ cd" line then parse_cd arr pos
  else raise (InvalidCmd line)

let rec find_dirs node ~keep =
  match node with
  | File _ -> []
  | Dir { children; _ } as d ->
      d :: List.bind ~f:(find_dirs ~keep) children
      |> List.filter ~f:is_dir |> List.filter ~f:keep

let part1 root =
  root
  |> find_dirs ~keep:(fun node -> size node <= 100_000)
  |> List.fold ~init:0 ~f:(fun acc node -> acc + size node)
  |> Int.to_string

let part2 root =
  let total_disk_space = 70_000_000 in
  let space_needed = 30_000_000 in
  let max_usable = total_disk_space - space_needed in
  let need_to_free = size root - max_usable in
  root
  |> find_dirs ~keep:(fun node -> size node >= need_to_free)
  |> List.map ~f:(fun node -> size node)
  |> List.min_elt ~compare:(fun a b -> a - b)
  |> Option.value_exn |> Int.to_string

let solve : Day_solution.solver =
 fun input ->
  let lines = List.map ~f:String.of_char_list input in
  let lines_arr = Array.of_list lines in
  let nodes, _ = parse_cmd lines_arr 0 in
  let root = List.nth_exn nodes 0 in
  { part1 = part1 root; part2 = part2 root }

let%test_unit "Day7 outputs match" =
  Test_helpers.solutions_match ~day:7 ~solver:solve ~expected_part1:"1555642"
    ~expected_part2:"5974547"
