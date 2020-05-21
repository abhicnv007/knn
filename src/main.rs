mod heap;
mod point;

fn main() {
    let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];

    // add the points to the point cloud
    let mut pc = point::PointCloud::new();
    for i in 0..points.len() {
        pc.add_point(point::Point::new(points[i]))
    }

    //get the closes point
    let (d, p) = pc.get_nearest_n(&point::Point::new([10.0, 14.0]), 4);

    println!("{:?} {:?}", d, p);

    // let mut h = heap::Heap::new(4);
    // for i in vec![6, 7, 8, 10, 4, 2, 3] {
    //     h.insert(i as f64)
    // }

    // println!("{:?}", h.get_elements())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn add_point() {
        let mut pc = point::PointCloud::new();
        // add 4 points
        let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
        for i in 0..points.len() {
            pc.add_point(point::Point::new(points[i]))
        }
        assert_eq!(pc.len(), 4);
    }
}
