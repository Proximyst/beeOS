use ::x86_64::structures::idt::{ExceptionStackFrame, Idt};

lazy_static! {
    pub static ref IDT: Idt = {
        let mut idt = Idt::new();
        idt.breakpoint.set_handler_fn(breakpoint);
        idt
    };
}

extern "x86-interrupt" fn breakpoint(stack: &mut ExceptionStackFrame) {
    println!("CPU EXCEPTION: BREAKPOINT");
    println!("{:#?}", stack);
}
