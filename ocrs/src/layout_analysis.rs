use rten_imageproc::{bounding_rect, BoundingRect, Line, LineF, Point, Rect, RotatedRect};
use crate::geom_util::{leftmost_edge, rightmost_edge};
mod empty_rects;
use empty_rects::{max_empty_rects, FilterOverlapping};
fn rects_separated_by_line(a: &RotatedRect, b: &RotatedRect, l: LineF) -> bool {
    panic!("STUB: not implemented");
}
/// Group rects into lines. Each line is a chain of oriented rects ordered
/// left-to-right, which may overlap.
///
/// `separators` is a list of line segments that prevent the formation of
/// lines which cross them. They can be used to specify column boundaries
/// for example.
pub fn group_into_lines(
    rects: &[RotatedRect],
    separators: &[LineF],
) -> Vec<Vec<RotatedRect>> {
    panic!("STUB: not implemented");
}
/// A text line is a sequence of RotatedRects for words, organized from left to
/// right.
type TextLine = Vec<RotatedRect>;
type TextParagraph = Vec<TextLine>;
/// Find separators between text blocks.
///
/// This includes separators between columns, as well as between sections (eg.
/// headings and article contents).
pub fn find_block_separators(words: &[RotatedRect]) -> Vec<Rect> {
    panic!("STUB: not implemented");
}
/// Group words into lines and sort them into reading order.
pub fn find_text_lines(words: &[RotatedRect]) -> Vec<Vec<RotatedRect>> {
    panic!("STUB: not implemented");
}
#[cfg(test)]
mod tests {
    use rten_imageproc::{BoundingRect, Point, Rect, RectF, RotatedRect};
    use super::{find_block_separators, find_text_lines};
    use crate::test_util::{gen_rect_grid, union_rects};
    #[test]
    fn test_find_block_separators() {
        struct Case {
            lines: i32,
            words: i32,
            word_h: i32,
            word_w: i32,
            line_gap: i32,
            word_gap: i32,
            expected_separators: usize,
        }
        let cases = [
            Case {
                lines: 2,
                words: 2,
                word_h: 10,
                word_w: 20,
                line_gap: 50,
                word_gap: -5,
                expected_separators: 2,
            },
        ];
        for Case {
            lines,
            words,
            word_h,
            word_w,
            line_gap,
            word_gap,
            expected_separators,
        } in cases {
            let words: Vec<RotatedRect> = gen_rect_grid(
                    Point::from_yx(0, 0),
                    (lines, words),
                    (word_h, word_w),
                    (line_gap, word_gap),
                )
                .into_iter()
                .map(|rect| RotatedRect::from_rect(rect.to_f32()))
                .collect();
            let separators = find_block_separators(&words);
            assert_eq!(separators.len(), expected_separators);
        }
    }
    #[test]
    fn test_find_text_lines() {
        let page = Rect::from_tlbr(0, 0, 80, 90);
        let col_rows = 10;
        let col_words = 5;
        let (line_gap, word_gap) = (3, 2);
        let (word_h, word_w) = (5, 5);
        let left_col = gen_rect_grid(
            Point::from_yx(0, 0),
            (col_rows, col_words),
            (word_h, word_w),
            (line_gap, word_gap),
        );
        let left_col_boundary = union_rects(&left_col).unwrap();
        assert!(page.contains(left_col_boundary));
        let right_col = gen_rect_grid(
            Point::from_yx(0, left_col_boundary.right() + 20),
            (col_rows, col_words),
            (word_h, word_w),
            (line_gap, word_gap),
        );
        let right_col_boundary = union_rects(&right_col).unwrap();
        assert!(page.contains(right_col_boundary));
        let mut words: Vec<_> = left_col
            .iter()
            .chain(right_col.iter())
            .copied()
            .map(|r| RotatedRect::from_rect(r.to_f32()))
            .collect();
        let mut rng = fastrand::Rng::with_seed(1234);
        rng.shuffle(&mut words);
        let lines = find_text_lines(&words);
        assert_eq!(lines.len() as i32, col_rows * 2);
        for line in lines {
            assert_eq!(line.len() as i32, col_words);
            let bounding_rect: Option<RectF> = line
                .iter()
                .fold(
                    None,
                    |br, r| match br {
                        Some(br) => Some(br.union(r.bounding_rect())),
                        None => Some(r.bounding_rect()),
                    },
                );
            let (line_height, line_width) = bounding_rect
                .map(|br| (br.height(), br.width()))
                .unwrap_or((0., 0.));
            assert!((line_height - word_h as f32).abs() <= 1.);
            let expected_width = col_words * (word_w + word_gap) - word_gap;
            assert!((line_width - expected_width as f32).abs() <= 1.);
        }
    }
}
