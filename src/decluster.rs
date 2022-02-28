use crate::point::Point;

pub fn decluster(points: &mut Vec<Point>, min_distance: f64) {
    for i in 0..points.len() {
        let Point {
            x: x1,
            y: y1,
            retain,
        } = points[i];
        if !retain {
            continue;
        };
        for j in i + 1..points.len() {
            let Point {
                x: x2,
                y: y2,
                retain,
            } = points[j];
            if !retain {
                continue;
            }
            let d = ((x1 - x2).powi(2) + (y1 - y2).powi(2)).sqrt();
            if d <= min_distance {
                points[j].retain = false;
            }
        }
    }
    points.retain(|p| p.retain);
}
