#![allow(unused_variables)]

use syntax::ast;
use syntax::ast::MetaItem;
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};

pub fn expand_derive_action_annotation(context: &mut ExtCtxt,
                                       span: Span,
                                       meta_item: &MetaItem,
                                       item: &Annotatable,
                                       push: &mut FnMut(Annotatable)) {
/*
    match item.node {
        ast::ItemEnum(def, _) => {
            // add definitions
        },
        _ => {context.span_err(span, "dummy is only permissiable on functions");}
    }
*/
}