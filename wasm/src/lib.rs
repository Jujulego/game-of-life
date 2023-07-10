extern crate nalgebra as na;
extern crate pythagore as py;

mod traits;
mod universe;
mod universe_style;
mod utils;

mod binary_tree;

#[cfg(feature = "binary-tree")]
mod binary_query;

#[cfg(feature = "quadtree")]
mod quadtree;
