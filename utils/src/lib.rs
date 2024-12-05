#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<T>,
    pub rows: usize,
    pub columns: usize,
}

impl<T> Grid<T>
where
    T: Copy + Clone,
{
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if col < self.columns && row < self.rows {
            self.grid.get(row * self.columns + col).copied()
        } else {
            None
        }
    }

    #[allow(unused)]
    fn row(&self, row: usize) -> Option<Vec<T>> {
        if row < self.rows {
            let start = row * self.columns;
            let end = start + self.columns;
            Some(self.grid[start..end].to_vec())
        } else {
            None
        }
    }

    #[allow(unused)]
    fn col(&self, col: usize) -> Option<Vec<T>> {
        if col < self.columns {
            Some(
                (0..self.rows)
                    .map(|row| self.get(row, col).expect("shouldn't go outbound"))
                    .collect(),
            )
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}
