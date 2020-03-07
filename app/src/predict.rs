use xstd::prelude::*;

pub fn predict<T>(frame: &cv::Mat<T>, from: &cv::Point, to: &cv::Point) -> Vec<cv::Point> {
    // FIXME
    let y = frame.n_rows() - 10;

    if from.x() == to.x() {
        return vec![*to, cv::Point::new(to.x(), y)];
    }

    let m = (to.y() - from.y()) as f32 / (to.x() - from.x()) as f32;
    let x = ((y - to.y()) as f32 / m + to.x() as f32) as i32;
    let p = cv::Point::new(x, y);

    if x >= 0 && x <= frame.n_cols() {
        vec![*to, p]
    } else {
        let bounce_x = ifte!(x <= 0, 0, frame.n_cols());
        let bounce_y = ((bounce_x - to.x()) as f32 * m + to.y() as f32) as i32;
        let b = cv::Point::new(bounce_x, bounce_y);
        let p_ = cv::Point::new(((y - bounce_y) as f32 / (-m) + bounce_x as f32) as i32, y);
        let mut v = vec![*to, b, p_];
        v.append(&mut predict(frame, &b, &p_));
        v
    }
}

// pub fn predict(state: &State, cfg: &Config, from: &Pos, to: &Pos, y: i32) -> (Pos, Option<Pos>) {
//     // straight impact
//     if from.x == to.x {
//         return (Pos::new(to.x, y), None);
//     }

//     let m = slope(from, to);
//     let x = ((y - to.y) as f32 / m + to.x as f32) as i32;

//     let pos = dbg!(Pos::new(x, y));
//     (pos, None)

//     // if let Some((l_start, l_end)) = dbg!(puck_bounces_side(state, &pos)) {
//     //     let m1 = slope(from, to);
//     //     let m2 = slope(&l_start, &l_end);

//     //     let b1 = from.y as f32 - from.x as f32 * m1;
//     //     let b2 = l_start.y as f32 - l_start.x as f32 * m2;

//     //     let x = (b2 - b1) / (m1 - m2);
//     //     let y_ = (m1 * x + b1) as i32;
//     //     let bp = Pos::new(x as i32, y_ as i32);
//     //     let pos = Pos::new(((y - bp.y) as f32 / (-m) + bp.x as f32) as i32, y);
//     //     (pos, Some(bp))
//     // } else {
//     //     (pos, None)
//     // }
// }

// fn slope(from: &cv::Point, to: &cv::Point) -> f32 {
//     (to.y() - from.y()) as f32 / (to.x() - from.x()) as f32
// }

// fn puck_bounces_side(state: &State, pos: &Pos) -> Option<(Pos, Pos)> {
//     let roi = &state.cfg.roi;
//     if pos.x < roi[3].x() {
//         Some((Pos::from(&roi[0]), Pos::from(&roi[3])))
//     } else if pos.x > roi[2].x() {
//         Some((Pos::from(&roi[1]), Pos::from(&roi[2])))
//     } else {
//         None
//     }
// }
