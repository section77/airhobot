use cv::*;

#[derive(Debug)]
pub struct Puck {
    pub center: Point,
}

impl Puck {

    pub fn new(center: Point) -> Puck {
        Puck { center }
    }

    // pub fn find(mask: &Mat<MASKED>) -> Option<Puck> {
    //     let moments = moments(&mask, true);
    //     if moments.m00 > 0.0 {
    //         let center = Point {
    //             x: (moments.m10 / moments.m00) as i32,
    //             y: (moments.m01 / moments.m00) as i32,
    //         };
    //         Some(Puck { center })
    //     } else {
    //         None
    //     }
    // }

    // pub fn center_point(&self) -> core::Point {
    //     core::Point { x: self.center.x, y: self.center.y }
    // }


    // pub fn contours(&self, hsv: &Mat) -> Vec<Contour> {
    //     find_contours(&Self::mask(hsv), RetrievalMode::CComp, ContourApproximationMode::Simple)
    // }


}
