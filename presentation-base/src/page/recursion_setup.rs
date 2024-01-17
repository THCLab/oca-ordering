use recursion::{Expandable, MappableFrame, PartiallyApplied};

use super::PageElement;

pub enum PageElementFrame<A> {
    Value(String),
    Page {
        name: String,
        attribute_order: Vec<A>,
    },
}

impl MappableFrame for PageElementFrame<PartiallyApplied> {
    type Frame<X> = PageElementFrame<X>;

    fn map_frame<A, B>(input: Self::Frame<A>, f: impl FnMut(A) -> B) -> Self::Frame<B> {
        match input {
            PageElementFrame::Value(v) => PageElementFrame::Value(v),
            PageElementFrame::Page {
                name,
                attribute_order,
            } => PageElementFrame::Page {
                name,
                attribute_order: attribute_order.into_iter().map(f).collect(),
            },
        }
    }
}

impl Expandable for PageElement {
    type FrameToken = PageElementFrame<PartiallyApplied>;

    fn from_frame(val: <Self::FrameToken as MappableFrame>::Frame<Self>) -> Self {
        match val {
            PageElementFrame::Value(v) => PageElement::Value(v),
            PageElementFrame::Page {
                name,
                attribute_order,
            } => PageElement::Page {
                name,
                attribute_order,
            },
        }
    }
}
