use std::ops::Range;

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<T>,
    pub rows: usize,
    pub columns: usize,
}

pub enum DiagonalDirection {
    TopLeft(usize),
    TopRight(usize),
    BottomLeft(usize),
    BottomRight(usize),
}

impl<T> Grid<T>
where
    T: Copy + Clone,
{
    /// Retrieves an element from the grid at the specified row and column indices.
    ///
    /// # Arguments
    ///
    /// * `row` - The row index of the element to retrieve, which must be less than `self.rows`.
    /// * `col` - The column index of the element to retrieve, which must be less than `self.columns`.
    ///
    /// # Returns
    ///
    /// Returns `Some(T)` containing the element if the specified indices are within bounds,
    /// otherwise returns `None` if the indices are out of bounds.
    pub fn get(&self, row: usize, col: usize) -> Option<T> {
        if col < self.columns && row < self.rows {
            self.grid.get(row * self.columns + col).copied()
        } else {
            None
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        if col < self.columns && row < self.rows {
            self.grid[row * self.columns + col] = value;
        }
    }

    pub fn row(&self, row: usize) -> Option<Vec<T>> {
        if row < self.rows {
            let start = row * self.columns;
            let end = start + self.columns;
            Some(self.grid[start..end].to_vec())
        } else {
            None
        }
    }

    pub fn row_slice(&self, row: usize, range: Range<usize>) -> Option<Vec<T>> {
        if row < self.rows && range.end <= self.columns {
            let start = row * self.columns;
            let end = start + self.columns;
            let slice = &self.grid[start..end];

            Some(slice[range].to_vec())
        } else {
            None
        }
    }

    pub fn col(&self, col: usize) -> Option<Vec<T>> {
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

    pub fn col_slice(&self, col: usize, range: Range<usize>) -> Option<Vec<T>> {
        if col < self.columns && range.end <= self.rows {
            Some(
                (range)
                    .map(|row| self.get(row, col).expect("shouldn't go outbound"))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn diagonal_from(
        &self,
        row: usize,
        col: usize,
        direction: DiagonalDirection,
    ) -> Option<Vec<T>> {
        // lazy bound checking: rely on self.get to check whether we are out of bounds
        match direction {
            DiagonalDirection::TopLeft(len) => {
                if len == 0 || row + 1 < len || col + 1 < len {
                    return None;
                }
                let (row_range, col_range) = (
                    (row + 1 - len..row + 1).rev(),
                    (col + 1 - len..col + 1).rev(),
                );
                Some(
                    (row_range)
                        .zip(col_range)
                        .map(|(row, col)| self.get(row, col))
                        .collect::<Option<Vec<T>>>()?,
                )
            }
            DiagonalDirection::TopRight(len) => {
                if len == 0 || row + 1 < len {
                    return None;
                }
                let (row_range, col_range) = ((row + 1 - len..row + 1).rev(), col..col + len);
                Some(
                    (row_range)
                        .zip(col_range)
                        .map(|(row, col)| self.get(row, col))
                        .collect::<Option<Vec<T>>>()?,
                )
            }
            DiagonalDirection::BottomLeft(len) => {
                if len == 0 || col + 1 < len {
                    return None;
                }
                let (row_range, col_range) = (row..row + len, (col + 1 - len..col + 1).rev());
                Some(
                    (row_range)
                        .zip(col_range)
                        .map(|(row, col)| self.get(row, col))
                        .collect::<Option<Vec<T>>>()?,
                )
            }
            DiagonalDirection::BottomRight(len) => {
                let (row_range, col_range) = (row..row + len, col..col + len);
                Some(
                    (row_range)
                        .zip(col_range)
                        .map(|(row, col)| self.get(row, col))
                        .collect::<Option<Vec<T>>>()?,
                )
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{DiagonalDirection, Grid};

    #[test]
    fn test_get_within_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            columns: 3,
        };
        assert_eq!(grid.get(1, 1), Some(5));
    }

    #[test]
    fn test_get_out_of_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4],
            rows: 2,
            columns: 2,
        };
        assert_eq!(grid.get(2, 2), None);
    }

    #[test]
    fn test_row_within_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            columns: 3,
        };
        assert_eq!(grid.row(1), Some(vec![4, 5, 6]));
    }

    #[test]
    fn test_row_out_of_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3],
            rows: 1,
            columns: 3,
        };
        assert_eq!(grid.row(1), None);
    }

    #[test]
    fn test_row_slice_within_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            columns: 3,
        };
        assert_eq!(grid.row_slice(1, 1..3), Some(vec![5, 6]));
    }

    #[test]
    fn test_row_slice_out_of_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            columns: 3,
        };
        assert_eq!(grid.row_slice(1, 3..4), None);
    }

    #[test]
    fn test_col_within_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 2,
            columns: 3,
        };
        assert_eq!(grid.col(2), Some(vec![3, 6]));
    }

    #[test]
    fn test_col_out_of_bounds() {
        let grid = Grid {
            grid: vec![1, 2],
            rows: 1,
            columns: 2,
        };
        assert_eq!(grid.col(2), None);
    }

    #[test]
    fn test_col_slice_within_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6],
            rows: 3,
            columns: 2,
        };
        assert_eq!(grid.col_slice(1, 0..2), Some(vec![2, 4]));
    }

    #[test]
    fn test_col_slice_out_of_bounds() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4],
            rows: 2,
            columns: 2,
        };
        assert_eq!(grid.col_slice(1, 0..3), None);
    }

    #[test]
    fn test_diagonal_from() {
        let grid = Grid {
            grid: vec![1, 2, 3, 4, 5, 6, 7, 8, 9],
            rows: 3,
            columns: 3,
        };
        // 123
        // 456
        // 789

        assert_eq!(
            grid.diagonal_from(0, 0, DiagonalDirection::BottomRight(3)),
            Some(vec![1, 5, 9])
        );
        assert_eq!(
            grid.diagonal_from(2, 0, DiagonalDirection::TopRight(3)),
            Some(vec![7, 5, 3])
        );
        assert_eq!(
            grid.diagonal_from(0, 2, DiagonalDirection::BottomLeft(3)),
            Some(vec![3, 5, 7])
        );
        assert_eq!(
            grid.diagonal_from(2, 2, DiagonalDirection::TopLeft(3)),
            Some(vec![9, 5, 1])
        );

        assert_eq!(
            grid.diagonal_from(0, 0, DiagonalDirection::BottomRight(4)),
            None
        ); // Out of bounds
    }
}
