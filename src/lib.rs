#![warn(rust_2018_idioms)]

mod heap;

#[derive(Clone)]
pub struct PointCloud<'a, T> {
    // stores all points
    points: Vec<&'a T>,

    // return the distance given 2 ponints
    dist_fn: fn(&T, &T) -> f64,
}

impl<'a, T> PointCloud<'a, T> {
    pub fn new(dist_fn: fn(&T, &T) -> f64) -> Self {
        Self {
            points: Vec::new(),
            dist_fn: dist_fn,
        }
    }

    pub fn add_point(&mut self, p: &'a T) {
        self.points.push(p)
    }

    pub fn get_nearest_n(&self, p: &T, n: usize) -> Vec<(f64, &T)> {
        let mut h = heap::Heap::new(n);
        for i in 0..self.points.len() {
            let d = (self.dist_fn)(self.points[i], p);
            h.insert(d, &self.points[i]);
        }
        let mut knn = Vec::new();
        for i in h.get_elements() {
            knn.push((i.0, *i.1));
        }
        knn
    }

    pub fn len(&self) -> usize {
        self.points.len()
    }
}
