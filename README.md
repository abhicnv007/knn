# Pointcloud

![Build Status](https://github.com/abhicnv007/pointcloud/workflows/Test/badge.svg)

A library to find nearest neighbours in rust.

## Usage

```rust
extern crate pointcloud;
use pointcloud::PointCloud;

fn manhattan(p: &[f64;2], q: &[f64;2]) -> f64 {
    let mut d = 0.0;
    for i in 0..p.len() {
        d += (p[i] - q[i]).abs();
    }
    d
}

fn main() {
    let mut pc = PointCloud::new(manhattan);
    let coords = vec![[1.0, 1.0], [2.0, 2.0], [10.0, 5.0], [11.0, 15.0]];
    for i in 0..coords.len() {
        pc.add_point(&coords[i]);
    }

    let d = pc.get_nearest_n(&[2.1, 2.1], 2);
    println!("{:?}", d)
    // output :
    // [(0.20000000000000018, [2.0, 2.0]), (2.2, [1.0, 1.0])]
}
```
