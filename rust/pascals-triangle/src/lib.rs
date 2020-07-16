pub struct PascalsTriangle {
    rows: Vec<Vec<u32>>
}


impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        let mut rows = Vec::new();
        rows.push(vec![1]);

        for i in 1..row_count {
            let mut row: Vec<u32> = Vec::new();


            for j in 0..i + 1 {
                let current_index_value = *rows.get((i - 1) as usize).unwrap().get(j as usize).unwrap_or(&0);
                let mut prev_index = j as i32 - 1;
                let prev_index_value = *rows.get((i - 1) as usize).unwrap().get((j as i32 - 1) as usize).unwrap_or(&0);
                row.push(current_index_value + prev_index_value)
            }

            rows.push(row);
        }

        PascalsTriangle {
            rows
        }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        self.rows.to_vec()
    }
}
