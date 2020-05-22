#![warn(rust_2018_idioms)]
// #![warn(missing_docs)]

mod heap;

#[derive(Clone)]
pub struct PointCloud<'a, T> {
    // stores all points
    points: Vec<&'a T>,

    // return the distance given 2 ponints
    dist_fn: fn(&T, &T) -> f64,
}

impl<'a, T> PointCloud<'a, T> {
    /// Constructs a new `PointCloud<T>`.
    ///
    /// It accepts a function to compute distance between any 2 objects.
    ///
    /// # Examples
    /// ```
    /// extern crate pointcloud;
    /// use pointcloud::PointCloud;
    /// let manhattan_dist = |p: &[f64;2], q: &[f64;2]| -> f64 {(p[0] - q[0]).abs() + (p[1] - q[1]).abs()};
    /// let pc = PointCloud::new(manhattan_dist);
    /// ```
    pub fn new(dist_fn: fn(&T, &T) -> f64) -> Self {
        Self {
            points: Vec::new(),
            dist_fn: dist_fn,
        }
    }

    /// Adds a point to the PointCloud.
    ///
    /// It accepts a reference to an object.
    ///
    /// # Examples
    /// ```
    /// extern crate pointcloud;
    /// use pointcloud::PointCloud;
    /// let dummy_dist = |p: &[f64;2], q: &[f64;2]| -> f64 {0.0};
    /// let mut pc = PointCloud::new(dummy_dist);
    /// let p = [1.89, 5.63];
    /// pc.add_point(&p);
    /// ```
    pub fn add_point(&mut self, p: &'a T) {
        self.points.push(p)
    }

    /// Gets the k nearest neighbours of an object.
    ///
    /// It accepts a reference to the object and how many neighbours to return.
    ///
    /// # Example
    /// ```
    /// extern crate pointcloud;
    /// use pointcloud::PointCloud;
    /// let manhattan_dist = |p: &[f64;2], q: &[f64;2]| -> f64 {(p[0] - q[0]).abs() + (p[1] - q[1]).abs()};
    /// let mut pc = PointCloud::new(manhattan_dist);
    /// let points = vec![[1.0, 1.0], [2.0, 2.0], [3.0, 2.0]];
    /// for p in &points {
    ///     pc.add_point(&p);
    /// }
    /// let q = [0.5, 0.5];
    /// pc.get_nearest_k(&q, 2);
    ///
    /// ```
    pub fn get_nearest_k(&self, p: &T, k: usize) -> Vec<(f64, &T)> {
        let mut h = heap::Heap::new(k);
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

    /// Get the len / number of objects in PointCloud
    /// Example:
    /// ```
    /// extern crate pointcloud;
    /// use pointcloud::PointCloud;
    /// let manhattan_dist = |p: &[f64;2], q: &[f64;2]| -> f64 {(p[0] - q[0]).abs() + (p[1] - q[1]).abs()};
    /// let mut pc = PointCloud::new(manhattan_dist);
    /// let points = vec![[1.0, 1.0], [2.0, 2.0], [3.0, 2.0]];
    /// for p in &points {
    ///     pc.add_point(&p);
    /// }
    /// pc.len();
    /// ```
    pub fn len(&self) -> usize {
        self.points.len()
    }
}
