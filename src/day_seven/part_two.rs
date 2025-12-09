use std::fs;

pub fn solution() -> Result<i128, String> {
  let file_path = "./src/static/input_part_seven.txt";
  let file_content = fs::read_to_string(file_path).map_err(|e| e.to_string())?;

  let lines: Vec<&str> = file_content.lines().collect();

  let first_line: &&str = lines.first().ok_or_else(|| "First Line not found!")?;
  let entry_pos = first_line.find('S').ok_or_else(|| "(S) not found in first line")?;

  let mut dp: Vec<Vec<i128>> = vec![vec![-1; first_line.len()]; lines.len()];
  let puzzle: Vec<Vec<char>> = lines.into_iter().map(|line| line.chars().collect()).collect();


  let total_splits = self::get_all_paths(&puzzle, &mut dp, 1, entry_pos);
  Ok(total_splits)

}

fn get_all_paths(puzzle: &Vec<Vec<char>>, dp: &mut Vec<Vec<i128>>, row: usize, col: usize) -> i128 {
  if row >= puzzle.len() || col >= puzzle[0].len() {
    return 1;
  }

  if dp[row][col] != -1 {
    return dp[row][col];
  }

  if puzzle[row][col] == '.' {
    dp[row][col] = self::get_all_paths(puzzle, dp, row + 1, col);
    return dp[row][col];
  }

  let mut paths: i128 = 0;

  if col > 0 {
    paths += self::get_all_paths(puzzle, dp, row + 1, col - 1);
  }

  if col + 1 < puzzle[0].len() {
    paths += self::get_all_paths(puzzle, dp, row + 1, col + 1);
  }

  dp[row][col] = paths;

  return dp[row][col];
}