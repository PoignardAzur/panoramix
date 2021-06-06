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

use crate::widget_sequence::WidgetSequence;

// TODO
use crate::flex::*;

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

        // Measure non-flex children.
        let mut major_non_flex = 0.0;
        let mut minor = self.direction.minor(bc.min());
        let mut child_widgets = self.children_seq.widgets_mut();
        for child in &mut child_widgets {
            if child.flex_params().flex == 0.0 {
                let child_bc = self
                    .direction
                    .constraints(&loosened_bc, 0., std::f64::INFINITY);
                let child_size = child.layout(ctx, &child_bc, data, env);

                if child_size.width.is_infinite() {
                    warn!("A non-Flex child has an infinite width.");
                }

                if child_size.height.is_infinite() {
                    warn!("A non-Flex child has an infinite height.");
                }

                major_non_flex += self.direction.major(child_size).expand();
                minor = minor.max(self.direction.minor(child_size).expand());
                // Stash size.
                let rect = Rect::from_origin_size(Point::ORIGIN, child_size);
                child.set_layout_rect(ctx, data, env, rect);
            }
        }

        let total_major = self.direction.major(bc.max());
        let remaining = (total_major - major_non_flex).max(0.0);
        let mut remainder: f64 = 0.0;
        let flex_sum: f64 = child_widgets
            .iter()
            .map(|child| child.flex_params().flex)
            .sum();
        let mut major_flex: f64 = 0.0;

        // Measure flex children.
        for child in &mut child_widgets {
            if child.flex_params().flex != 0.0 {
                let desired_major = remaining * child.flex_params().flex / flex_sum + remainder;
                let actual_major = desired_major.round();
                remainder = desired_major - actual_major;
                let min_major = 0.0;

                let child_bc = self
                    .direction
                    .constraints(&loosened_bc, min_major, actual_major);
                let child_size = child.layout(ctx, &child_bc, data, env);

                major_flex += self.direction.major(child_size).expand();
                minor = minor.max(self.direction.minor(child_size).expand());
                // Stash size.
                let rect = Rect::from_origin_size(Point::ORIGIN, child_size);
                child.set_layout_rect(ctx, data, env, rect);
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
        // Finalize layout, assigning positions to each child.
        let mut major = spacing.next().unwrap_or(0.);
        let mut child_paint_rect = Rect::ZERO;
        for child in child_widgets {
            let rect = child.layout_rect();
            let extra_minor = minor - self.direction.minor(rect.size());
            let alignment = child
                .flex_params()
                .alignment
                .unwrap_or(self.flex_params.cross_alignment);
            let align_minor = alignment.align(extra_minor);
            let pos: Point = self.direction.pack(major, align_minor).into();

            child.set_layout_rect(ctx, data, env, rect.with_origin(pos));
            child_paint_rect = child_paint_rect.union(child.paint_rect());
            major += self.direction.major(rect.size()).expand();
            major += spacing.next().unwrap_or(0.);
        }

        if flex_sum > 0.0 && total_major.is_infinite() {
            warn!("A child of Flex is flex, but Flex is unbounded.")
        }

        if flex_sum > 0.0 {
            major = total_major;
        }

        let my_size: Size = self.direction.pack(major, minor).into();

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
            CrossAxisAlignment::Center => (val / 2.0).round(),
            CrossAxisAlignment::End => val,
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
