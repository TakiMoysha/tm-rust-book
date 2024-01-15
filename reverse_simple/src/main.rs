use sysinfo::System;


fn get_base_address(pid: i32) -> Option<vm_address_t> {
    unsafe {
        let mut address: vm_address_t = 0;
    }

    None 
}

fn patch_value(pid: i32, address: vm_address_t, value: u64) -> bool {
    false
}
/// # Find process
/// find the base address
/// base address -> searched value
/// patch searched value
fn main() {
    let mut system = System::new_all();
    system.refresh_all();

    let mut proc_pid: i32 = 0;
    let mut proc_name: &str = "";
    
    for process in system.processes_by_exact_name("htop") {
        proc_pid = process.pid().as_u32() as i32;
        proc_name = process.name();
    }

    println!("Target process: {:?} - {:?}", proc_name, proc_pid);


}
