#![cfg_attr(not(test), no_std)]
use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};

pub struct Plane<const N: usize> {
    board: [bool; N],
    next: [bool; N],
    width: usize,
    height: usize,
}

impl<const N: usize> Plane<N> {
    pub fn new(width: usize, height: usize) -> Option<Self> {
        if N < ((width + 1) * (height + 1)).into() {
            return None;
        }
        Some(Self {
            board: [false; N],
            next: [false; N],
            width,
            height,
        })
    }
    fn index(&self, x: usize, y: usize) -> usize {
        ((1 + y) * (self.width + 2) + 1 + x).into()
    }
    pub fn point(&self, x: usize, y: usize) -> bool {
        if self.width < x {
            return false;
        }
        if self.height < y {
            return false;
        }
        self.board[self.index(x, y)]
    }
    fn set(&mut self, x: usize, y: usize, value: bool) {
        if self.width < x {
            return;
        }
        if self.height < y {
            return;
        }
        self.board[self.index(x, y)] = value
    }
    pub fn randomize(&mut self, seed: u64) {
        let mut rng = SmallRng::seed_from_u64(seed);
        for x in 0..self.height {
            for y in 0..self.width {
                self.set(x, y, rng.gen::<bool>());
            }
        }
    }
    fn around_indices(&self, x: usize, y: usize) -> [usize; 8] {
        let index = self.index(x, y);
        let b1 = index - (self.width + 2);
        let a1 = index + (self.width + 2);
        [b1 - 1, b1, b1 + 1, index - 1, index + 1, a1 - 1, a1, a1 + 1]
    }
    fn is_live(&self, x: usize, y: usize) -> bool {
        let now = self.point(x, y);
        let mut live_count = if now {
            Around::Live(0)
        } else {
            Around::Dead(0)
        };
        let around_indices = self.around_indices(x, y);
        for i in 0..around_indices.len() {
            let index = around_indices[i];
            if self.board[index] {
                live_count = live_count.incr();
            }
        }
        match live_count {
            Around::Live(i) => match i {
                2 | 3 => true,
                _ => false,
            },
            Around::Dead(i) => match i {
                3 => true,
                _ => false,
            },
        }
    }
    pub fn tick(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let index = self.index(x, y);
                self.next[index] = self.is_live(x, y);
            }
        }
        self.board = self.next;
    }
}

enum Around {
    Live(usize),
    Dead(usize),
}

impl Around {
    fn incr(self) -> Self {
        match self {
            Around::Live(i) => Around::Live(i + 1),
            Around::Dead(i) => Around::Dead(i + 1),
        }
    }
}

impl<const N: usize> core::fmt::Debug for Plane<N> {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        for y in 0..self.height {
            write!(f, "\n")?;
            for x in 0..self.width {
                if self.point(x, y) {
                    write!(f, "o ")?;
                } else {
                    write!(f, "x ")?;
                }
            }
        }
        write!(f, "\n")
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn randomize_and_point() {
        let mut plane = crate::Plane::<256>::new(4, 4).unwrap();
        plane.randomize(1);
        println!("{:?}", plane);
        assert_eq!(plane.point(0, 0), true);
        assert_eq!(plane.point(1, 0), false);
        assert_eq!(plane.point(2, 0), false);
        assert_eq!(plane.point(3, 0), true);

        assert_eq!(plane.point(0, 1), false);
        assert_eq!(plane.point(1, 1), true);
        assert_eq!(plane.point(2, 1), true);
        assert_eq!(plane.point(3, 1), false);

        assert_eq!(plane.point(0, 2), false);
        assert_eq!(plane.point(1, 2), false);
        assert_eq!(plane.point(2, 2), true);
        assert_eq!(plane.point(3, 2), true);

        assert_eq!(plane.point(0, 3), false);
        assert_eq!(plane.point(1, 3), false);
        assert_eq!(plane.point(2, 3), true);
        assert_eq!(plane.point(3, 3), false);
    }
    #[test]
    fn around_indices() {
        let plane = crate::Plane::<256>::new(4, 4).unwrap();
        assert_eq!(plane.around_indices(0, 0), [0, 1, 2, 6, 8, 12, 13, 14]);
        assert_eq!(plane.around_indices(1, 0), [1, 2, 3, 7, 9, 13, 14, 15]);
    }
    #[test]
    fn tick_static_box() {
        let mut plane = crate::Plane::<256>::new(4, 4).unwrap();
        plane.set(0, 0, true);
        plane.set(0, 1, true);
        plane.set(1, 0, true);
        plane.set(1, 1, true);

        plane.tick();
        assert_eq!(plane.point(0, 0), true);
        assert_eq!(plane.point(0, 1), true);
        assert_eq!(plane.point(1, 0), true);
        assert_eq!(plane.point(1, 1), true);
    }
    #[test]
    fn tick_blinker() {
        let mut plane = crate::Plane::<256>::new(6, 6).unwrap();
        plane.set(0, 1, true);
        plane.set(1, 1, true);
        plane.set(2, 1, true);

        plane.tick();
        assert_eq!(plane.point(0, 0), false);
        assert_eq!(plane.point(0, 1), false);
        assert_eq!(plane.point(0, 2), false);

        assert_eq!(plane.point(1, 0), true);
        assert_eq!(plane.point(1, 1), true);
        assert_eq!(plane.point(1, 2), true);

        assert_eq!(plane.point(2, 0), false);
        assert_eq!(plane.point(2, 1), false);
        assert_eq!(plane.point(2, 2), false);

        plane.tick();

        assert_eq!(plane.point(0, 0), false);
        assert_eq!(plane.point(0, 1), true);
        assert_eq!(plane.point(0, 2), false);

        assert_eq!(plane.point(1, 0), false);
        assert_eq!(plane.point(1, 1), true);
        assert_eq!(plane.point(1, 2), false);

        assert_eq!(plane.point(2, 0), false);
        assert_eq!(plane.point(2, 1), true);
        assert_eq!(plane.point(2, 2), false);
    }
}
