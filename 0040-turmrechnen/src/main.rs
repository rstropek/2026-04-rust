const START: i32 = 3;

fn calculate_result(start: i32) -> [(i32, char, i32, i32); 16] {
    let mut results = [(0, ' ', 0, 0); 16];
    let mut current = start;
    
    let mut calculation_loop = |op: char| {
        for i in 2..=9 {
            let next = if op == '/' { current / i } else { current * i };
            results[(i as usize - 2) + if op == '/' { 8 } else { 0 }] = (current, op, i, next);
            current = next;
        }        
    };
    
    calculation_loop('*');
    calculation_loop('/');
    results
}

fn main() {
    for (current, op, i, next) in calculate_result(START) {
        println!("{current:>10} {op} {i} = {next}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn start_value_equal_end_value() {
        let results = calculate_result(START);
        let (_, _, _, last_result) = results[15];
        assert_eq!(last_result, START, "The final result should be equal to the starting value.");
    }
}
