extern crate pointcloud;

use pointcloud::Point;
use pointcloud::PointCloud;

#[test]
fn add_point() {
    let mut pc = PointCloud::new();
    // add 4 points
    let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
    for i in 0..points.len() {
        pc.add_point(Point::new(points[i]))
    }
    assert_eq!(pc.len(), 4);
}

#[test]
fn test_get_nearest_n() {
    let mut pc = PointCloud::new();
    pc.add_point(Point::new([2.0, 2.0]));
    pc.add_point(Point::new([2.0, 1.0]));
    pc.add_point(Point::new([3.0, 1.0]));

    let p = Point::new([1.0, 1.0]);
    let (d, _) = pc.get_nearest_n(&p, 1);
    // assert that it returns n points
    assert_eq!(d.len(), 1);
    assert_eq!(d[0], 1.0);

    let (d, _) = pc.get_nearest_n(&p, 2);

    let expected = vec![1.0, 2.0f64.sqrt()];
    assert_eq!(d.len(), 2);
    assert_eq!(d, expected);
}
