use std::{cmp::min, collections::HashSet, fs, i32};

pub fn solution() -> Result<i32, String> {
  let file_path = "./src/static/input_part_ten.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let mut total_sum = 0;

  for line in file_content.lines() {
    let manual: Vec<&str> = line.split_whitespace().collect();


    let mut switch: Vec<bool> = manual[0]
        .replace('[', "")
        .replace(']', "")
        .chars()
        .map(|c| if c == '#' {false} else {true})
        .collect();

    let operations = manual
      .iter()
      .enumerate()
      .filter(|(idx, _)| {
        return *idx >= 1 && *idx < manual.len() - 1
      })
      .map(|(_, positions)|{
        return positions
          .replace("(", "")
          .replace(")", "")
          .split(",")
          .map(|val| val.parse::<usize>().map_err(|err|err.to_string()))
          .collect::<Result<Vec<usize>, String>>();
      })
      .collect::<Result<Vec<Vec<usize>>, String>>()?;
    total_sum += get_min_operation_count(0, 0, &mut switch, &operations);

  }

  return Ok(total_sum);
}



fn get_min_operation_count(curr_idx: usize, operation_done: i32, switch: &mut Vec<bool>, operations: &Vec<Vec<usize>>) -> i32 {

  if !switch.contains(&false) {
    return operation_done;
  }
  else if curr_idx >= operations.len() {
    return i32::MAX;
  }

  let without = get_min_operation_count(curr_idx + 1, operation_done, switch, operations);

  operations[curr_idx].iter().for_each(|op|{
    if *op < switch.len() {
      switch[*op] = !switch[*op];
    }
  });

  let with = get_min_operation_count(curr_idx + 1, operation_done + 1, switch, operations);

  operations[curr_idx].iter().for_each(|op|{
    if *op < switch.len() {
      switch[*op] = !switch[*op];
    }
  });

  return min(with, without);
}