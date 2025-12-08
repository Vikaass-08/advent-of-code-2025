use std::fs;

// Assuming it's ASCII digits, otherwise .chars() would fail
pub fn solution() -> Result<i32, String> {
  let file_path = "./src/static/input_part_three.txt";
  let file_content = fs::read_to_string(file_path).map_err(|err| err.to_string())?;

  let mut total_joltage: i32 = 0;

  for line in file_content.lines() {
    let largest_char_idx = self::find_largest_number_idx(line);
    if largest_char_idx == (line.len() - 1) {
      let slice: &str = &line[0..largest_char_idx];
      let second_largest_idx = self::find_largest_number_idx(slice);
      let mut combined: String = String::new();
      combined.push(slice.as_bytes()[second_largest_idx] as char);
      combined.push(line.as_bytes()[largest_char_idx] as char);
      total_joltage += combined.parse::<i32>().map_err(|e| e.to_string())?;
    }
    else {
      let slice = &line[(largest_char_idx + 1)..];
      let second_largest_idx = self::find_largest_number_idx(slice);
      let mut combined: String = String::new();
      combined.push(line.as_bytes()[largest_char_idx] as char);
      combined.push(slice.as_bytes()[second_largest_idx] as char);
      total_joltage += combined.parse::<i32>().map_err(|e| e.to_string())?;
    }
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
