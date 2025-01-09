use unicorn_engine::{Arch, Mode, Permission, RegisterX86, Unicorn};

const X86_CODE64: &[u8] = b"\x48\x31\xc0\x48\x31\xff\x0f\x05";

const ADDRESS: u64 = 0x400000;

fn execute() {
    let mut emu =
        Unicorn::new(Arch::X86, Mode::MODE_64).expect("Failed to create and init unicorn");

    emu.mem_map(ADDRESS, 0x1000, Permission::READ | Permission::WRITE)
        .expect("Failed to map memory");

    let r_ecx = 0x12345678;
    let r_edx = 0x87654321;

    println!("Write 0x{:x} to ECX", r_ecx);
    println!("Write 0x{:x} to EDX", r_edx);

    emu.reg_write(RegisterX86::ECX, r_ecx)
        .expect("Failed to write ECX");
    emu.reg_write(RegisterX86::EDX, r_edx)
        .expect("Failed to write EDX");

    emu.emu_start(ADDRESS, ADDRESS + X86_CODE32.len() as u64, 0, 0)
        .expect("Failed to start emulation");

    println!("Read ECX: 0x{:x}", emu.reg_read(RegisterX86::ECX).unwrap());
    println!("Read EDX: 0x{:x}", emu.reg_read(RegisterX86::EDX).unwrap());
}
