//! An iterative algorithm that searches for a set of points where each point is separated from all others by at least the minimum specified distance.
use crate::point::Point;

/**
 The decluster algorithm cycles through the points set calculating the distance between each pair of points. If that distance is greater than the `min_distance` then the point
 is marked for removal by the vec's `retain` function.

Note that once a point is marked for removal there is no need to process it further.

# Arguments
    * points - the vec of Point structures
    * min_distance - the minimum distance allowed between points
*/
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
