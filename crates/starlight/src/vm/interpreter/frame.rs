use std::mem::size_of;

use gccjit_rs::{ctx::Context, ty::Type, ty::Typeable};
use wtf_rs::round_up;

use crate::{
    gc::cell::{GcPointer, Trace, Tracer},
    vm::{code_block::CodeBlock, environment::Environment, value::JsValue},
};

impl Typeable for CallFrame {
    fn get_type(ctx: &Context) -> Type {
        let prev = ctx.new_field(None, ctx.new_type::<()>().make_pointer(), "prev");
        let sp = ctx.new_field(None, ctx.new_type::<u64>().make_pointer(), "sp");
        let limit = ctx.new_field(None, ctx.new_type::<u64>().make_pointer(), "limit");
        let ip = ctx.new_field(None, ctx.new_type::<u8>().make_pointer(), "ip");
        let code_block = ctx.new_field(None, ctx.new_type::<()>().make_pointer(), "code_block");
        let this = ctx.new_field(None, ctx.new_type::<u64>(), "this");
        let ctor = ctx.new_field(None, ctx.new_type::<bool>(), "ctor");
        let exit_on_return = ctx.new_field(None, ctx.new_type::<bool>(), "exit_on_return");
        let env = ctx.new_field(None, ctx.new_type::<u64>(), "env");

        ctx.new_struct_type(
            None,
            "CallFrame",
            &[
                prev,
                sp,
                limit,
                ip,
                code_block,
                this,
                ctor,
                exit_on_return,
                env,
            ],
        )
        .as_type()
    }
}

#[repr(C, align(8))]
pub struct CallFrame {
    pub prev: *mut CallFrame,
    pub sp: *mut JsValue,
    pub limit: *mut JsValue,
    pub callee: JsValue,
    pub ip: *mut u8,
    pub code_block: Option<GcPointer<CodeBlock>>,
    pub this: JsValue,
    pub ctor: bool,
    pub exit_on_return: bool,
    pub env: Option<GcPointer<Environment>>,
    /// (Environment,Instruction) stack
    pub try_stack: Vec<(Option<GcPointer<Environment>>, *mut u8, *mut JsValue)>,
    pub locals_start: *mut JsValue,
}
impl CallFrame {
    pub unsafe fn get_iloc(&self, at: u32) -> JsValue {
        self.locals_start.add(at as usize).read()
    }
    pub unsafe fn get_iloc_ptr(&self, at: u32) -> *mut JsValue {
        self.locals_start.add(at as usize)
    }
    pub unsafe fn set_iloc(&mut self, at: u32, val: JsValue) {
        self.locals_start.add(at as usize).write(val);
    }
    #[inline(always)]
    pub unsafe fn pop(&mut self) -> JsValue {
        if self.sp <= self.limit {
            //panic!("stack underflow");
        }
        self.sp = self.sp.sub(1);
        self.sp.read()
    }
    #[inline]
    pub unsafe fn at(&mut self, index: isize) -> &mut JsValue {
        &mut *self.sp.offset(index)
    }
    #[inline(always)]
    pub unsafe fn push(&mut self, val: JsValue) {
        self.sp.write(val);
        self.sp = self.sp.add(1);
    }
}
unsafe impl Trace for CallFrame {
    fn trace(&mut self, visitor: &mut dyn Tracer) {
        self.callee.trace(visitor);
        self.code_block.trace(visitor);
        self.this.trace(visitor);
        self.env.trace(visitor);
        for (env, _, _) in self.try_stack.iter_mut() {
            env.trace(visitor);
        }
    }
}

pub const FRAME_SIZE: usize =
    round_up(size_of::<CallFrame>(), size_of::<JsValue>()) / size_of::<JsValue>();
