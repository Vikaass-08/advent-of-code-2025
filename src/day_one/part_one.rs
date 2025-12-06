use std::fs;

pub fn solution() -> i32 {
  let file_path = "./src/static/input_part_one.txt";
  let file_content = fs::read_to_string(file_path);

  let mut position: i32 = i32::from(50);
  let mut count: i32 = 0;

  if let Ok(content) = file_content {
    let operations: Vec<&str> = content.lines().collect();
    for op in operations {
      let dir = &op[0..1];
      let to_be_moved: i32 = op[1..].parse().expect("failed to parse to_be_moved");
      let (_, pos) = self::move_postion(position, to_be_moved, dir);
      if pos == 0 {
        count += 1;
      }
      position = pos
    }
  }
  return count;
}


fn move_postion(curr_pos: i32, to_be_moved: i32, dir: &str) -> (i32, i32) {

  if to_be_moved == 0 {
    return (0, curr_pos);
  }

  let mut zero_counter = to_be_moved / 100;
  let trimmed_to_be_moved = (to_be_moved) % 100;

  if dir == "R" {
    if trimmed_to_be_moved + curr_pos >= 100  && curr_pos != 0{
      zero_counter += 1;
    }
    return (zero_counter, (curr_pos + trimmed_to_be_moved) % 100)
  }


  if trimmed_to_be_moved >= curr_pos  &&  curr_pos != 0 {
    zero_counter += 1;
  }

  return (zero_counter, (curr_pos + 100 - trimmed_to_be_moved) % 100);
}