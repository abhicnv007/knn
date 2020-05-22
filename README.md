# Pointcloud

![Build Status](https://github.com/abhicnv007/pointcloud/workflows/Test/badge.svg)

A library to find nearest neighbours in rust.

## Usage

Add this to your `Cargo.toml`

```toml
[dependencies]
knn = "0.1.3"
```

and use like this

```rust
extern crate knn;
use knn::PointCloud;

fn main() {
    let manhattan = |p: &[f64;2], q: &[f64;2]| {(q[0] - p[0]).abs() + (q[1] - p[1]).abs()};
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

For updated and detailed docs refer [here](https://docs.rs/knn/)
