#![allow(clippy::cast_sign_loss, clippy::cast_precision_loss)]

use itertools::Itertools;

use super::{
    coordinates::{CardinalDirection, PrincipalWinds},
    vecs::Vec2,
};

#[derive(Clone)]
pub struct Grid<T> {
    x_size: usize,
    y_size: usize,
    cells: Vec<T>,
}

impl<T> Grid<T> {
    pub(crate) fn new(x_size: usize, y_size: usize, cells: Vec<T>) -> Self {
        assert!(cells.len() == x_size * y_size);
        Self {
            x_size,
            y_size,
            cells,
        }
    }

    pub fn x_size(&self) -> usize {
        self.x_size
    }

    pub fn y_size(&self) -> usize {
        self.y_size
    }

    pub fn at(&self, v: Vec2) -> Option<&T> {
        if !self.is_valid_coord(&v) {
            return None;
        }

        let idx: usize = v.y as usize * self.x_size + v.x as usize;
        self.cells.get(idx)
    }

    pub fn set_at(&mut self, v: Vec2, val: T) {
        assert!(self.is_valid_coord(&v));

        let idx: usize = v.y as usize * self.x_size + v.x as usize;
        self.cells[idx] = val;
    }

    pub fn set_at_safe(&mut self, v: Vec2, val: T) {
        if !self.is_valid_coord(&v) {
            return;
        }

        self.set_at(v, val);
    }

    pub fn neighbors4(&self, v: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        CardinalDirection::ALL
            .map(move |dir| v.move_in_direction(dir))
            .into_iter()
            .filter(|coord| self.is_valid_coord(coord))
    }

    pub fn neighbors8(&self, v: Vec2) -> impl Iterator<Item = Vec2> + '_ {
        PrincipalWinds::ALL
            .map(move |dir| v.move_in_direction(dir))
            .into_iter()
            .filter(|coord| self.is_valid_coord(coord))
    }

    pub fn neighbor_cells4(&self, v: Vec2) -> impl Iterator<Item = (Vec2, &T)> + '_ {
        self.neighbors4(v)
            .map(|coord| (coord, self.at(coord).unwrap()))
    }

    pub fn neighbor_cells8(&self, v: Vec2) -> impl Iterator<Item = (Vec2, &T)> + '_ {
        self.neighbors8(v)
            .map(|coord| (coord, self.at(coord).unwrap()))
    }

    pub fn is_valid_coord(&self, v: &Vec2) -> bool {
        v.x >= 0 && v.x < self.x_size as i32 && v.y >= 0 && v.y < self.y_size as i32
    }

    pub fn iter_coords(&self) -> impl Iterator<Item = Vec2> + '_ {
        (0..self.y_size)
            .flat_map(move |y| (0..self.x_size).map(move |x| Vec2::new(x as i32, y as i32)))
    }

    pub fn iter_cells(&self) -> impl Iterator<Item = (Vec2, &T)> + '_ {
        self.iter_coords().map(|c| (c, self.at(c).unwrap()))
    }

    pub fn cells_matching<F>(&self, f: F) -> Vec<Vec2>
    where
        F: Fn(&T) -> bool,
    {
        self.iter_cells()
            .filter(|(_, cell)| f(cell))
            .map(|(coord, _)| coord)
            .collect_vec()
    }

    pub fn map_cells<X, F>(&self, f: F) -> Grid<X>
    where
        F: Fn(Vec2, &T) -> X,
    {
        Grid::new(
            self.x_size,
            self.y_size,
            self.iter_cells()
                .map(|(coord, value)| f(coord, value))
                .collect_vec(),
        )
    }

    pub fn raycast_coords(
        &self,
        from_coord: Vec2,
        direction: PrincipalWinds,
    ) -> impl Iterator<Item = Vec2> + '_ {
        let max_possible_size = self.x_size.max(self.y_size);

        (1..max_possible_size)
            .map(move |amount| from_coord.move_in_direction_by(direction, amount as u32))
            .take_while(|coord| self.is_valid_coord(coord))
    }

    pub fn raycast_coords_inclusive(
        &self,
        from_coord: Vec2,
        direction: PrincipalWinds,
    ) -> impl Iterator<Item = Vec2> + '_ {
        let max_possible_size = self.x_size.max(self.y_size);

        (0..max_possible_size)
            .map(move |amount| from_coord.move_in_direction_by(direction, amount as u32))
            .take_while(|coord| self.is_valid_coord(coord))
    }

    pub fn raycast_cells(
        &self,
        from_coord: Vec2,
        direction: PrincipalWinds,
    ) -> impl Iterator<Item = &T> + '_ {
        self.raycast_coords(from_coord, direction)
            .map(|coord| self.at(coord).unwrap())
    }

    pub fn raycast_cells_inclusive(
        &self,
        from_coord: Vec2,
        direction: PrincipalWinds,
    ) -> impl Iterator<Item = &T> + '_ {
        self.raycast_coords_inclusive(from_coord, direction)
            .map(|coord| self.at(coord).unwrap())
    }
}

impl<T> std::fmt::Debug for Grid<T>
where
    T: std::fmt::Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (coord, value) in self.iter_cells() {
            if coord.x == 0 {
                writeln!(f)?;
            }

            write!(f, "{value:?}")?;
        }

        writeln!(f)
    }
}
