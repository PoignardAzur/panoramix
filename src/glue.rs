//! Bunch of types mostly stolen from Crochet, to use as glue with Druid

#![allow(unused)]

use druid::Data;
use std::collections::HashMap;
use std::sync::Arc;

pub struct GlobalEventCx<'a> {
    pub app_data: &'a mut DruidAppData,
}

impl<'a> GlobalEventCx<'a> {
    pub fn new(app_data: &'a mut DruidAppData) -> Self {
        Self { app_data }
    }
}

/// The type we use for app data for Druid integration.
///
/// Currently this is action queues.
///
/// It should probably be a vec of actions, but we can refine
/// later. For button clicks it doesn't matter.
#[derive(Clone, Data, Default)]
pub struct DruidAppData(Arc<HashMap<Id, Action>>);

/// Actions that can be produced by widgets,
#[derive(Clone)]
pub enum Action {
    Clicked,
    FutureResolved,
}

impl DruidAppData {
    pub(crate) fn queue_action(&mut self, id: Id, action: Action) {
        Arc::make_mut(&mut self.0).insert(id, action);
    }

    pub(crate) fn dequeue_action(&mut self, id: Id) -> Option<Action> {
        if self.0.contains_key(&id) {
            Arc::make_mut(&mut self.0).remove(&id)
        } else {
            None
        }
    }

    /// Report whether the id has a non-empty action queue.
    pub(crate) fn has_action(&self, id: Id) -> bool {
        self.0.contains_key(&id)
    }
}

// ---

use std::sync::atomic::{AtomicUsize, Ordering};

static ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

/// An identifier for an element.
///
/// It's a bit heavy-handed to have this id as well as widget
/// id in Druid; likely the two concepts should be unified. But
#[derive(Clone, Copy, Debug, PartialOrd, PartialEq, Ord, Eq, Hash)]
pub struct Id(usize);

impl Id {
    /// Allocate a new unique id.
    pub fn new() -> Id {
        Id(ID_COUNTER.fetch_add(1, Ordering::Relaxed))
    }
}

// ---

use tracing_subscriber::prelude::*;
use tracing_subscriber::{fmt, EnvFilter};
use tracing_unwrap::ResultExt;

pub fn init_tracing() {
    let fmt_layer = fmt::layer().with_target(true);
    let filter_layer = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("warn"))
        .unwrap_or_log();

    tracing_subscriber::registry()
        .with(filter_layer)
        .with(fmt_layer)
        .init();
}
