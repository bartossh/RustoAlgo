pub struct PascalsTriangle(Vec<Vec<u32>>);

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut triangle: Vec<Vec<u32>> = Vec::with_capacity(row_count as usize);
        for row in 0..row_count {
            let mut row_vec = Vec::with_capacity((row + 1) as usize);
            for col in 0..row + 1 {
                if col == 0 || col == row {
                    row_vec.push(1);
                } else {
                    row_vec.push(
                        triangle[(row - 1) as usize][(col - 1) as usize]
                            + triangle[(row - 1) as usize][col as usize],
                    );
                }
            }
            triangle.push(row_vec);
        }
        Self(triangle)
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        return self.0.clone();
    }
}
