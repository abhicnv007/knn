extern crate pointcloud;

use pointcloud::PointCloud;

#[derive(Clone, Debug)]
pub struct Point {
    coords: [f64; 2],
}

impl Point {
    pub fn new(c: [f64; 2]) -> Point {
        Point { coords: c }
    }
}

fn euclidean(p: &Point, q: &Point) -> f64 {
    let mut sosq = 0.0;
    for i in 0..p.coords.len() {
        sosq += (p.coords[i] - q.coords[i]).powf(2.0)
    }
    sosq.sqrt()
}

#[test]
fn add_point() {
    let mut pc = PointCloud::new(euclidean);
    // add 4 points
    let coords = vec![[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
    let points: Vec<Point> = coords.iter().map(|c| Point::new(*c)).collect();

    for i in 0..points.len() {
        pc.add_point(&points[i])
    }
    assert_eq!(pc.len(), 4);
}

#[test]
fn test_get_nearest_n() {
    let mut pc = PointCloud::new(euclidean);

    let points = [
        Point::new([2.0, 2.0]),
        Point::new([2.0, 1.0]),
        Point::new([3.0, 1.0]),
    ];
    for i in 0..points.len() {
        pc.add_point(&points[i])
    }

    let p = Point::new([1.0, 1.0]);
    let d = pc.get_nearest_n(&p, 1);
    // assert that it returns n points
    assert_eq!(d.len(), 1);
    println!("{:?}", d);
    assert_eq!(d[0].0, 1.0);
    assert_eq!(d[0].1 as *const _, &points[1]);

    let d = pc.get_nearest_n(&p, 2);

    let expected = vec![(1.0, &points[1]), (2.0f64.sqrt(), &points[0])];
    assert_eq!(d.len(), 2);
    for i in 0..expected.len() {
        assert_eq!(d[i].0, expected[i].0);
        assert_eq!(d[i].1 as *const _, expected[i].1);
    }
}
