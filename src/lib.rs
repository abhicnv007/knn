mod heap;

#[derive(Clone, Debug)]
pub struct Point {
    coords: [f64; 2],
}

impl Point {
    pub fn new(c: [f64; 2]) -> Point {
        Point { coords: c }
    }
}

fn dist_squared(p: &Point, q: &Point) -> f64 {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add_point() {
        let mut pc = PointCloud::new();
        pc.add_point(Point { coords: [1.0, 2.0] });
        pc.add_point(Point { coords: [2.5, 6.5] });
        assert_eq!(pc.len(), 2);
    }

    #[test]
    fn test_get_nearest_n() {
        let mut pc = PointCloud::new();
        pc.add_point(Point { coords: [2.0, 2.0] });
        pc.add_point(Point { coords: [2.0, 1.0] });
        pc.add_point(Point { coords: [3.0, 1.0] });

        let (d, _p) = pc.get_nearest_n(&Point { coords: [1.0, 1.0] }, 1);
        // assert that it returns n points
        assert_eq!(d.len(), 1);
        assert_eq!(d[0], 1.0);
    }
}

// fn main() {
//     let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];

//     // add the points to the point cloud
//     let mut pc = PointCloud::new();
//     for i in 0..points.len() {
//         pc.add_point(Point::new(points[i]))
//     }

//     //get the closes point
//     let (d, p) = pc.get_nearest_n(&Point::new([10.0, 14.0]), 4);

//     println!("{:?} {:?}", d, p);

//     // let mut h = heap::Heap::new(4);
//     // for i in vec![6, 7, 8, 10, 4, 2, 3] {
//     //     h.insert(i as f64)
//     // }

//     // println!("{:?}", h.get_elements())
// }

// #[cfg(test)]
// mod tests {
//     use super::*;
//     #[test]
//     fn add_point() {
//         let mut pc = point::PointCloud::new();
//         // add 4 points
//         let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
//         for i in 0..points.len() {
//             pc.add_point(point::Point::new(points[i]))
//         }
//         assert_eq!(pc.len(), 4);
//     }
// }
