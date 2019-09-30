use ::x86_64::structures::idt::{InterruptStackFrame, InterruptDescriptorTable};

lazy_static! {
    pub static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.double_fault.set_handler_fn(double_fault);
        idt
    };
}

extern "x86-interrupt" fn double_fault(stack: &mut InterruptStackFrame, err: u64) {
    println!("CPU EXCEPTION ({}): DOUBLE FAULT", err);
    panic!("{:#?}", stack);
}