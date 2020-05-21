use crate::heap;

#[derive(Clone, Debug)]
pub struct Point {
    coords: [f64; 2],
}

impl Point {
    pub fn new(c: [f64; 2]) -> Point {
        Point { coords: c }
    }
}

pub fn dist_squared(p: &Point, q: &Point) -> f64 {
    let mut sosq = 0.0;
    for i in 0..p.coords.len() {
        sosq += (p.coords[i] - q.coords[i]).powf(2.0)
    }
    return sosq.sqrt();
}

#[derive(Clone, Debug)]
pub struct PointCloud {
    points: Vec<Point>,
}

impl PointCloud {
    pub fn new() -> PointCloud {
        PointCloud { points: Vec::new() }
    }

    pub fn add_point(&mut self, p: Point) {
        self.points.push(p)
    }

    pub fn get_nearest_n(&self, p: &Point, n: usize) -> (Vec<f64>, Point) {
        let mut h = heap::Heap::new(n);
        let min_point = Point { coords: [0.0, 0.0] };
        for i in 0..self.points.len() {
            println!("{}", i);
            let d = dist_squared(&self.points[i], p);
            h.insert(d);
        }
        (h.get_elements(), min_point)
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
