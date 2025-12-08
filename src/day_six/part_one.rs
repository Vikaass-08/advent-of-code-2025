use std::{fs};

pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_six.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let lines: Vec<&str> = file_content.lines().collect();
  let last_line = lines.last().ok_or("Something went wrong")?;

  let opeation_per_column: Vec<&str> = last_line.trim().split_whitespace().collect();


  let mut result: Vec<i128> = vec![0; opeation_per_column.len()];

  for (idx, line) in lines.iter().enumerate() {
    // Skip last line
    if idx == lines.len() - 1 {
        continue;
    }

    let numbers: Vec<i128> = line
      .trim()
      .split_whitespace()
      .map(|val| val.parse::<i128>().map_err(|err| err.to_string()))
      .collect::<Result<Vec<i128>, String>>()?;

    for (n_idx, num) in numbers.iter().enumerate() {
      if idx == 0 {
        result[n_idx] = num.clone();
      } else if opeation_per_column[n_idx] == "+" {
        result[n_idx] += num;
      } else {
        result[n_idx] *= num;
      }
    }
    
  }

  Ok(result.iter().sum())
}
