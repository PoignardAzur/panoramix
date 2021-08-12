//! This file is copy-n-pasted from the Druid Flex widget implementation,
//! with some modifications to support tree mutation.

//! A widget that arranges its children in a one-dimensional array.

use druid::kurbo::common::FloatExt;
use druid::kurbo::{Point, Rect, Size};

use crate::glue::DebugState;
use druid::{
    BoxConstraints, Env, Event, EventCtx, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, UpdateCtx,
    Widget,
};

use crate::flex::{Axis, CrossAxisAlignment, FlexContainerParams, MainAxisAlignment};
use crate::widget_sequence::WidgetSequence;

use tracing::trace;

/// A container Druid widget, copy-pasted from Druid with some modifications
pub struct FlexWidget<Children: WidgetSequence> {
    pub(crate) direction: Axis,
    pub(crate) flex_params: FlexContainerParams,
    pub children_seq: Children,
}

use crate::glue::DruidAppData;
impl<Children: WidgetSequence> Widget<DruidAppData> for FlexWidget<Children> {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut DruidAppData, env: &Env) {
        // FIXME
        ctx.children_changed();

        for child in self.children_seq.widgets_mut() {
            child.event(ctx, event, data, env);
        }
    }

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &DruidAppData,
        env: &Env,
    ) {
        for child in self.children_seq.widgets_mut() {
            child.lifecycle(ctx, event, data, env);
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &DruidAppData,
        data: &DruidAppData,
        env: &Env,
    ) {
        for child in self.children_seq.widgets_mut() {
            child.update(ctx, old_data, data, env);
        }
    }

    fn layout(
        &mut self,
        ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        data: &DruidAppData,
        env: &Env,
    ) -> Size {
        use log::warn;
        bc.debug_check("Flex");
        // we loosen our constraints when passing to children.
        let loosened_bc = bc.loosen();

        // minor-axis values for all children
        // these two are calculated but only used if we're baseline aligned
        let mut max_above_baseline = 0f64;
        let mut max_below_baseline = 0f64;
        let mut any_use_baseline = self.flex_params.cross_alignment == CrossAxisAlignment::Baseline;

        let mut child_widgets = self.children_seq.widgets_mut();

        // Measure non-flex children.
        let mut major_non_flex = 0.0;
        let mut minor = self.direction.minor(bc.min());
        let mut flex_sum = 0.0;
        for child in &mut child_widgets {
            if let Some(flex) = child.flex_params().flex {
                flex_sum += flex
            } else {
                let child_bc = self
                    .direction
                    .constraints(&loosened_bc, 0., std::f64::INFINITY);
                let alignment = child.flex_params().alignment;
                any_use_baseline &= alignment == Some(CrossAxisAlignment::Baseline);

                let child_size = child.layout(ctx, &child_bc, data, env);
                let baseline_offset = child.baseline_offset();

                if child_size.width.is_infinite() {
                    warn!("A non-Flex child has an infinite width.");
                }

                if child_size.height.is_infinite() {
                    warn!("A non-Flex child has an infinite height.");
                }

                major_non_flex += self.direction.major(child_size).expand();
                minor = minor.max(self.direction.minor(child_size).expand());
                max_above_baseline = max_above_baseline.max(child_size.height - baseline_offset);
                max_below_baseline = max_below_baseline.max(baseline_offset);
            }
        }

        let total_major = self.direction.major(bc.max());
        let remaining = (total_major - major_non_flex).max(0.0);
        let mut remainder: f64 = 0.0;

        let mut major_flex: f64 = 0.0;
        let px_per_flex = remaining / flex_sum;
        // Measure flex children.
        for child in &mut child_widgets {
            if let Some(flex) = child.flex_params().flex {
                let desired_major = flex * px_per_flex + remainder;
                let actual_major = desired_major.round();
                remainder = desired_major - actual_major;

                let child_bc = self.direction.constraints(&loosened_bc, 0.0, actual_major);
                let child_size = child.layout(ctx, &child_bc, data, env);
                let baseline_offset = child.baseline_offset();

                major_flex += self.direction.major(child_size).expand();
                minor = minor.max(self.direction.minor(child_size).expand());
                max_above_baseline = max_above_baseline.max(child_size.height - baseline_offset);
                max_below_baseline = max_below_baseline.max(baseline_offset);
            }
        }

        // figure out if we have extra space on major axis, and if so how to use it
        let extra = if self.flex_params.fill_major_axis {
            (remaining - major_flex).max(0.0)
        } else {
            // if we are *not* expected to fill our available space this usually
            // means we don't have any extra, unless dictated by our constraints.
            (self.direction.major(bc.min()) - (major_non_flex + major_flex)).max(0.0)
        };

        let mut spacing = Spacing::new(self.flex_params.main_alignment, extra, child_widgets.len());

        // the actual size needed to tightly fit the children on the minor axis.
        // Unlike the 'minor' var, this ignores the incoming constraints.
        let minor_dim = match self.direction {
            Axis::Horizontal if any_use_baseline => max_below_baseline + max_above_baseline,
            _ => minor,
        };

        let extra_height = minor - minor_dim.min(minor);

        let mut major = spacing.next().unwrap_or(0.);
        let mut child_paint_rect = Rect::ZERO;
        for child in &mut child_widgets {
            let child_size = child.layout_rect().size();
            let alignment = child
                .flex_params()
                .alignment
                .unwrap_or(self.flex_params.cross_alignment);
            let child_minor_offset = match alignment {
                // This will ignore baseline alignment if it is overridden on children,
                // but is not the default for the container. Is this okay?
                CrossAxisAlignment::Baseline if matches!(self.direction, Axis::Horizontal) => {
                    let child_baseline = child.baseline_offset();
                    let child_above_baseline = child_size.height - child_baseline;
                    extra_height + (max_above_baseline - child_above_baseline)
                }
                CrossAxisAlignment::Fill => {
                    let fill_size: Size = self
                        .direction
                        .pack(self.direction.major(child_size), minor_dim)
                        .into();
                    let child_bc = BoxConstraints::tight(fill_size);
                    child.layout(ctx, &child_bc, data, env);
                    0.0
                }
                _ => {
                    let extra_minor = minor_dim - self.direction.minor(child_size);
                    alignment.align(extra_minor)
                }
            };

            let child_pos: Point = self.direction.pack(major, child_minor_offset).into();
            child.set_origin(ctx, data, env, child_pos);
            child_paint_rect = child_paint_rect.union(child.paint_rect());
            major += self.direction.major(child_size).expand();
            major += spacing.next().unwrap_or(0.);
        }

        if flex_sum > 0.0 && total_major.is_infinite() {
            warn!("A child of Flex is flex, but Flex is unbounded.")
        }

        if flex_sum > 0.0 {
            major = total_major;
        }

        let my_size: Size = self.direction.pack(major, minor_dim).into();

        // if we don't have to fill the main axis, we loosen that axis before constraining
        let my_size = if !self.flex_params.fill_major_axis {
            let max_major = self.direction.major(bc.max());
            self.direction
                .constraints(bc, 0.0, max_major)
                .constrain(my_size)
        } else {
            bc.constrain(my_size)
        };

        let my_bounds = Rect::ZERO.with_size(my_size);
        let insets = child_paint_rect - my_bounds;
        ctx.set_paint_insets(insets);

        let baseline_offset = match self.direction {
            Axis::Horizontal => max_below_baseline,
            Axis::Vertical => {
                if let Some(child) = &child_widgets.last() {
                    let child_bl = child.baseline_offset();
                    let child_max_y = child.layout_rect().max_y();
                    let extra_bottom_padding = my_size.height - child_max_y;
                    child_bl + extra_bottom_padding
                } else {
                    0.0
                }
            }
        };

        ctx.set_baseline_offset(baseline_offset);
        trace!(
            "Computed layout: size={}, baseline_offset={}",
            my_size,
            baseline_offset
        );
        my_size
    }

    fn paint(&mut self, ctx: &mut PaintCtx, data: &DruidAppData, env: &Env) {
        for child in self.children_seq.widgets_mut() {
            child.paint(ctx, data, env);
        }
    }

    fn debug_state(&self, data: &DruidAppData) -> DebugState {
        let children_state = self
            .children_seq
            .widgets()
            .iter()
            .map(|child| child.debug_state(data))
            .collect();

        let name = match self.direction {
            Axis::Horizontal => "Row",
            Axis::Vertical => "Column",
        };

        DebugState {
            display_name: name.to_string(),
            children: children_state,
            ..Default::default()
        }
    }
}

