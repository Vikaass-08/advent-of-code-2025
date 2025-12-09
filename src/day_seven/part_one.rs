use std::fs;

pub fn solution() -> Result<i32, String> {
  let file_path = "./src/static/input_part_seven.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let lines: Vec<&str> = file_content.lines().collect();

  let first_line = lines.first().ok_or_else(|| "First Line not found!")?;
  let entry_pos = first_line.find('S').ok_or_else(|| "(S) not found in first line")?;

  let mut is_beam_present: Vec<bool> = vec![false; first_line.len()];

  is_beam_present[entry_pos] = true;

  let mut total_splits: i32 = 0;

  for line in lines.iter() {
    for (idx, ch) in line.chars().enumerate() {
      if ch != '^' || !is_beam_present[idx] {
        continue;
      }

      if idx > 0 && !is_beam_present[idx - 1] {
        is_beam_present[idx - 1] = true;
      }
      if idx + 1 < line.len() {
        is_beam_present[idx + 1] = true;
      }

      is_beam_present[idx] = false;
      total_splits += 1;
    }
  }

  Ok(total_splits)

}
