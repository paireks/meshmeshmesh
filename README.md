# meshmeshmesh [![Crates.io Version](https://img.shields.io/crates/v/meshmeshmesh)](https://crates.io/crates/meshmeshmesh)

![meshmeshmesh](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/meshmeshmesh180.bmp)

## Introduction

Open-source mesh engine written in Rust programming language.

It could be useful for mesh modification, processing, fixing, etc.

## Installation

https://crates.io/crates/meshmeshmesh

Run the following command:

```text
cargo add meshmeshmesh
```

## Features

### Mesh analysis tools
- Triangle/Ray intersection
- Mesh/Ray intersection
- Normals calculation
- Area
- Finding non-manifold edges
- Is connected
- Mesh -> Graph
- ... and more ;)

### Mesh processing tools
- Mesh welding
![Welding](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/welding.gif)
- Split by angle
![Split by angle](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/split_by_angle.gif)
- Normals flipping
![Normals flipping](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/normals_flipping.gif)
- Mesh planar simplify
![Planar simplify](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/simplify_planar.gif)

### Mesh creation tools
- 3D Polygon triangulation with holes
![Polygon triangulation](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/polygon_triangulation.gif)

## Documentation & Examples

https://docs.rs/meshmeshmesh/latest/meshmeshmesh/

## Dependencies

meshmeshmesh is using:

- iTriangle (https://github.com/iShape-Rust/iTriangle) for triangulation / tesselation.

## Author

Wojciech Radaczyński (https://radaczynski.pl/)