impl CrossAxisAlignment {
    /// Given the difference between the size of the container and the size
    /// of the child (on their minor axis) return the necessary offset for
    /// this alignment.
    fn align(self, val: f64) -> f64 {
        match self {
            CrossAxisAlignment::Start => 0.0,
            // in vertical layout, baseline is equivalent to center
            CrossAxisAlignment::Center | CrossAxisAlignment::Baseline => (val / 2.0).round(),
            CrossAxisAlignment::End => val,
            CrossAxisAlignment::Fill => 0.0,
        }
    }
}

pub struct Spacing {
    alignment: MainAxisAlignment,
    extra: f64,
    n_children: usize,
    index: usize,
    equal_space: f64,
    remainder: f64,
}

impl Spacing {
    /// Given the provided extra space and children count,
    /// this returns an iterator of `f64` spacing,
    /// where the first element is the spacing before any children
    /// and all subsequent elements are the spacing after children.
    fn new(alignment: MainAxisAlignment, extra: f64, n_children: usize) -> Spacing {
        let extra = if extra.is_finite() { extra } else { 0. };
        let equal_space = if n_children > 0 {
            match alignment {
                MainAxisAlignment::Center => extra / 2.,
                MainAxisAlignment::SpaceBetween => extra / (n_children - 1).max(1) as f64,
                MainAxisAlignment::SpaceEvenly => extra / (n_children + 1) as f64,
                MainAxisAlignment::SpaceAround => extra / (2 * n_children) as f64,
                _ => 0.,
            }
        } else {
            0.
        };
        Spacing {
            alignment,
            extra,
            n_children,
            index: 0,
            equal_space,
            remainder: 0.,
        }
    }

