//! Types used by Panoramix to lay out widgets.
//!
//! Most of these type are copy-pasted straight from Druid's code, with some minor modifications. That said, they may change in future versions and aren't intended to be API-compatible with Druid.

use druid::kurbo::Size;
use druid::BoxConstraints;

use druid::{Color, KeyOrValue};

#[derive(Debug, Clone, PartialEq)]
pub struct BorderStyle {
    pub width: KeyOrValue<f64>,
    pub color: KeyOrValue<Color>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct ContainerStyle {
    pub background: Option<KeyOrValue<Color>>,
    pub border: Option<BorderStyle>,
    pub corner_radius: KeyOrValue<f64>,
}

/// Optional parameters for a [Flex](crate::elements::Flex) container (row or column).
///
/// See [Flex::with_flex_container_params](crate::elements::Flex::with_flex_container_params).
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct FlexContainerParams {
    pub cross_alignment: CrossAxisAlignment,
    pub main_alignment: MainAxisAlignment,
    pub fill_major_axis: bool,
}

/// Optional parameters for an item in a [Flex](crate::elements::Flex) container (row or column).
///
/// Elements that represent a single flex item generally have a `with_flex_params` method that you can pass this struct to.
#[derive(Copy, Clone, Default, Debug, PartialEq)]
pub struct FlexParams {
    pub flex: f64,
    pub alignment: Option<CrossAxisAlignment>,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Axis {
    Horizontal,
    Vertical,
}

/// The alignment of the widgets on the container's cross (or minor) axis.
///
/// If a widget is smaller than the container on the minor axis, this determines
/// where it is positioned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CrossAxisAlignment {
    /// Top or leading.
    ///
    /// In a vertical container, widgets are top aligned. In a horiziontal
    /// container, their leading edges are aligned.
    Start,
    /// Widgets are centered in the container.
    Center,
    /// Bottom or trailing.
    ///
    /// In a vertical container, widgets are bottom aligned. In a horiziontal
    /// container, their trailing edges are aligned.
    End,
}

/// Arrangement of children on the main axis.
///
/// If there is surplus space on the main axis after laying out children, this
/// enum represents how children are laid out in this space.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MainAxisAlignment {
    /// Top or leading.
    ///
    /// Children are aligned with the top or leading edge, without padding.
    Start,
    /// Children are centered, without padding.
    Center,
    /// Bottom or trailing.
    ///
    /// Children are aligned with the bottom or trailing edge, without padding.
    End,
    /// Extra space is divided evenly between each child.
    SpaceBetween,
    /// Extra space is divided evenly between each child, as well as at the ends.
    SpaceEvenly,
    /// Space between each child, with less at the start and end.
    ///
    /// This divides space such that each child is separated by `n` units,
    /// and the start and end have `n/2` units of padding.
    SpaceAround,
}

impl FlexParams {
    /// Create custom `FlexParams` with a specific `flex_factor` and an optional
    /// [`CrossAxisAlignment`].
    ///
    /// You likely only need to create these manually if you need to specify
    /// a custom alignment; if you only need to use a custom `flex_factor` you
    /// can pass an `f64` to any of the functions that take `FlexParams`.
    ///
    /// By default, the widget uses the alignment of its parent [Flex](crate::elements::Flex) container.
    pub fn new(flex: f64, alignment: impl Into<Option<CrossAxisAlignment>>) -> Self {
        FlexParams {
            flex,
            alignment: alignment.into(),
        }
    }
}

impl Axis {
    pub(crate) fn major(self, coords: Size) -> f64 {
        match self {
            Axis::Horizontal => coords.width,
            Axis::Vertical => coords.height,
        }
    }

    pub(crate) fn minor(self, coords: Size) -> f64 {
        match self {
            Axis::Horizontal => coords.height,
            Axis::Vertical => coords.width,
        }
    }

    pub(crate) fn pack(self, major: f64, minor: f64) -> (f64, f64) {
        match self {
            Axis::Horizontal => (major, minor),
            Axis::Vertical => (minor, major),
        }
    }

    /// Generate constraints with new values on the major axis.
    pub(crate) fn constraints(
        self,
        bc: &BoxConstraints,
        min_major: f64,
        major: f64,
    ) -> BoxConstraints {
        match self {
            Axis::Horizontal => BoxConstraints::new(
                Size::new(min_major, bc.min().height),
                Size::new(major, bc.max().height),
            ),
            Axis::Vertical => BoxConstraints::new(
                Size::new(bc.min().width, min_major),
                Size::new(bc.max().width, major),
            ),
        }
    }
}

impl From<f64> for FlexParams {
    fn from(flex: f64) -> FlexParams {
        FlexParams {
            flex,
            alignment: None,
        }
    }
}
