use std::{fs, vec};

pub fn solution() -> Result<i32, String> {
  let file_path = "./src/static/input_part_four.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let moves: Vec<Vec<i32>> = vec![
    vec![0, -1],
    vec![-1, -1],
    vec![-1, 0],
    vec![-1, 1],
    vec![0, 1],
    vec![1, 1],
    vec![1, 0],
    vec![1, -1]
  ];

  let lines: Vec<&str> = file_content.lines().collect();
  let mut rolls: Vec<Vec<char>> = Vec::new();
  for line in lines {
    rolls.push(Vec::new());
    let curr_row = rolls.len() - 1;
    rolls[curr_row].extend(line.chars());
  }

  let mut res = 0;

  loop {
    let count = self::get_valid_roll_count(&mut rolls, &moves);
    res += count;

    if count == 0 {
        break;
    }
  }

  return Ok(res);
}


fn get_valid_roll_count(rolls: &mut Vec<Vec<char>>, moves: &Vec<Vec<i32>> ) -> i32 {
  let row: usize = rolls.len();
  let col = rolls[0].len();

  let mut count: i32 = 0;
  let mut valid_cells = Vec::new();

  for r in 0..row {
    for c in 0..col {
      if rolls[r][c] == '.' || rolls[r][c] == 'x' {
        continue;
      }
      if self::get_adjacent_roll_count(r as i32, c as i32, row as i32, col as i32, rolls, &moves) < 4 {
        count += 1;
        valid_cells.push(vec![r, c]);
      }
    }
  }

  for cell in valid_cells {
    rolls[cell[0]][cell[1]] = 'x';
  }

  return count;
}

fn get_adjacent_roll_count(row: i32, col: i32, n: i32, m: i32, rolls: &Vec<Vec<char>>, moves: &Vec<Vec<i32>>) -> i32 {

  let mut adjacent_roll_count = 0;

  for pos in moves {
    if self::is_valid(row + pos[0], col + pos[1], n, m, rolls) {
      adjacent_roll_count += 1;
    }
  }

  return adjacent_roll_count;
}

fn is_valid(row: i32, col: i32, n: i32, m: i32, rolls: &Vec<Vec<char>>) -> bool {
  row >= 0 && row < n && col >= 0 && col < m && rolls[row as usize][col as usize] == '@'
}