    fn next_space(&mut self) -> f64 {
        let desired_space = self.equal_space + self.remainder;
        let actual_space = desired_space.round();
        self.remainder = desired_space - actual_space;
        actual_space
    }
}

impl Iterator for Spacing {
    type Item = f64;

    fn next(&mut self) -> Option<f64> {
        if self.index > self.n_children {
            return None;
        }
        let result = {
            if self.n_children == 0 {
                self.extra
            } else {
                #[allow(clippy::match_bool)]
                match self.alignment {
                    MainAxisAlignment::Start => match self.index == self.n_children {
                        true => self.extra,
                        false => 0.,
                    },
                    MainAxisAlignment::End => match self.index == 0 {
                        true => self.extra,
                        false => 0.,
                    },
                    MainAxisAlignment::Center => match self.index {
                        0 => self.next_space(),
                        i if i == self.n_children => self.next_space(),
                        _ => 0.,
                    },
                    MainAxisAlignment::SpaceBetween => match self.index {
                        0 => 0.,
                        i if i != self.n_children => self.next_space(),
                        _ => match self.n_children {
                            1 => self.next_space(),
                            _ => 0.,
                        },
                    },
                    MainAxisAlignment::SpaceEvenly => self.next_space(),
                    MainAxisAlignment::SpaceAround => {
                        if self.index == 0 || self.index == self.n_children {
                            self.next_space()
                        } else {
                            self.next_space() + self.next_space()
                        }
                    }
                }
            }
        };
        self.index += 1;
        Some(result)
    }
}
