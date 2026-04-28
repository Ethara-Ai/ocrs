use wasm_bindgen::prelude::*;
use rten::{op_registry, Model, ModelOptions, OpRegistry};
use rten_imageproc::{min_area_rect, BoundingRect, PointF};
use rten_tensor::prelude::*;
use crate::{
    ImageSource, OcrEngine as BaseOcrEngine, OcrEngineParams, OcrInput, TextItem,
};
/// Options for constructing an [OcrEngine].
#[wasm_bindgen]
pub struct OcrEngineInit {
    detection_model: Option<Model>,
    recognition_model: Option<Model>,
}
impl Default for OcrEngineInit {
    fn default() -> OcrEngineInit {
        panic!("STUB: not implemented");
    }
}
#[wasm_bindgen]
impl OcrEngineInit {
    #[wasm_bindgen(constructor)]
    pub fn new() -> OcrEngineInit {
        panic!("STUB: not implemented");
    }
    fn op_registry() -> OpRegistry {
        panic!("STUB: not implemented");
    }
    /// Load a model for text detection.
    #[wasm_bindgen(js_name = setDetectionModel)]
    pub fn set_detection_model(&mut self, data: Vec<u8>) -> Result<(), String> {
        panic!("STUB: not implemented");
    }
    /// Load a model for text recognition.
    #[wasm_bindgen(js_name = setRecognitionModel)]
    pub fn set_recognition_model(&mut self, data: Vec<u8>) -> Result<(), String> {
        panic!("STUB: not implemented");
    }
}
/// OcrEngine is the main API for performing OCR in WebAssembly.
#[wasm_bindgen]
pub struct OcrEngine {
    engine: BaseOcrEngine,
}
#[wasm_bindgen]
impl OcrEngine {
    /// Construct a new `OcrEngine` using the models and other settings given
    /// by `init`.
    ///
    /// To detect text in an image, `init` must have a detection model set.
    /// To recognize text, `init` must have a recognition model set.
    #[wasm_bindgen(constructor)]
    pub fn new(init: OcrEngineInit) -> Result<OcrEngine, String> {
        panic!("STUB: not implemented");
    }
    /// Prepare an image for analysis by the OCR engine.
    ///
    /// The image is an array of pixels in row-major, channels last order. This
    /// matches the format of the
    /// [ImageData](https://developer.mozilla.org/en-US/docs/Web/API/ImageData)
    /// API. Supported channel combinations are RGB and RGBA. The number of
    /// channels is inferred from the length of `data`.
    #[wasm_bindgen(js_name = loadImage)]
    pub fn load_image(
        &self,
        width: u32,
        height: u32,
        data: &[u8],
    ) -> Result<Image, String> {
        panic!("STUB: not implemented");
    }
    /// Detect text in an image.
    ///
    /// Returns a list of lines that were found. These can be passed to
    /// `recognizeText` identify the characters.
    #[wasm_bindgen(js_name = detectText)]
    pub fn detect_text(&self, image: &Image) -> Result<Vec<DetectedLine>, String> {
        panic!("STUB: not implemented");
    }
    /// Recognize text that was previously detected with `detectText`.
    ///
    /// Returns a list of `TextLine` objects that can be used to query the text
    /// and bounding boxes of each line.
    #[wasm_bindgen(js_name = recognizeText)]
    pub fn recognize_text(
        &self,
        image: &Image,
        lines: Vec<DetectedLine>,
    ) -> Result<Vec<TextLine>, String> {
        panic!("STUB: not implemented");
    }
    /// Detect and recognize text in an image.
    ///
    /// Returns a single string containing all the text found in reading order.
    #[wasm_bindgen(js_name = getText)]
    pub fn get_text(&self, image: &Image) -> Result<String, String> {
        panic!("STUB: not implemented");
    }
    /// Detect and recognize text in an image.
    ///
    /// Returns a list of `TextLine` objects that can be used to query the text
    /// and bounding boxes of each line.
    #[wasm_bindgen(js_name = getTextLines)]
    pub fn get_text_lines(&self, image: &Image) -> Result<Vec<TextLine>, String> {
        panic!("STUB: not implemented");
    }
}
/// A pre-processed image that can be passed as input to `OcrEngine.loadImage`.
#[wasm_bindgen]
pub struct Image {
    input: OcrInput,
}
#[wasm_bindgen]
impl Image {
    /// Return the number of channels in the image.
    pub fn channels(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Return the width of the image.
    pub fn width(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Return the height of the image.
    pub fn height(&self) -> usize {
        panic!("STUB: not implemented");
    }
    /// Return the image data in row-major, channels-last order.
    pub fn data(&self) -> Vec<u8> {
        panic!("STUB: not implemented");
    }
}
#[wasm_bindgen]
#[derive(Clone)]
pub struct RotatedRect {
    rect: rten_imageproc::RotatedRect,
}
#[wasm_bindgen]
impl RotatedRect {
    /// Return an array of the X and Y coordinates of corners of this rectangle,
    /// arranged as `[x0, y0, ... x3, y3]`.
    pub fn corners(&self) -> Vec<f32> {
        panic!("STUB: not implemented");
    }
    /// Return the coordinates of the axis-aligned bounding rectangle of this
    /// rotated rect.
    ///
    /// The result is a `[left, top, right, bottom]` array of coordinates.
    #[wasm_bindgen(js_name = boundingRect)]
    pub fn bounding_rect(&self) -> Vec<f32> {
        panic!("STUB: not implemented");
    }
}
/// A line of text that has been detected, but not recognized.
///
/// This contains information about the location of the text, but not the
/// string contents.
#[wasm_bindgen]
#[derive(Clone)]
pub struct DetectedLine {
    words: Vec<RotatedRect>,
}
#[wasm_bindgen]
impl DetectedLine {
    fn new(words: Vec<RotatedRect>) -> DetectedLine {
        panic!("STUB: not implemented");
    }
    #[wasm_bindgen(js_name = rotatedRect)]
    pub fn rotated_rect(&self) -> RotatedRect {
        panic!("STUB: not implemented");
    }
    pub fn words(&self) -> Vec<RotatedRect> {
        panic!("STUB: not implemented");
    }
}
/// Bounding box and text of a word that was recognized.
#[wasm_bindgen]
#[derive(Clone)]
pub struct TextWord {
    rect: RotatedRect,
    text: String,
}
#[wasm_bindgen]
impl TextWord {
    pub fn text(&self) -> String {
        panic!("STUB: not implemented");
    }
    /// Return the oriented bounding rectangle containing the characters in
    /// this word.
    #[wasm_bindgen(js_name = rotatedRect)]
    pub fn rotated_rect(&self) -> RotatedRect {
        panic!("STUB: not implemented");
    }
}
/// A sequence of `TextWord`s that were recognized, forming a line.
#[wasm_bindgen]
#[derive(Clone)]
pub struct TextLine {
    line: Option<super::TextLine>,
}
#[wasm_bindgen]
impl TextLine {
    pub fn text(&self) -> String {
        panic!("STUB: not implemented");
    }
    pub fn words(&self) -> Vec<TextWord> {
        panic!("STUB: not implemented");
    }
}
