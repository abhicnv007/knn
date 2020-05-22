use criterion::{criterion_group, criterion_main, Criterion};

extern crate rand;
use rand::Rng;

extern crate knn;
use knn::PointCloud;

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

fn benchmark_pointcloud(c: &mut Criterion) {
    c.bench_function("PointCloud: insert 1 million points", |b| {
        let mut pc = PointCloud::new(euclidean);
        let points: Vec<Point> = (0..1_000_000)
            .map(|_| Point::new([rand::thread_rng().gen(), rand::thread_rng().gen()]))
            .collect();

        b.iter(|| {
            for p in &points {
                pc.add_point(p);
            }
        });
    });

    c.bench_function(
        "PointCloud: get nearest 100 neighbours out of 1 million",
        |b| {
            // create a pointcloud with 1000 elements
            let manhattan_dist = |p: &Point, q: &Point| -> f64 {
                (p.coords[0] - q.coords[0]).abs() + (p.coords[1] - q.coords[1]).abs()
            };
            let mut pc = PointCloud::new(manhattan_dist);
            let points: Vec<Point> = (0..1_000_000)
                .map(|_| Point::new([rand::thread_rng().gen(), rand::thread_rng().gen()]))
                .collect();
            for p in &points {
                pc.add_point(p);
            }

            b.iter(|| pc.get_nearest_k(&Point::new([100.0, 100.0]), 100));
        },
    );
}

criterion_group!(benches, benchmark_pointcloud);
criterion_main!(benches);
