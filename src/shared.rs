use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub content: Vec<T>,
}

impl<T: PartialEq> Grid<T> {
    pub fn count_all_neighbours(&self, x: isize, y: isize, neighbour_kind: T) -> usize {
        let mut count = 0;

        for dx in [-1, 0, 1].iter() {
            for dy in [-1, 0, 1].iter() {
                let x_idx = x + dx;
                let y_idx = y + dy;
                if (*dx == 0 && *dy == 0)
                    || y_idx < 0
                    || y_idx >= self.height
                    || x_idx < 0
                    || x_idx >= self.width
                {
                    continue;
                }
                if self[(x_idx, y_idx)] == neighbour_kind {
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
