use criterion::{criterion_group, criterion_main, Criterion};

extern crate rand;
use rand::Rng;

extern crate pointcloud;
use pointcloud::Point;
use pointcloud::PointCloud;

fn benchmark_pointcloud(c: &mut Criterion) {
    c.bench_function("PointCloud: insert 1 million points", |b| {
        let mut pc = PointCloud::new();
        let mut points = Vec::new();
        // do the random number outside so that it does not affect the benchmark
        for _ in 0..1_000_000 {
            points.push([rand::thread_rng().gen(), rand::thread_rng().gen()]);
        }

        b.iter(|| {
            for p in &points {
                pc.add_point(Point::new(*p));
            }
        });
    });

    c.bench_function(
        "PointCloud: get nearest 10 neighbours out of 1 million",
        |b| {
            // create a pointcloud with 1000 elements
            let mut pc = PointCloud::new();
            for _ in 0..1_000_000 {
                pc.add_point(Point::new([
                    rand::thread_rng().gen(),
                    rand::thread_rng().gen(),
                ]));
            }

            b.iter(|| pc.get_nearest_n(&Point::new([100.0, 100.0]), 10));
        },
    );
}

criterion_group!(benches, benchmark_pointcloud);
criterion_main!(benches);
