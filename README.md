![crates.io](https://img.shields.io/crates/d/pathfinder.svg)
[![Build Status](https://travis-ci.org/pontuslaestadius/pathfinder.svg?branch=master)](https://travis-ci.org/pontuslaestadius/pathfinder)
[![Documentation](https://img.shields.io/badge/docs.rs-latest-blue.svg)](https://docs.rs/pathfinder/latest/pathfinder/)
[![Test coverage](https://img.shields.io/badge/Tarpaulin%20Coverage-64-yellow.svg)](https://github.com/xd009642/tarpaulin)
[![GitHub license](https://img.shields.io/github/license/pontuslaestadius/pathfinder.svg)](https://github.com/pontuslaestadius/pathfinder/blob/master/LICENSE)


![Logo of the project](examples/out/hello_world.png)


> Graph large number of Connected nodes mapped on to an Image or Gif.


Placing positioned objects on to an Image can cause many issue. This library is meant 
to ease interacting with images and create higher order abstractions which makes it 
easy to populate larger Image surfaces with large number of surface objects.


### Features

* [Connecting Nodes](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/hello_world.rs)
* [Traversing paths](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/mvp.rs)
* [Data visualisation](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/git_log.rs)
* [Large number of Nodes](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/random.rs)
* [Plot large number of Nodes](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/node_plot.rs)
* [Gifs](https://github.com/pontuslaestadius/pathfinder/blob/master/examples/hello_world_gif.rs)
* [Invocation macros for structures](https://docs.rs/pathfinder/latest/pathfinder/#macros)


## Example

Easily manipulate large number of Nodes with less accuracy through Groups.

```rust
use pathfinder::{Group, map};
use std::path::Path;

let mut groups = Group::from_list(&[(0, 0), (100, 100)]);
for group in groups.iter_mut() {
    group.add(100);
}
Map::new()
    .map(&groups)
    .save(&Path::new("out.png"));
```

Path through a list of connected nodes.

```rust
let pos = [(0, 0), (100, 100), (150, 50), (2000, 4000), (400, 600)];
let nodes = Node::from_list(&pos); // Nodes are named in sequence: A-Z.
let nodes = Node::linked_list(nodes);
let net = Network::new(nodes);
let path = net.path("A", "E");
node::path_print(&path?);
```


### Output

![Node plot](examples/out/node_plot.gif "Gif")

![Groups](examples/out/random.jpg "Groups")

