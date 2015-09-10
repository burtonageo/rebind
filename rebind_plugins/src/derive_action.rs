#![allow(unused_variables)]

use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn expand_derive_action_annotation(context: &mut ExtCtxt,
                                       span: Span,
                                       meta_item: &MetaItem,
                                       item: &Annotatable,
                                       push: &mut FnMut(Annotatable)) {
    // code goes here...
}