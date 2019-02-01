use rustcv::core::{Mat, Point, Scalar};
use rustcv::imgproc::arrowed_line;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Path {
    history: VecDeque<Point>,
}

const HISTORY_SIZE: usize = 5;

impl Path {
    pub fn new() -> Path {
        Path {
            history: VecDeque::with_capacity(HISTORY_SIZE),
        }
    }

    pub fn push(&mut self, p: Point) {
        if self.history.len() >= HISTORY_SIZE {
            self.history.pop_front();
        }
        self.history.push_back(p);
    }

    pub fn draw_path(&self, mat: &mut Mat, color: Scalar, thickness: i32) {
        let mut iter = self.history.iter().peekable();
        loop {
            match (iter.next(), iter.peek()) {
                (Some(p1), Some(p2)) => arrowed_line(mat, p1.clone(), *p2.clone(), color, thickness),
                _                    => break,
            }
        }
    }

    pub fn predict(&self, x: i32) -> Option<Point> {
        if self.history.len() < 2 {
            return None
        }

        let mut tmp = self.history.clone();
        match (tmp.pop_back(), tmp.pop_back()) {
            (Some(p2), Some(p1)) => {
                let m = (p2.y - p1.y) / (p2.x - p1.x);
                let n = p1.y - p1.x * m;
                Some(Point { x, y: n + x * m })
            },
            _ => None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn path_push() {
        let points = (1..10).map(|i| Point{ x: i, y: i }).collect::<Vec<_>>();

        let mut path = Path::new();
        points.iter().for_each(|p| path.push(*p));


        assert_eq!(path.history.len(), HISTORY_SIZE);

        let expected_points = points[4..].iter();
        for (point, expected) in path.history.iter().zip(expected_points) {
            assert_eq_points(point, expected);
        }
    }




    #[test]
    fn predict() {
        let mut path = Path::new();
        path.push(Point{x: 1, y: 1});
        assert_eq_points_opt(&None, &path.predict(9));

        path.push(Point{x: 7, y: 7});
        assert_eq_points_opt(&Some(Point{x: 9, y: 9}), &path.predict(9));

        path.push(Point{x: 3, y: 11});
        assert_eq_points_opt(&Some(Point{x: 9, y: 5}), &path.predict(9));

        path.push(Point{x: 6, y: 14});
        assert_eq_points_opt(&Some(Point{x: 5, y: 13}), &path.predict(5));
    }


    fn assert_eq_points(p1: &Point, p2: &Point) {
        if p1.x != p2.x || p1.y != p2.y {
            panic!("points don't match: p1: `{:?}`, p2: `{:?}`", p1, p2);
        };
    }

    fn assert_eq_points_opt(p1: &Option<Point>, p2: &Option<Point>) {
        match (p1, p2) {
            (Some(p1), Some(p2)) => assert_eq_points(p1, p2),
            (None, None)         => (),
            _                    => panic!("results don't match: left: `{:?}`, right: `{:?}`", p1, p2),
        };
    }
}
