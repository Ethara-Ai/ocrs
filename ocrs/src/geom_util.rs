//! Shared utilities for working with shapes.
use rten_imageproc::{Coord, Line, LineF, RotatedRect};
/// Return the edge of a rotated rect consisting of the two right-most vertices.
pub fn rightmost_edge(r: &RotatedRect) -> LineF {
    panic!("STUB: not implemented");
}
/// Return the edge of a rotated rect consisting of the two left-most vertices.
pub fn leftmost_edge(r: &RotatedRect) -> LineF {
    panic!("STUB: not implemented");
}
/// Normalize a line so that it's endpoints are sorted from top to bottom.
pub fn downwards_line<T: Coord>(l: Line<T>) -> Line<T> {
    panic!("STUB: not implemented");
}
