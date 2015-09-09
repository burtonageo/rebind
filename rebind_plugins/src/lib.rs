#![feature(rustc_private, plugin_registrar)]

extern crate rebind;
extern crate rustc;
extern crate syntax;

mod derive_action;

use rustc::plugin::Registry;
use syntax::parse::token::intern;
use syntax::ext::base::SyntaxExtension;

use derive_action::expand_derive_action_annotation;

#[plugin_registrar]
#[doc(hidden)]
pub fn plugin_registrar(registry: &mut Registry) {
    registry.register_syntax_extension(intern("derive_Action"),
                                       SyntaxExtension::MultiDecorator(
                                           Box::new(expand_derive_action_annotation)));
}