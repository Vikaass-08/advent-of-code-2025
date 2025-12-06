use std::fs;

// Assuming it's ASCII digits, otherwise .chars() would fail
pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_three.txt";
  let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;

  let mut total_joltage: i128 = 0;

  for line in file_content.lines() {
    let line_size = line.len();
    let mut largest_joltage = String::new();
    let mut last_taken_idx = 0;

    for idx in 1..=12 {
      let slice = &line[last_taken_idx..(line_size - (12 - idx))];
      let largest_idx_in_slice = self::find_largest_number_idx(slice);
      largest_joltage.push(slice.as_bytes()[largest_idx_in_slice] as char);
      last_taken_idx = last_taken_idx + largest_idx_in_slice + 1;
    }

    total_joltage += largest_joltage.parse::<i128>().map_err(|e| e.to_string())?;
    println!("{}", largest_joltage.parse::<i128>().map_err(|e| e.to_string())?);
  }

  return Ok(total_joltage);
}


fn find_largest_number_idx(line: &str) -> usize {
  let mut largest_idx = 0 as usize;
  let mut largest_char = &b'0';

  for (idx, v) in line.as_bytes().iter().enumerate() {
    if v > largest_char {
      largest_char = v;
      largest_idx = idx;
    }
  }

  return largest_idx;
}