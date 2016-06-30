#![feature(plugin_registrar, rustc_private)]

#[macro_use]
extern crate lazy_static;
extern crate rustc;
extern crate syntax;
use std::time::{UNIX_EPOCH, SystemTime};

lazy_static! {
    static ref EPOCH: u64 = match SystemTime::now().duration_since(UNIX_EPOCH) {
        Ok(dur) => dur,
        Err(err) => err.duration(),
    }.as_secs();
}

use rustc::plugin::Registry;
use syntax::codemap::Span;
use syntax::ast::{IntTy, Lit_, LitIntType, Sign, TokenTree};
use syntax::ext::base::{DummyResult, ExtCtxt, MacEager, MacResult};
use syntax::ext::build::AstBuilder;

#[plugin_registrar]
pub fn plugin_registrar(reg: &mut Registry) {
    reg.register_macro("build_epoch", build_epoch);
}

fn build_epoch<'a>(cx: &'a mut ExtCtxt, span: Span, tts: &[TokenTree]) -> Box<MacResult + 'a> {
    if !tts.is_empty() {
        cx.span_err(span, "macro takes no arguments");
        return DummyResult::any(span);
    }

    let lit = Lit_::LitInt(*EPOCH as u64, LitIntType::SignedIntLit(IntTy::TyI64, Sign::Plus));
    MacEager::expr(cx.expr_lit(span, lit))
}
