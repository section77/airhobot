//#![feature(test)]
extern crate airhobot;
extern crate test;

use airhobot::*;

use test::{black_box, Bencher};

#[bench]
fn push_to_history(b: &mut Bencher) {
    let mut path = Path::new();
    b.iter(|| {
        black_box(path.push(Point { x: 1, y: 1 }));
    });
}

#[bench]
fn predict(b: &mut Bencher) {
    let mut path = Path::new();
    path.push(Point { x: 2, y: 2 });
    path.push(Point { x: 3, y: 3 });

    b.iter(|| {
        black_box(path.predict(5));
    });
}
