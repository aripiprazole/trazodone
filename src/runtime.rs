use hvm::ReduceCtx;

pub type ReduceContext = *mut ReduceCtx<'static>;
pub type Pointer = u64;
pub type Tag = u64;
pub type Position = u64;
pub type Host = *mut u64;

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
pub unsafe extern "C" fn hvm__get_term(ctx: ReduceContext) -> Pointer {
    let ctx = get_context(ctx);

    ctx.term
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__link(ctx: ReduceContext, position: Position, pointer: Pointer) -> Pointer {
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
pub unsafe extern "C" fn hvm__alloc(ctx: ReduceContext, arity: u64) -> u64 {
    let ctx = get_context(ctx);

    hvm::runtime::alloc(ctx.heap, ctx.tid, arity)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__load_argument(ctx: ReduceContext, index: u64) -> Pointer {
    let ctx = get_context(ctx);

    hvm::runtime::load_arg(ctx.heap, ctx.term, index)
}

#[no_mangle]
#[allow(clippy::missing_safety_doc)]
pub unsafe extern "C" fn hvm__get_loc(pointer: Pointer, argument: Position) -> Pointer {
    hvm::runtime::get_loc(pointer, argument)
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

fn get_context<'a>(ctx: ReduceContext) -> ReduceCtx<'a> {
    unsafe {
        if ctx.is_null() {
            panic!("Reduce context is null");
        } else {
            ctx.read()
        }
    }
}
