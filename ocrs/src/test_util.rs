use rten_imageproc::{Point, Rect};
/// Generate a grid of uniformly sized and spaced rects.
///
/// `grid_shape` is a (rows, columns) tuple. `rect_size` and `gap_size` are
/// (height, width) tuples.
pub fn gen_rect_grid(
    top_left: Point,
    grid_shape: (i32, i32),
    rect_size: (i32, i32),
    gap_size: (i32, i32),
) -> Vec<Rect> {
    panic!("STUB: not implemented");
}
/// Return the union of `rects` or `None` if rects is empty.
pub fn union_rects(rects: &[Rect]) -> Option<Rect> {
    panic!("STUB: not implemented");
}
