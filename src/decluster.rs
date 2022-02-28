//! An iterative algorithm that replaces clustered points with random points until a maximally distributed set is acheived.
use crate::point::Point;

/**
 * The decluster algorithm cycles through the points set calculating the distnace between each pair of points. If that distance is greater than `max_distance`
the point is marked for removal by the vec's `retain` function.

Note that one a point is marked for removal there is no need to process it further.

# Arguments
    * points - the vec of Point structures
    * min_distance - the minimum allowed separtion distance between points
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
