use std::cmp;

pub fn solver(input: &String) -> String {
    let seats: Vec<&str> = input.split("\n").collect();
    let mut max = 0;
    for seat in seats {
        let mut column = 0;
        let mut row = 0;
        let mut min_row = 0;
        let mut max_row = 127;
        let mut min_col = 0;
        let mut max_col = 8;
        for (i, c) in seat.chars().enumerate() {
            println!("{} {} {} {} {}", c, min_row, max_row, min_col, max_col);
            if i == 6 {
                match c {
                    'F' => row = max_row,
                    'B' => row = min_row,
                    _ => continue,
                }
            } else if i == 9 {
                match c {
                    'R' => column = max_col,
                    'L' => column = min_col,
                    _ => continue,
                }
            } else {
                match c {
                    'F' => max_row = (max_row + min_row) / 2,
                    'B' => min_row = (max_row + min_row) / 2,
                    'L' => max_col = (max_col + min_col) / 2,
                    'R' => min_col = (max_col + min_col) / 2,
                    _ => continue,
                }
            }
        }
        max = cmp::max(max, row * 8 + column);
        println!("{}, {}", row, column);
    }
    return max.to_string();
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_five_solver() {
        assert_eq!(solver(
            &"BFFFBBFRRR
            FFFBBBFRRR
            BBFFBBFRLL
            FBFBBFFRLR".to_string()
        ),
        "820")
    }
}