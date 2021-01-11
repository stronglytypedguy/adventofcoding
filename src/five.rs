use std::collections::BinaryHeap;

pub fn solver(input: &String) -> String {
    let seats: Vec<&str> = input.split("\n").collect();
    
    let mut heap = BinaryHeap::new();
    for seat in seats {
        let seat_id = calculate_seat_id(&seat);
        heap.push(seat_id)
    }
    let max = heap.pop().unwrap();
    while heap.pop().unwrap() == heap.peek().unwrap() + 1 {
    }
    return format!("Max is: {}, My Seat is: {}", max, heap.peek().unwrap() + 1).to_string();
}

fn calculate_seat_id(seat: &str) -> i64 {
    let mut lower_row = 0f64;
    let mut upper_row = 127f64;
    let mut lower_col = 0f64;
    let mut upper_col = 7f64;
    for letter in seat.chars() {
        if letter == 'F' {
            let half = ((lower_row + upper_row) / 2f64).floor();
            upper_row = half;
        } else if letter == 'B' {
            let half = ((lower_row + upper_row) / 2f64).ceil();
            lower_row = half;
        } else if letter == 'L' {
            let half = ((lower_col + upper_col) / 2f64).floor();
            upper_col = half;
        } else if letter == 'R' {
            let half = ((lower_col + upper_col) / 2f64).ceil();
            lower_col = half;
        }
    }
    return ((lower_row * 8f64) + lower_col) as i64; // Note the choice of lower/upper is irrelevant since they end up being equal.
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