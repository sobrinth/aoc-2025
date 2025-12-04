use std::ops::Index;

#[derive(Debug, Clone)]
pub struct Grid<T> {
    pub width: isize,
    pub height: isize,
    pub content: Vec<T>,
}

impl<T> Index<(isize, isize)> for Grid<T> {
    type Output = T;

    fn index(&self, (x, y): (isize, isize)) -> &Self::Output {
        &self.content[(y * self.width + x) as usize]
    }
}
