# Introduction
Pathfinder can create nodes, groups of nodes and connection in between them, and plot them resulting in an image.
It handles text interpretation and data visualisation based on input files.

[![Build Status](https://travis-ci.org/pontuslaestadius/pathfinder.svg?branch=master)](https://travis-ci.org/pontuslaestadius/pathfinder)
 
[Documentation](https://docs.rs/pathfinder/0.3.8/pathfinder/)

# Examples
Inside the examples directory you can find example implementations of common functions. Along with some output examples. The following are constructed from /examples.

![Pathfinder Logotype](out/examples/hello_world.png "Logo")

![Pathfinder Logotype_gif](out/examples/hello_world_gif.gif "Gif")

![Groups example](out/examples/random.png "Groups")

![Data Visualization](out/examples/git_log.png "Data")

A Node is the primary focus. As it's properties are reflected for Groups of nodes as well.
They are instantiated simply and are used drawing on a canvas.

```
use pathfinder::{Coordinate, Node};

let a: Node = Node::new("A", Coordinate::new(0,0));
let mut b: Node = Node::new("B", Coordinate::new(100,100));

b.link(&a);
```

This library contains a wrapper for image and gif encoding. Which can be used to easily draw up node, groups and links.

```
let mut map = Map::new();
map = map
    .map(&groups)
    .map(&nodes);

map.image.unwrap().save(&path);
```

# Planned activities
- Functioning pathing algorithms.
- Linking nodes more easier.
- More predictable node layout.
- Add more algorithms for pathfinding on a linked network.
