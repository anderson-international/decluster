use rand::Rng;

pub struct Point {
    pub x: f64,
    pub y: f64,
    pub retain: bool,
}

impl Point {
    pub fn rnd_vec(window_size: [f64; 2], count: usize) -> Vec<Point> {
        (0..count).map(|_| Point::rnd(window_size)).collect()
    }

    pub fn rnd(window_size: [f64; 2]) -> Point {
        let mut rng = rand::thread_rng();
        Point {
            retain: true,
            x: rng.gen_range(0..window_size[0] as i32) as f64,
            y: rng.gen_range(0..window_size[1] as i32) as f64,
        }
    }
}
