use core::slice::SlicePattern;
use std::sync::atomic::{AtomicU64, Ordering};

use hvm::{Ptr, ReduceCtx};

pub type ReduceContext = *mut ReduceCtx<'static>;
pub type Pointer = u64;
pub type Tag = u64;
pub type Position = u64;
pub type Host = *mut u64;

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__llvm_eq(a: u64, b: u64) -> bool {
    a == b
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__llvm_or(a: bool, b: bool) -> bool {
    a || b
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__llvm_and(a: bool, b: bool) -> bool {
    a && b
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__increment_cost(ctx: ReduceContext) {
    let ctx = get_context(ctx);

    hvm::runtime::inc_cost(ctx.heap, ctx.tid)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_host(ctx: ReduceContext) -> Host {
    let ctx = get_context(ctx);

    ctx.host
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_host_value(ctx: ReduceContext) -> Ptr {
    let ctx = get_context(ctx);

    *ctx.host
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_term(ctx: ReduceContext) -> Pointer {
    let ctx = get_context(ctx);

    ctx.term
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__link(
    ctx: ReduceContext,
    position: Position,
    pointer: Pointer,
) -> Pointer {
    let ctx = get_context(ctx);

    hvm::runtime::link(ctx.heap, position, pointer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__free(ctx: ReduceContext, position: Position, arity: u64) {
    let ctx = get_context(ctx);

    hvm::runtime::free(ctx.heap, ctx.tid, position, arity)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__insert_redex(ctx: ReduceContext, vlen: u64) -> u64 {
    let ctx = get_context(ctx);
    let redex = hvm::runtime::new_redex(*ctx.host, *ctx.cont, vlen);

    ctx.redex.insert(ctx.tid, redex)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__update_cont(ctx: ReduceContext, goup: u64) {
    let ctx = get_context(ctx);

    *ctx.cont = goup;
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__update_host(ctx: ReduceContext, vbuf: *mut AtomicU64, vlen: u64) {
    let ctx = get_context(ctx);
    let host = vbuf.add((vlen - 1) as usize).read().load(Ordering::Relaxed);

    *ctx.host = host;
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__visit(
    ctx: ReduceContext,
    parameter_index: u64,
    goup: u64,
    vbuf: *mut AtomicU64,
    vlen: u64,
) {
    let ctx = get_context(ctx);

    if parameter_index < vlen - 1 {
        let vbuf = vbuf
            .add(parameter_index as usize)
            .read()
            .load(Ordering::Relaxed);
        let visit = hvm::runtime::new_visit(vbuf, ctx.hold, goup);
        ctx.visit.push(visit);
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__increase_vlen(
    ctx: ReduceContext,
    parameter_index: u64,
    vbuf: *mut AtomicU64,
    vlen: u64,
) -> u64 {
    let ctx = get_context(ctx);

    if hvm::runtime::is_whnf(hvm::runtime::load_arg(ctx.heap, ctx.term, parameter_index)) {
        0
    } else {
        let atomic = vbuf.add(vlen as usize).read();
        let position = hvm::runtime::get_loc(ctx.term, 0);
        atomic.store(position, Ordering::Relaxed);

        1
    }
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_vbuf(ctx: ReduceContext) -> *mut &'static [AtomicU64] {
    let ctx = get_context(ctx);

    let vbuf = ctx.heap.vbuf.get_unchecked(ctx.tid);
    Box::into_raw(Box::new(vbuf.as_slice()))
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__alloc(ctx: ReduceContext, arity: u64) -> u64 {
    let ctx = get_context(ctx);

    hvm::runtime::alloc(ctx.heap, ctx.tid, arity)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__load_argument(
    ctx: ReduceContext,
    term: Pointer,
    index: u64,
) -> Pointer {
    let ctx = get_context(ctx);

    hvm::runtime::load_arg(ctx.heap, term, index)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_loc(pointer: Pointer, argument: Position) -> Pointer {
    hvm::runtime::get_loc(pointer, argument)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_ext(pointer: Pointer) -> Tag {
    hvm::runtime::get_ext(pointer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_tag(pointer: Pointer) -> Tag {
    hvm::runtime::get_tag(pointer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_number(pointer: Pointer) -> u64 {
    hvm::runtime::get_num(pointer)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_u60(value: u64) -> Pointer {
    hvm::runtime::U6O(value)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_constructor(fun: u64, position: Position) -> Pointer {
    hvm::runtime::Ctr(fun, position)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_erased() -> Pointer {
    hvm::runtime::Era()
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_binary(operand: Position, position: Position) -> Pointer {
    hvm::runtime::Op2(operand, position)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_app(position: Position) -> Pointer {
    hvm::runtime::App(position)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_var(position: Position) -> Pointer {
    hvm::runtime::Var(position)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_lam(position: Position) -> Pointer {
    hvm::runtime::Lam(position)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__create_function(fun: u64, position: Position) -> Pointer {
    hvm::runtime::Fun(fun, position)
}

fn get_context<'a>(ctx: ReduceContext) -> ReduceCtx<'a> {
    unsafe {
        if ctx.is_null() {
            panic!("Reduce context is null");
        } else {
            ctx.read()
        }
    }
}
