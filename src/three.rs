pub fn solver(input: &String) -> String {
    println!("Starting Three");
    let rows: Vec<&str> = input.split("\n").collect();
    let mut counts = vec![];
    let downs = vec![1, 1, 1, 1, 2];
    let rights = vec![1, 3, 5, 7, 1];
    for i in 0..5 {
        let down = downs[i];
        let right = rights[i];
        let mut count = 0;
        let mut position_down = 0;
        let mut position_right = 0;
        while position_down < rows.len() - 1 {
            let row = rows[position_down];
            let tree = row.chars().nth(position_right).unwrap();
            if tree == '#' {
                count += 1;
            }
            position_down += down;
            position_right = (position_right + right) % row.len();
        }
        counts.push(count);
    }
    println!("{:?}", counts);
    let answer: u64 = counts.iter().fold(1, |acc, x| acc * x);
    println!("Finished Three, Answer: {:#?}", answer);
    return answer.to_string();
}


#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_three_solver() {
        let map = "........#....#..##..#...#.....#
        ...............#....##........#
        .#....##...##..#...............
        .#.......#......#.##..##...#...
        .....#.#....#..##...#.....#....
        ...#.#..##...###......#..#..#.#
        .....#..##........#.##......#..
        ..##.....###.........##........
        ..............##..#.#.#.#......
        .#....##..#.##.#....#..#.#..#..
        .#.#....#.##.#...#....#.....#..
        ..#...#.#.....#....#.......##..".to_string();
        assert_eq!(solver(&map), "0");
    }
}