use cgmath::{EuclideanSpace, Zero};
use super::{Coordinate, Offset, Matrix, Color};

#[derive(Clone, Copy, PartialEq, Debug)]
pub(super) struct Piece {
    pub kind: Kind,
    pub position: Offset,
    pub rotation: Rotation,
}

impl Piece {
    const CELL_COUNT: usize = 4;

    pub fn moved_by(&self, offset: Offset) -> Self {
        Self {
            position: self.position + offset,
            ..*self
        }
    }

    pub fn cells(&self) -> Option<[Coordinate;Self::CELL_COUNT]> {
        let offsets = dbg!(self.kind.cells()
            .map(self.rotator()))
            .map(self.positioner());

        let mut coords = [Coordinate::origin();Self::CELL_COUNT];

        for (offset, coord_slot) in offsets.into_iter().zip(&mut coords) {
            let positive_offset = offset.cast::<usize>()?;
            let coord = Coordinate::from_vec(positive_offset);

            if Matrix::valid_coord(coord) {
                *coord_slot = coord;
            } else {
                return None;
            }
        }

        Some(coords)
    }

    fn rotator(&self) -> impl Fn(Offset) -> Offset + '_ {
        |cell| match self.kind {
            Kind::O => cell,
            _ => {
                let grid_offset = self.rotation.intrinsic_offset() * (self.kind.grid_size() - 1);
                cell * self.rotation + grid_offset
            }
        }
    }

    fn positioner(&self) -> impl Fn(Offset) -> Offset {
        let position = self.position;
        move |cell| cell + position
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Kind { O, I, T, L, J, S, Z }

impl Kind {
    pub const ALL: [Self;7] = [Self::O, Self::I, Self::T, Self::L, Self::J, Self::S, Self::Z];

    fn cells(&self) -> [Offset;Piece::CELL_COUNT] {
        match self {
            Self::O => &[( 1,1), ( 1,2), (2,1), (2,2)],
            Self::I => &[( 0,2), ( 1,2), (2,2), (3,2)],
            Self::T => &[( 0,1), ( 1,1), (2,1), (1,2)],
            Self::L => &[( 0,1), ( 1,1), (2,1), (2,2)],
            Self::J => &[( 0,2), ( 0,1), (1,1), (2,1)],
            Self::S => &[( 1,1), ( 1,1), (1,2), (2,2)],
            Self::Z => &[( 0,2), ( 1,2), (1,1), (2,1)],
        }.map(Offset::from)
    }

    fn grid_size(&self) -> isize {
        match self {
            Self::I => 4,
            _ => 3,
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::O => Color::Yellow,
            Self::I => Color::Cyan,
            Self::T => Color::Purple,
            Self::L => Color::Orange,
            Self::J => Color::Blue,
            Self::S => Color::Green,
            Self::Z => Color::Red,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Rotation { N, E, S, W }

impl Rotation {
    fn intrinsic_offset(&self) -> Offset {
        match self {
            Self::N => Offset::zero(),
            Self::E => Offset::new(0, 1),
            Self::S => Offset::new(1, 1),
            Self::W => Offset::new(1, 0),
        }
    }
}

impl std::ops::Mul<Rotation> for Offset {
    type Output = Self;

    fn mul(self, rotation: Rotation) -> Self::Output {
        match rotation {
            Rotation::N => self,
            Rotation::E => Self::new( self.y, -self.x),
            Rotation::S => Self::new(-self.x, -self.y),
            Rotation::W => Self::new(-self.y,  self.x),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn s_piece_positioning() {
        let z = Piece {
            kind: Kind::Z,
            position: Offset::new(5, 6),
            rotation: Rotation::W,
        };

        assert_eq!(
            z.cells(),
            Some([(5, 6), (5, 7), (6, 7), (6, 8)].map(Coordinate::from))
        );
    }
}
