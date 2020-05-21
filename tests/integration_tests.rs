extern crate pointcloud;

#[test]
fn add_point() {
    let mut pc = pointcloud::PointCloud::new();
    // add 4 points
    let points = [[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
    for i in 0..points.len() {
        pc.add_point(pointcloud::Point::new(points[i]))
    }
    assert_eq!(pc.len(), 4);
}
