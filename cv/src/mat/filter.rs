use crate::{prelude::*, Mat};
use opencv::prelude::MatTrait;

pub trait Filter {
    fn blur(&mut self, ksize: i32);
    fn erode(&mut self, ksize: i32);
    fn dilate(&mut self, ksize: i32);
}

impl<T> Filter for Mat<T> {
    fn blur(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();
        let kernel = opencv::core::Size::new(ksize, ksize);
        let anchor = opencv::core::Point::new(-1, -1);
        opencv::imgproc::blur(&src, &mut self.inner, kernel, anchor, 4).expect("blur");
    }

    fn erode(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();

        let kernel = opencv::imgproc::get_structuring_element(
            opencv::imgproc::MORPH_ELLIPSE,
            opencv::core::Size::new(ksize, ksize),
            opencv::core::Point::new(-1, -1),
        )
        .unwrap();
        let anchor = opencv::core::Point::new(-1, -1);
        let iterations = 1;
        // https://docs.opencv.org/4.2.0/d2/de8/group__core__array.html#ga209f2f4869e304c82d07739337eae7c5
        let border_type = opencv::core::BORDER_CONSTANT;
        let border_value = opencv::imgproc::morphology_default_border_value().expect("morphology_default_border_value");
        opencv::imgproc::erode(
            &src,
            &mut self.inner,
            &kernel,
            anchor,
            iterations,
            border_type,
            border_value,
        )
        .expect("erode");
    }

    fn dilate(&mut self, ksize: i32) {
        if ksize == 0 {
            return;
        }

        let src = self.inner.clone().unwrap();

        let kernel = opencv::imgproc::get_structuring_element(
            opencv::imgproc::MORPH_ELLIPSE,
            opencv::core::Size::new(ksize, ksize),
            opencv::core::Point::new(-1, -1),
        )
        .unwrap();
        let anchor = opencv::core::Point::new(-1, -1);
        let iterations = 1;
        // https://docs.opencv.org/4.2.0/d2/de8/group__core__array.html#ga209f2f4869e304c82d07739337eae7c5
        let border_type = opencv::core::BORDER_CONSTANT;
        let border_value = opencv::imgproc::morphology_default_border_value().expect("morphology_default_border_value");
        opencv::imgproc::dilate(
            &src,
            &mut self.inner,
            &kernel,
            anchor,
            iterations,
            border_type,
            border_value,
        )
        .expect("dilate");
    }
}
