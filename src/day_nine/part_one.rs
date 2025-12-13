use std::{cmp::max, fs};

pub fn solution() -> Result<i64, String> {
  let file_path = "./src/static/input_part_nine.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let positions: Vec<Vec<i64>> = file_content
    .lines()
    .into_iter()
    .map(|dimentions| dimentions
        .split(",")
        .map(|dir| dir.parse::<i64>().map_err(|err| err.to_string()))
        .collect::<Result<Vec<i64>, String>>()
    )
    .collect::<Result<Vec<Vec<i64>>, String>>()?;

  let largest_rectange_area: i64 = get_largest_rectangle_area(&positions);

  Ok(largest_rectange_area)
}


fn get_largest_rectangle_area(positions: &Vec<Vec<i64>>) -> i64 {

  let mut largest_rect: i64 = 0;

  for i in 0..positions.len() {
    for j in (i + 1)..positions.len() {
      let (a1, a2) = (positions[i][0], positions[i][1]);
      let (b1, b2) = (positions[j][0], positions[j][1]);
      let x_cor = (a1 - b1).abs() + 1;
      let y_cor = (a2 - b2).abs() + 1;

      if x_cor == y_cor {
        continue;
      }

      largest_rect = max(largest_rect, x_cor * y_cor);
    }
  }

  return largest_rect;
}