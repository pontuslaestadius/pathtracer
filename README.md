# Introduction
Pathfinder can create nodes, groups of nodes and connection in between them, and plot them resulting in an image.
It handles text interpretation and data visualisation based on input files.

[![Build Status](https://travis-ci.org/pontuslaestadius/pathfinder.svg?branch=master)](https://travis-ci.org/pontuslaestadius/pathfinder)
 
[Documentation](https://docs.rs/pathfinder/0.3.8/pathfinder/)

Test coverage: 21%

# Preview

Inside the examples directory you can find example implementations of common functions. Along with some output examples. The following are constructed from /examples.

![Pathfinder Logotype](examples/out/hello_world.png "Logo")

![Pathfinder Logotype_gif](examples/out/hello_world_gif.gif "Gif")

![Groups example](examples/out/random.png "Groups")

![Data Visualization](examples/out/git_log.png "Data")

# Example

Each struct has access to the Map wrapper, which can be used to easily draw up Nodes, Groups and Links. Groups can be generated simply using add_children.

```
extern crate pathfinder;
use pathfinder::Group;
use pathfinder::map::network;
use std::path::Path;

let mut groups = Group::from_list([(0, 0), (100, 100)]);
for group in groups.iter_mut() {
    network::add_children(&mut group, 100);
}
let mut map = Map::new();
map.map(&groups)
   .image.unwrap().save(&Path::new("out.png"));
```

