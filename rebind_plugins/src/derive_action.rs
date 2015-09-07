use syntax::ast;
use syntax::ast::{MetaItem, Expr};
use syntax::codemap::Span;
use syntax::ext::base::{Annotatable, ExtCtxt};
use syntax::ext::build::AstBuilder;
use syntax::ext::deriving::generic::{Struct, Substructure, TraitDef, ty};
use syntax::ext::deriving::generic::{combine_substructure, EnumMatching, FieldInfo, MethodDef};

pub fn expand_action_annotation(context: &mut ExtCtxt,
                                span: Span,
                                meta_item: &MetaItem,
                                item: &Annotatable,
                                push: &mut FnMut(Annotatable)) {

}