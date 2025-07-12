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
- Face normals calculation
- Area
- Finding non-manifold edges
- Is connected
- Mesh -> Graph
- ... and more ;)

### Mesh processing tools
- Mesh welding
- Split by angle
- Normals flipping
![Normals flipping](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/normals_flipping.gif)
- Mesh planar simplify
![Planar simplify](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/simplify_planar.gif)
- Mesh deduplication
![Deduplication](/img/deduplication.gif)

### Mesh creation tools
- 3D Polygon triangulation with holes
![Polygon triangulation](https://raw.githubusercontent.com/paireks/meshmeshmesh/refs/heads/master/img/polygon_triangulation.gif)

### File formats

This library works natively on .bim file format, which is open minimalist text mesh format described here: https://github.com/paireks/dotbim 

To load dotbim scene example:

```rust
use std::fs;
use serde_json::{from_value, to_string};
use meshmeshmesh::*;

fn import_scene() {
    let path = "File.bim";
    let read_file = fs::read_to_string(path).unwrap();
    let json: serde_json::Value = serde_json::from_str(&*read_file).unwrap();
    let mut input_scene: scene::Scene = from_value(json).unwrap();
}
```

To save dotbim scene example:

```rust
use std::fs;
use serde_json::{from_value, to_string};
use meshmeshmesh::*;

fn export_scene() {
    let file_serialized = to_string(&input_scene);
    let file_serialized_string = file_serialized.ok().unwrap();
    let path_after = "FileExported.bim";
    fs::write(path_after, file_serialized_string).expect("Unable to write the file");
}
```

There are many ways to export or convert into this file format using tools described here: https://github.com/paireks/dotbim?tab=readme-ov-file#apps-supporting-bim

## Documentation & Examples

https://docs.rs/meshmeshmesh/latest/meshmeshmesh/

## Dependencies

meshmeshmesh is using:

- iTriangle (https://github.com/iShape-Rust/iTriangle) for triangulation / tesselation.
- serde & serde_json (https://github.com/serde-rs/json) for json serialization / deserialization.

## License

meshmeshmesh is under the AGPL-3.0 license.

## Author

Wojciech Radaczyński (https://radaczynski.pl/)
