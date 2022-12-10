open Core

let tree_is_visible grid row col =
  let is_visible_top = ref true in
  let is_visible_bottom = ref true in
  let is_visible_right = ref true in
  let is_visible_left = ref true in
  let value = grid.(row).(col) in
  let full_row = grid.(row) in
  for col_inx = 0 to Array.length grid.(0) - 1 do
    if col_inx < col && full_row.(col_inx) >= value then
      is_visible_left := false
    else if col_inx > col && full_row.(col_inx) >= value then
      is_visible_right := false
  done;
  for row_inx = 0 to Array.length grid - 1 do
    if row_inx < row && grid.(row_inx).(col) >= value then
      is_visible_top := false
    else if row_inx > row && grid.(row_inx).(col) >= value then
      is_visible_bottom := false
  done;
  !is_visible_top || !is_visible_bottom || !is_visible_left || !is_visible_right

let visibility_score grid row col =
  let top_score = ref 0 in
  let right_score = ref 0 in
  let right_done = ref false in
  let bottom_score = ref 0 in
  let bottom_done = ref false in
  let left_score = ref 0 in
  let value = grid.(row).(col) in
  let full_row = grid.(row) in
  for col_inx = 0 to Array.length grid.(0) - 1 do
    if col_inx < col && full_row.(col_inx) >= value then left_score := 1
    else if col_inx < col && full_row.(col_inx) < value then
      left_score := !left_score + 1
    else if col_inx > col && full_row.(col_inx) >= value && not !right_done then (
      right_score := !right_score + 1;
      right_done := true)
    else if col_inx > col && full_row.(col_inx) < value && not !right_done then
      right_score := !right_score + 1
  done;
  for row_inx = 0 to Array.length grid - 1 do
    if row_inx < row && grid.(row_inx).(col) >= value then top_score := 1
    else if row_inx < row && grid.(row_inx).(col) < value then
      top_score := !top_score + 1
    else if row_inx > row && grid.(row_inx).(col) >= value && not !bottom_done
    then (
      bottom_score := !bottom_score + 1;
      bottom_done := true)
    else if row_inx > row && grid.(row_inx).(col) < value && not !bottom_done
    then bottom_score := !bottom_score + 1
  done;
  !top_score * !right_score * !bottom_score * !left_score

let part1 grid =
  let count = ref 0 in
  Array.iteri grid ~f:(fun row_inx row ->
      Array.iteri row ~f:(fun col_inx _ ->
          if tree_is_visible grid row_inx col_inx then count := !count + 1));
  !count |> Int.to_string

let part2 grid =
  let max_score = ref 0 in
  Array.iteri grid ~f:(fun row_inx row ->
      Array.iteri row ~f:(fun col_inx _ ->
          let score = visibility_score grid row_inx col_inx in
          if score > !max_score then max_score := score));
  !max_score |> Int.to_string

let solve : Day_solution.solver =
 fun input ->
  let map_row row = List.map ~f:Char.get_digit_exn row |> Array.of_list in
  let grid = List.map ~f:map_row input |> Array.of_list in
  { part1 = part1 grid; part2 = part2 grid }

let%test_unit "Day8 outputs match" =
  Test_helpers.solutions_match ~day:8 ~solver:solve ~expected_part1:"1717"
    ~expected_part2:"321975"
