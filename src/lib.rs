#![warn(rust_2018_idioms)]

mod heap;

#[derive(Clone)]
pub struct PointCloud<T> {
    // stores all points
    points: Vec<T>,

    // return the distance given 2 ponints
    dist_fn: fn(&T, &T) -> f64,
}

impl<T> PointCloud<T> {
    pub fn new(dist_fn: fn(&T, &T) -> f64) -> PointCloud<T> {
        PointCloud {
            points: Vec::new(),
            dist_fn: dist_fn,
        }
    }

    pub fn add_point(&mut self, p: T) {
        self.points.push(p)
    }

    pub fn get_nearest_n(&self, p: &T, n: usize) -> Vec<f64> {
        let mut h = heap::Heap::new(n);
        for i in 0..self.points.len() {
            let d = (self.dist_fn)(&self.points[i], p);
            h.insert(d);
        }
        h.get_elements()
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
