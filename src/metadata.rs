use derivative::Derivative;

/// Placeholder type for elements that don't raise events.
///
/// Equivalent to `!`.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum NoEvent {}

/// Placeholder type, for elements that don't require their component's local state.
///
/// (This is used by all elements except ComponentOutput; this is basically the type-system equivalent of a sentinel value.)
#[derive(Default, Debug, Clone, PartialEq, Eq, Hash)]
pub struct NoState;

/// Marker type used by component scaffholding.
///
/// Instances of this type can only be returned by [`CompCtx::use_metadata`](crate::CompCtx::use_metadata).
///
/// **Note:** This is a unit type; it stores no state, and is only passed around to methods
/// as a concise way to bind type information.
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
