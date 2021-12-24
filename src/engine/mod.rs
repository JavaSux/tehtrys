use std::ops::{Index, IndexMut};
use cgmath::EuclideanSpace;
use rand::{prelude::{SliceRandom, ThreadRng}, thread_rng};
use self::{piece::{Piece, Kind as PieceKind}, geometry::GridIncrement};

mod piece;
mod geometry;

type Coordinate = cgmath::Point2<usize>;
type Offset = cgmath::Vector2<isize>;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum MoveKind { Left, Right }

impl MoveKind {
    fn offset(&self) -> Offset {
        match self {
            MoveKind::Left => Offset::new(-1, 0),
            MoveKind::Right => Offset::new(1, 0),
        }
    }
}

pub struct Engine {
    matrix: Matrix,
    bag: Vec<PieceKind>,
    rng: ThreadRng,
    cursor: Option<Piece>,
}

impl Engine {
    pub fn new() -> Self {
        Engine {
            matrix: Matrix::blank(),
            bag: Vec::new(),
            rng: thread_rng(),
            cursor: None,
        }
    }

    pub fn with_matrix(matrix: Matrix) -> Self {
        Self {
            matrix,
            ..Self::new()
        }
    }

    fn refill_bag(&mut self) {
        debug_assert!(self.bag.is_empty());
        self.bag.extend_from_slice(PieceKind::ALL.as_slice());
        self.bag.shuffle(&mut self.rng);
    }

    fn place_cursor(&mut self) {
        let cursor = self.cursor.take().expect("Called place_cursor without a cursor");

        debug_assert!(
            self.matrix.is_placeable(&cursor),
            "Tried to place cursor in an unplaceable location: {:?}",
            cursor
        );

        let color = cursor.kind.color();
        for coord in cursor.cells().unwrap() {
            self.matrix[coord] = Some(color);
        }
    }

    fn move_cursor(&mut self, kind: MoveKind) -> Result<(), ()> {
        let Some(cursor) = self.cursor.as_mut() else {
            return Ok(());
        };

        let new = cursor.moved_by(kind.offset());

        if self.matrix.is_clipping(&new) {
            return Err(());
        }

        Ok(self.cursor = Some(new))
    }

    fn tick_down(&mut self) {
        self.cursor = Some(self.ticked_down_cursor().unwrap());
    }

    pub fn cursor_has_hit_bottom(&self) -> bool {
        self.cursor.is_some() &&
        self.ticked_down_cursor().is_none()
    }

    fn ticked_down_cursor(&self) -> Option<Piece> {
        let Some(cursor) = self.cursor else { return None; };
        let new = cursor.moved_by(Offset::new(0, -1));
        (!self.matrix.is_clipping(&new)).then_some(new)
    }

    fn hard_drop(&mut self) {
        while let Some(new) = self.ticked_down_cursor() {
            self.cursor = Some(new);
        }
        self.place_cursor();
    }

    pub fn cells(&self) -> CellIter<'_> {
        CellIter {
            position: Coordinate::origin(),
            cells: self.matrix.0.iter(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Color { Yellow, Cyan, Purple, Orange, Blue, Green, Red }

pub struct Matrix([Option<Color>;Self::SIZE]);

impl Matrix {
    pub const WIDTH: usize = 10;
    pub const HEIGHT: usize = 20;
    const SIZE: usize = Self::WIDTH * Self::HEIGHT;

    fn on_matrix(coord: Coordinate) -> bool {
        Self::valid_coord(coord) && coord.y < Self::HEIGHT
    }

    fn valid_coord(coord: Coordinate) -> bool {
        coord.x < Self::WIDTH
    }

    fn indexing(Coordinate { x, y }: Coordinate) -> usize {
        y * Self::WIDTH + x
    }

    pub fn blank() -> Self {
        Self([None; Self::SIZE])
    }

    fn is_clipping(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return true; };
        cells.into_iter().any(|coord|
            !Matrix::on_matrix(coord) ||
            self[coord].is_some()
        )
    }

    fn is_placeable(&self, piece: &Piece) -> bool {
        let Some(cells) = piece.cells() else { return false; };
        cells.into_iter().all(|coord|
            Matrix::on_matrix(coord) &&
            self[coord].is_none()
        )
    }
}

impl Index<Coordinate> for Matrix {
    type Output = Option<Color>;

    fn index(&self, coord: Coordinate) -> &Self::Output {
        assert!(Self::on_matrix(coord));
        &self.0[Self::indexing(coord)]
    }
}

impl IndexMut<Coordinate> for Matrix {
    fn index_mut(&mut self, coord: Coordinate) -> &mut Self::Output {
        assert!(Self::on_matrix(coord));
        &mut self.0[Self::indexing(coord)]
    }
}

pub struct CellIter<'matrix> {
    position: Coordinate,
    cells: ::std::slice::Iter<'matrix, Option<Color>>,
}

impl<'matrix> Iterator for CellIter<'matrix> {
    type Item = (Coordinate, &'matrix Option<Color>) ;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(cell) = self.cells.next() else {
            return None;
        };

        let coord = self.position;
        self.position.grid_inc();

        return Some((coord, cell));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cell_iter() {
        let mut matrix = Matrix::blank();
        matrix[Coordinate::new(2, 0)] = Some(Color::Blue);
        matrix[Coordinate::new(3, 1)] = Some(Color::Green);

        let mut iter = CellIter {
            position: Coordinate::origin(),
            cells: matrix.0.iter(),
        };

        let first_five = (&mut iter).take(5).collect::<Vec<_>>();
        assert_eq!(
            first_five,
            [
                (Coordinate::new(0, 0), &None),
                (Coordinate::new(1, 0), &None),
                (Coordinate::new(2, 0), &Some(Color::Blue)),
                (Coordinate::new(3, 0), &None),
                (Coordinate::new(4, 0), &None),
            ],
        );

        let other_item = (&mut iter).skip(8).next();
        assert_eq!(
            other_item,
            Some((Coordinate::new(3, 1), &Some(Color::Green))),
        );

        assert!(iter.all(|(_, contents)| contents.is_none()));
    }
}
