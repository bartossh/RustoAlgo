pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points: Vec<(usize, usize)> = Vec::new();


    for (idx, x) in input.iter().enumerate() {
        for (idy, y) in x.iter().enumerate() {
            
            let mut greater_or_equal_row = true;
            'check_g: for v in x.iter() {
                if *v > *y {
                    greater_or_equal_row = false;
                    break 'check_g; 
                }
            }

            let mut less_or_equal_col = true;
            'check_l: for i in 0..input.len() {
                if input[i][idy] < *y {
                    less_or_equal_col = false;
                    break 'check_l;
                }
            }
            if greater_or_equal_row && less_or_equal_col {
                saddle_points.push((idx, idy));
            }
        }
    }
    saddle_points
}
