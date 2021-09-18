use vm::VM;

pub fn vm_chunk(vm: &VM) {
    crate::chunk::disassemble(&vm.chunk);
}

pub fn vm_stack(vm: &VM) {
    println!("== Stack ==");
    let stack = &vm.stack;
    for idx in (0..stack.len()).rev() {
        println!("{}\t{}", idx, stack[idx]);
    }
}