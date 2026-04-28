use core::f32;
use std::collections::HashMap;
use rayon::prelude::*;
use rten::ctc::{CtcDecoder, CtcHypothesis};
use rten::{thread_pool, Dimension, FloatOperators};
use rten_imageproc::{
    bounding_rect, BoundingRect, Line, Point, PointF, Polygon, Rect, RotatedRect,
};
use rten_tensor::prelude::*;
use rten_tensor::{NdTensor, NdTensorView, NdTensorViewMut, Tensor};
use crate::errors::ModelRunError;
use crate::geom_util::{downwards_line, leftmost_edge, rightmost_edge};
use crate::model::Model;
use crate::preprocess::BLACK_VALUE;
use crate::text_items::{TextChar, TextLine};
/// Return a polygon which contains all the rects in `words`.
///
/// `words` is assumed to be a series of disjoint rectangles ordered from left
/// to right. The returned points are arranged in clockwise order starting from
/// the top-left point.
///
/// There are several ways to compute a polygon for a line. The simplest is
/// to use [min_area_rect] on the union of the line's points. However the result
/// will not tightly fit curved lines. This function returns a polygon which
/// closely follows the edges of individual words.
fn line_polygon(words: &[RotatedRect]) -> Vec<Point> {
    panic!("STUB: not implemented");
}
/// Compute width to resize a text line image to, for a given height.
fn resized_line_width(orig_width: i32, orig_height: i32, height: i32) -> u32 {
    panic!("STUB: not implemented");
}
/// Details about a text line needed to prepare the input to the text
/// recognition model.
#[derive(Clone)]
struct TextRecLine {
    /// Index of this line in the list of lines found in the image.
    index: usize,
    /// Region of the image containing this line.
    region: Polygon,
    /// Width to resize this line to.
    resized_width: u32,
}
fn prepare_text_line(
    image: NdTensorView<f32, 3>,
    page_rect: Rect,
    line_region: &Polygon,
    resized_width: u32,
    output_height: usize,
) -> NdTensor<f32, 2> {
    panic!("STUB: not implemented");
}
/// Prepare an NCHW tensor containing a batch of text line images, for input
/// into the text recognition model.
///
/// For each line in `lines`, the line region is extracted from `image`, resized
/// to a fixed `output_height` and a line-specific width, then copied to the
/// output tensor. Lines in the batch can have different widths, so the output
/// is padded on the right side to a common width of `output_width`.
fn prepare_text_line_batch(
    image: &NdTensorView<f32, 3>,
    lines: &[TextRecLine],
    page_rect: Rect,
    output_height: usize,
    output_width: usize,
) -> NdTensor<f32, 4> {
    panic!("STUB: not implemented");
}
/// Return the bounding rectangle of the slice of a polygon with X coordinates
/// between `min_x` and `max_x` inclusive.
fn polygon_slice_bounding_rect(
    poly: Polygon<i32, &[Point]>,
    min_x: i32,
    max_x: i32,
) -> Option<Rect> {
    panic!("STUB: not implemented");
}
/// Method used to decode sequence model outputs to a sequence of labels.
///
/// See [CtcDecoder] for more details.
#[derive(Copy, Clone, Default)]
pub enum DecodeMethod {
    #[default]
    Greedy,
    BeamSearch { width: u32 },
}
#[derive(Clone, Default)]
pub struct RecognitionOpt<'a> {
    pub debug: bool,
    /// Method used to decode character sequence outputs to character values.
    pub decode_method: DecodeMethod,
    pub alphabet: &'a str,
    pub excluded_char_labels: Option<&'a [usize]>,
}
/// Input and output from recognition for a single text line.
struct LineRecResult {
    /// Input to the recognition model.
    line: TextRecLine,
    /// Length of input sequences to recognition model, padded so that all
    /// lines in batch have the same length.
    rec_input_len: usize,
    /// Length of output sequences from recognition model, used as input to
    /// CTC decoding.
    ctc_input_len: usize,
    /// Output label sequence produced by CTC decoding.
    ctc_output: CtcHypothesis,
}
/// Combine information from the input and output of text line recognition
/// to produce [TextLine]s containing character sequences and bounding boxes
/// for each line.
///
/// Entries in the result may be `None` if no text was recognized for a line.
fn text_lines_from_recognition_results(
    results: &[LineRecResult],
    alphabet: &str,
) -> Vec<Option<TextLine>> {
    panic!("STUB: not implemented");
}
/// Extracts character sequences and coordinates from text lines detected in
/// an image.
pub struct TextRecognizer {
    model: Box<dyn Model + Send + Sync>,
    input_shape: Vec<Dimension>,
}
impl TextRecognizer {
    /// Initialize a text recognizer from a trained RTen model. Fails if the
    /// model does not have the expected inputs or outputs.
    pub fn from_model(
        model: impl Model + Send + Sync + 'static,
    ) -> anyhow::Result<TextRecognizer> {
        panic!("STUB: not implemented");
    }
    /// Return the expected height of input line images.
    fn input_height(&self) -> u32 {
        panic!("STUB: not implemented");
    }
    /// Run text recognition on an NCHW batch of text line images, and return
    /// a `[batch, seq, label]` tensor of class probabilities.
    fn run(&self, input: NdTensor<f32, 4>) -> Result<NdTensor<f32, 3>, ModelRunError> {
        panic!("STUB: not implemented");
    }
    /// Prepare a text line for input into the recognition model.
    ///
    /// This method exists for model debugging purposes to expose the
    /// preprocessing that [TextRecognizer::recognize_text_lines] does.
    pub fn prepare_input(
        &self,
        image: NdTensorView<f32, 3>,
        line: &[RotatedRect],
    ) -> NdTensor<f32, 2> {
        panic!("STUB: not implemented");
    }
    /// Recognize text lines in an image.
    ///
    /// `image` is a CHW greyscale image with values in the range `ZERO_VALUE` to
    /// `ZERO_VALUE + 1`. `lines` is a list of detected text lines, where each line
    /// is a sequence of word rects. `model` is a recognition model which accepts an
    /// NCHW tensor of greyscale line images and outputs a `[sequence, batch, label]`
    /// tensor of log probabilities of character classes, which must be converted to
    /// a character sequence using CTC decoding.
    ///
    /// Entries in the result can be `None` if no text was found in a line.
    pub fn recognize_text_lines(
        &self,
        image: NdTensorView<f32, 3>,
        lines: &[Vec<RotatedRect>],
        opts: RecognitionOpt,
    ) -> anyhow::Result<Vec<Option<TextLine>>> {
        panic!("STUB: not implemented");
    }
    /// Post-process recognition model outputs to filter excluded characters.
    ///
    /// `input_seq_slice` is a (seq, char_prob) matrix of log probabilities for
    /// characters. `excluded_char_labels` specifies indices of characters that
    /// should be excluded, by setting the log probability to -Inf.
    fn filter_excluded_char_labels<'a>(
        excluded_char_labels: Option<&[usize]>,
        input_seq_slice: &'a mut NdTensorViewMut<'_, f32, 2>,
    ) -> NdTensorView<'a, f32, 2> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use rten_imageproc::{BoundingRect, Point, Polygon, RotatedRect, Vec2};
    use super::line_polygon;
    #[test]
    fn test_line_polygon() {
        let words: Vec<RotatedRect> = (0..5)
            .map(|i| {
                let center = Point::from_yx(10., i as f32 * 20.);
                let width = 10.;
                let height = 5.;
                let up = Vec2::from_yx(if i % 2 == 0 { -1. } else { 1. }, 0.);
                RotatedRect::new(center, up, width, height)
            })
            .collect();
        let poly = Polygon::new(line_polygon(&words));
        assert!(poly.is_simple());
        for word in words {
            let center = word.bounding_rect().center();
            assert!(
                poly.contains_pixel(Point::from_yx(center.y.round() as i32, center.x
                .round() as i32))
            );
        }
    }
}
