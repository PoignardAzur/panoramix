use derivative::Derivative;

/// Placeholder type for elements that don't raise events.
///
/// Equivalent to `!`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoState;

#[derive(Derivative)]
#[derivative(
    Clone(bound = ""),
    Copy(bound = ""),
    Default(bound = ""),
    PartialEq(bound = ""),
    Eq(bound = ""),
    Hash(bound = "")
)]
pub struct Metadata<ComponentEvent, ComponentState> {
    _marker: std::marker::PhantomData<(ComponentEvent, ComponentState)>,
}

impl<ComponentEvent, ComponentState> Metadata<ComponentEvent, ComponentState> {
    pub fn new() -> Self {
        Default::default()
    }
}
