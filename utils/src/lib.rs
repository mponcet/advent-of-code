use std::ops::Range;

#[derive(Debug)]
pub struct Grid<T> {
    pub grid: Vec<T>,
    pub rows: i32,
    pub columns: i32,
}

pub enum DiagonalDirection {
    TopLeft(usize),
    TopRight(usize),
    BottomLeft(usize),
    BottomRight(usize),
}

impl Grid<char> {
    pub fn parse(input: &str) -> Self {
        let mut columns = 0;
        let grid = input
            .lines()
            .flat_map(|line| {
                columns = line.len() as i32;
                line.chars()
            })
            .collect::<Vec<_>>();
        let rows = grid.len() as i32 / columns;

        Grid {
            grid,
            rows,
            columns,
        }
    }
}

impl Grid<u8> {
    pub fn parse(input: &str) -> Self {
        let mut columns = 0;
        let grid = input
            .lines()
            .flat_map(|line| {
                columns = line.len() as i32;
                line.bytes() //.map(|b| b - b'0')
            })
            .collect::<Vec<_>>();
        let rows = grid.len() as i32 / columns;

        Grid {
            grid,
            rows,
            columns,
        }
    }
}

impl<T: Copy + Clone + std::fmt::Display> std::fmt::Display for Grid<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in 0..self.rows {
            for col in 0..self.columns {
                write!(f, "{}", self.get(row, col).unwrap())?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

impl<T> Grid<T>
where
    T: Copy + Clone,
{
    pub fn get(&self, row: i32, col: i32) -> Option<T> {
        if row >= 0 && col >= 0 && col < self.columns && row < self.rows {
            self.grid
                .get(row as usize * self.columns as usize + col as usize)
                .copied()
        } else {
            None
        }
    }

    pub fn set(&mut self, row: i32, col: i32, value: T) -> Result<(), &'static str> {
        if row >= 0 && col >= 0 && col < self.columns && row < self.rows {
            self.grid[row as usize * self.columns as usize + col as usize] = value;
            Ok(())
        } else {
            Err("out of bounds")
        }
    }

    pub fn row(&self, row: i32) -> Option<Vec<T>> {
        if row >= 0 && row < self.rows {
            let start = (row * self.columns) as usize;
            let end = start + self.columns as usize;
            Some(self.grid[start..end].to_vec())
        } else {
            None
        }
    }

    pub fn row_slice(&self, row: i32, range: Range<i32>) -> Option<Vec<T>> {
        if row >= 0 && row < self.rows && range.start >= 0 && range.end <= self.columns {
            let start = (row * self.columns) as usize;
            let end = start + self.columns as usize;
            let slice = &self.grid[start..end];

            let range = range.start as usize..range.end as usize;
            Some(slice[range].to_vec())
        } else {
            None
        }
    }

    pub fn col(&self, col: i32) -> Option<Vec<T>> {
        if col >= 0 && col < self.columns {
            Some(
                (0..self.rows)
                    .map(|row| self.get(row, col).expect("shouldn't go outbound"))
                    .collect(),
            )
        } else {
            None
        }
    }

    pub fn col_slice(&self, col: i32, range: Range<i32>) -> Option<Vec<T>> {
        if col >= 0 && col < self.columns && range.start >= 0 && range.end <= self.rows {
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
        row: i32,
        col: i32,
        direction: DiagonalDirection,
    ) -> Option<Vec<T>> {
        // lazy bound checking: rely on self.get to check whether we are out of bounds
        match direction {
            DiagonalDirection::TopLeft(len) => {
                let len = len as i32;
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
                let len = len as i32;
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
                let len = len as i32;
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
                let len = len as i32;
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

    pub fn position_iter(&self) -> impl Iterator<Item = (i32, i32)> + '_ {
        (0..self.rows).flat_map(|row| (0..self.columns).map(move |col| (row, col)))
    }

    pub fn neighbors_cross(&self, row: i32, col: i32) -> impl Iterator<Item = (i32, i32)> + '_ {
        [
            (row + 1, col),
            (row, col - 1),
            (row - 1, col),
            (row, col + 1),
        ]
        .into_iter()
        .filter(|&(row, col)| row >= 0 && row < self.rows && col >= 0 && col < self.columns)
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
