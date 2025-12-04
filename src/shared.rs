use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub content: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    pub fn count_neighbours_eight(&self, x: isize, y: isize, neighbour_kind: T) -> usize {
        let mut count = 0;
        let valid_x_offsets = if x == 0 {
            [0, 1].as_slice()
        } else if x == self.width - 1 {
            [-1, 0].as_slice()
        } else {
            [-1, 0, 1].as_slice()
        };
        let valid_y_offsets = if y == 0 {
            [0, 1].as_slice()
        } else if y == self.height - 1 {
            [-1, 0].as_slice()
        } else {
            [-1, 0, 1].as_slice()
        };

        for dx in valid_x_offsets {
            for dy in valid_y_offsets {
                if *dx == 0 && *dy == 0 {
                    continue;
                }
                if self[(x + dx, y + dy)] == neighbour_kind {
                    count += 1;
                }
            }
        }
        count
    }
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.content[(y * self.width + x) as usize]
    }
}
