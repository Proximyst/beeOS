use ::x86_64::structures::idt::{
    InterruptDescriptorTable,
    InterruptStackFrame,
};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault);
        idt
    };
}

/// Upon a double fault, this will be called and panic the entire kernel.
extern "x86-interrupt" fn double_fault(
    stack: &mut InterruptStackFrame,
    err: u64,
) {
    println!("CPU EXCEPTION ({}): DOUBLE FAULT", err);
    panic!("{:#?}", stack);
}
