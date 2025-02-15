// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

use serde::{Deserialize, Deserializer};

/// Image fit options.
/// This provides the deserializer for `usvg::FitTo`.
#[derive(Deserialize)]
#[serde(
  tag = "mode",
  content = "value",
  rename_all = "lowercase",
  deny_unknown_fields,
  remote = "usvg::FitTo"
)]
enum FitToDef {
  /// Keep original size.
  Original,
  /// Scale to width.
  Width(u32),
  /// Scale to height.
  Height(u32),
  /// Zoom by factor.
  Zoom(f32),
}

/// The javascript options passed to `render()`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsOptions {
  /// Font related options.
  pub font: JsFontOptions,

  /// Target DPI.
  ///
  /// Impact units conversion.
  ///
  /// Default: 96.0
  pub dpi: f64,

  /// A list of languages.
  ///
  /// Will be used to resolve a `systemLanguage` conditional attribute.
  ///
  /// Format: en, en-US.
  ///
  /// Default: [en]
  pub languages: Vec<String>,

  /// The default shape rendering method.
  ///
  /// Will be used when an SVG element's `shape-rendering` property is set to `auto`.
  ///
  /// Default: GeometricPrecision
  #[serde(deserialize_with = "deserialize_shape_rendering")]
  pub shape_rendering: usvg::ShapeRendering,

  /// The default text rendering method.
  ///
  /// Will be used when an SVG element's `text-rendering` property is set to `auto`.
  ///
  /// Default: OptimizeLegibility
  #[serde(deserialize_with = "deserialize_text_rendering")]
  pub text_rendering: usvg::TextRendering,

  /// The default image rendering method.
  ///
  /// Will be used when an SVG element's `image-rendering` property is set to `auto`.
  ///
  /// Default: OptimizeQuality
  #[serde(deserialize_with = "deserialize_image_rendering")]
  pub image_rendering: usvg::ImageRendering,

  /// The size to render the SVG.
  ///
  /// Default: Original
  #[serde(with = "FitToDef")]
  pub fit_to: usvg::FitTo,

  /// The background color of the SVG.
  ///
  /// Default: `None`
  pub background: Option<String>,

  /// Crop options
  pub crop: JsCropOptions,
}

impl Default for JsOptions {
  fn default() -> JsOptions {
    JsOptions {
      font: JsFontOptions::default(),
      dpi: 96.0,
      languages: vec!["en".to_string()],
      shape_rendering: usvg::ShapeRendering::default(),
      text_rendering: usvg::TextRendering::default(),
      image_rendering: usvg::ImageRendering::default(),
      fit_to: usvg::FitTo::Original,
      background: None,
      crop: JsCropOptions::default(),
    }
  }
}

/// The font options passed to `load_fonts()`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsFontOptions {
  /// If system fonts should be loaded.
  ///
  /// Default: true
  pub load_system_fonts: bool,

  /// A list of font files to load.
  pub font_files: Vec<String>,

  /// A list of font directories to load.
  pub font_dirs: Vec<String>,

  /// The default font family.
  ///
  /// Will be used when no `font-family` attribute is set in the SVG.
  ///
  /// Default: Times New Roman
  pub default_font_family: String,

  /// The default font size.
  ///
  /// Will be used when no `font-size` attribute is set in the SVG.
  ///
  /// Default: 12
  pub default_font_size: f64,

  /// The 'serif' font family.
  ///
  /// Default: Times New Roman
  pub serif_family: String,

  /// The 'sans-serif' font family.
  ///
  /// Default: Arial
  pub sans_serif_family: String,

  /// The 'cursive' font family.
  ///
  /// Default: Comic Sans MS
  pub cursive_family: String,

  /// The 'fantasy' font family.
  ///
  /// Default: Impact
  pub fantasy_family: String,

  /// The 'monospace' font family.
  ///
  /// Default: Courier New
  pub monospace_family: String,
}

impl Default for JsFontOptions {
  fn default() -> JsFontOptions {
    JsFontOptions {
      load_system_fonts: true,
      font_files: vec![],
      font_dirs: vec![],
      default_font_family: "Times New Roman".to_string(),
      default_font_size: 12.0,
      serif_family: "Times New Roman".to_string(),
      sans_serif_family: "Arial".to_string(),
      cursive_family: "Comic Sans MS".to_string(),
      fantasy_family: "Impact".to_string(),
      monospace_family: "Courier New".to_string(),
    }
  }
}

/// The font options passed to `load_fonts()`.
#[derive(Deserialize)]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JsCropOptions {
  /// The rectangle's left x-axis coordinate.
  ///
  /// Default: 0
  pub left: i32,

  /// The rectangle's top y-axis coordinate.
  ///
  /// Default: 0
  pub top: i32,

  /// The rectangle's right x-axis coordinate. `None` targets the svg width.
  ///
  /// Default: None
  pub right: Option<i32>,

  /// The rectangle's bottom y-axis coordinate. `None` targets the svg height.
  ///
  /// Default: None
  pub bottom: Option<i32>,
}

impl Default for JsCropOptions {
  fn default() -> JsCropOptions {
    JsCropOptions {
      left: 0,
      top: 0,
      right: None,
      bottom: None,
    }
  }
}

/// Deserializes `usvg::ShapeRendering`
fn deserialize_shape_rendering<'de, D>(deserializer: D) -> Result<usvg::ShapeRendering, D::Error>
where
  D: Deserializer<'de>,
{
  match u64::deserialize(deserializer)? {
        0 => Ok(usvg::ShapeRendering::OptimizeSpeed),
        1 => Ok(usvg::ShapeRendering::CrispEdges),
        2 => Ok(usvg::ShapeRendering::GeometricPrecision),
        n => Err(serde::de::Error::custom(format_args!(
            "Invalid ShapeRendering value: {}. Must be these numbers: 0 (OptimizeSpeed), 1 (CrispEdges), or 2 (GeometricPrecision).",
            n
        ))),
    }
}

/// Deserializes `usvg::TextRendering`
fn deserialize_text_rendering<'de, D>(deserializer: D) -> Result<usvg::TextRendering, D::Error>
where
  D: Deserializer<'de>,
{
  match u64::deserialize(deserializer)? {
        0 => Ok(usvg::TextRendering::OptimizeSpeed),
        1 => Ok(usvg::TextRendering::OptimizeLegibility),
        2 => Ok(usvg::TextRendering::GeometricPrecision),
        n => Err(serde::de::Error::custom(format_args!(
            "Invalid TextRendering value: {}. Must be these numbers: 0 (OptimizeSpeed), 1 (OptimizeLegibility), or 2 (GeometricPrecision).",
            n
        ))),
    }
}

/// Deserializes `usvg::ImageRendering`
fn deserialize_image_rendering<'de, D>(deserializer: D) -> Result<usvg::ImageRendering, D::Error>
where
  D: Deserializer<'de>,
{
  match u64::deserialize(deserializer)? {
    0 => Ok(usvg::ImageRendering::OptimizeQuality),
    1 => Ok(usvg::ImageRendering::OptimizeSpeed),
    n => Err(serde::de::Error::custom(format_args!(
      "Invalid ImageRendering value: {}. Must be these numbers: 0 (OptimizeQuality) or 1 (OptimizeSpeed).",
      n
    ))),
  }
}
