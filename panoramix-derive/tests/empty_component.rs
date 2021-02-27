use panoramix::elements::EmptyElement;
use panoramix::{ElementTree, NoEvent};

use panoramix_derive::component;

#[component]
fn EmptyComponent(_state: &(), _my_props: ()) -> impl ElementTree<(), NoEvent> {
    EmptyElement::new()
}

fn main() {
    let element = EmptyComponent::new(());
    println!("element = {:?}", element);
}
