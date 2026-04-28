use anyhow::anyhow;
use rten::{Dimension, FloatOperators, Operators, RunOptions};
use rten_imageproc::{
    find_contours, min_area_rect, simplify_polygon, RetrievalMode, RotatedRect,
};
use rten_tensor::prelude::*;
use rten_tensor::{NdTensor, NdTensorView, Tensor};
use crate::model::Model;
use crate::preprocess::BLACK_VALUE;
/// Parameters that control post-processing of text detection model outputs.
#[derive(Clone, Debug, PartialEq)]
pub struct TextDetectorParams {
    /// Threshold for minimum area of returned rectangles.
    ///
    /// This can be used to filter out rects created by small false positives in
    /// the mask, at the risk of filtering out true positives. The more accurate
    /// the model producing the mask is, the smaller this value can be.
    pub min_area: f32,
    /// Threshold for per-pixel scores in output segmentation mask for
    /// classifying a pixel as text.
    pub text_threshold: f32,
}
impl Default for TextDetectorParams {
    fn default() -> TextDetectorParams {
        panic!("STUB: not implemented");
    }
}
/// Find the minimum-area oriented rectangles containing each connected
/// component in the binary mask `mask`.
fn find_connected_component_rects(
    mask: NdTensorView<bool, 2>,
    expand_dist: f32,
    min_area: f32,
) -> Vec<RotatedRect> {
    panic!("STUB: not implemented");
}
/// Text detector which finds the oriented bounding boxes of words in an input
/// image.
pub struct TextDetector {
    model: Box<dyn Model + Send + Sync>,
    params: TextDetectorParams,
    input_shape: Vec<Dimension>,
}
impl TextDetector {
    /// Initialize a DetectionModel from a trained RTen model.
    ///
    /// This will fail if the model doesn't have the expected inputs or outputs.
    pub fn from_model(
        model: impl Model + Send + Sync + 'static,
        params: TextDetectorParams,
    ) -> anyhow::Result<TextDetector> {
        panic!("STUB: not implemented");
    }
    /// Return the confidence threshold used to determine whether a pixel is
    /// text or not.
    pub fn threshold(&self) -> f32 {
        panic!("STUB: not implemented");
    }
    /// Detect text words in a greyscale image.
    ///
    /// `image` is a greyscale CHW image with values in the range `ZERO_VALUE` to
    /// `ZERO_VALUE + 1`. `model` is a model which takes an NCHW input tensor and
    /// returns a binary segmentation mask predicting whether each pixel is part of
    /// a text word or not. The image is padded and resized to the model's expected
    /// input size before performing detection.
    ///
    /// The result is an unsorted list of the oriented bounding rectangles of
    /// connected components (ie. text words) in the mask.
    pub fn detect_words(
        &self,
        image: NdTensorView<f32, 3>,
        debug: bool,
    ) -> anyhow::Result<Vec<RotatedRect>> {
        panic!("STUB: not implemented");
    }
    /// Detect text pixels in an image.
    ///
    /// Takes a greyscale (CHW) input image and returns a probability map
    /// indicating whether each pixel in the input is text.
    ///
    /// See [detect_words](TextDetector::detect_words) for more details of
    /// expected input.
    pub fn detect_text_pixels(
        &self,
        image: NdTensorView<f32, 3>,
        debug: bool,
    ) -> anyhow::Result<NdTensor<f32, 2>> {
        panic!("STUB: not implemented");
    }
}
#[cfg(test)]
mod tests {
    use rten_imageproc::{fill_rect, Point};
    use rten_tensor::prelude::*;
    use rten_tensor::NdTensor;
    use super::find_connected_component_rects;
    use crate::test_util::gen_rect_grid;
    #[test]
    fn test_find_connected_component_rects() {
        let mut mask = NdTensor::zeros([400, 400]);
        let (grid_h, grid_w) = (5, 5);
        let (rect_h, rect_w) = (10, 50);
        let rects = gen_rect_grid(
            Point::from_yx(10, 10),
            (grid_h, grid_w),
            (rect_h, rect_w),
            (10, 5),
        );
        for r in rects.iter() {
            let expanded = r.adjust_tlbr(0, 0, 1, 1);
            fill_rect(mask.view_mut(), expanded, true);
        }
        let min_area = 100.;
        let components = find_connected_component_rects(mask.view(), 0., min_area);
        assert_eq!(components.len() as i32, grid_h * grid_w);
        for c in components.iter() {
            let mut shape = [c.height().round() as i32, c.width().round() as i32];
            shape.sort();
            let mut expected_shape = [rect_h, rect_w];
            expected_shape.sort();
            assert_eq!(shape, expected_shape);
        }
    }
}
