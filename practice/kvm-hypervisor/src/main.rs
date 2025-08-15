use nix::sys::mman;
use std::ptr;
use std::{os::raw::c_void, ptr::NonNull};

use kvm_ioctls::Kvm;

const GUEST_PHYS_ADDR: u64 = 0x1000;
const GUEST_SIZE: usize = 256 << 20;
const GDT_OFFSET: usize = 0x0;
const PML4_OFFSET: usize = 0x1000;
const PAGE_TABLE_SIZE: usize = 0x1000;

pub struct UnderVm {
    vcpu: kvm_ioctls::VcpuFd,
    guest_mem: NonNull<c_void>,
}

impl UnderVm {
    pub fn new() -> anyhow::Result<Self> {
        let kvm = Kvm::new()?;
        let vm = kvm.create_vm()?;
        let vcpu = vm.create_vcpu(0)?;

        // mapping memory from hypervisor's to guest VM
        let guest_mem = unsafe {
            mman::mmap_anonymous(
                None,
                GUEST_SIZE.try_into()?,
                mman::ProtFlags::PROT_READ | mman::ProtFlags::PROT_WRITE,
                mman::MapFlags::MAP_PRIVATE | mman::MapFlags::MAP_ANONYMOUS,
            )?
        };
        let mem_region = kvm_bindings::kvm_userspace_memory_region {
            guest_phys_addr: GUEST_PHYS_ADDR,
            memory_size: GUEST_SIZE as u64,
            userspace_addr: guest_mem.as_ptr() as u64,
            ..Default::default()
        };

        unsafe {
            vm.set_user_memory_region(mem_region)?;
        }

        let mut me = Self { vcpu, guest_mem };

        me.setup_long_mode()?;
        me.map_pages()?;
        me.init_reigsters()?;

        Ok(me)
    }

    fn setup_long_mode(&mut self) -> anyhow::Result<()> {
        let mut sregs = self.vcpu.get_sregs()?;

        sregs.gdt.base = GUEST_PHYS_ADDR + GDT_OFFSET as u64;
        sregs.gdt.limit = 3 * 8 - 1;

        unsafe {
            let gdt_ptr = self.guest_mem.as_ptr().add(GDT_OFFSET) as *mut u64;

            // null descriptor
            *gdt_ptr.add(0) = 0x0000000000000000;
            // code segment (64-bit, executable, present)
            *gdt_ptr.add(1) = 0x00209A0000000000;
            // data segment (64-bit, writable, present)
            *gdt_ptr.add(2) = 0x0000920000000000;
        }

        // Configure code segment for long mode
        sregs.cs.base = 0; // Base address for the segment (ignored in long mode)
        sregs.cs.limit = 0xffffffff; // Limit (also ignored in long mode)
        sregs.cs.selector = 1 << 3; // GDT index
        sregs.cs.present = 1; // Segment is present
        sregs.cs.type_ = 11; // Code: execute/read, accessed
        sregs.cs.dpl = 0; // Descriptor privilege level (kernel ring 0)
        sregs.cs.db = 0; // Must be 0 in long mode (64-bit code)
        sregs.cs.s = 1; // Descriptor type: 1 = Code / Data (0 = system)
        sregs.cs.l = 1; // Long mode active (64-bit segment)
        sregs.cs.g = 1; // Granularity = 4 KB units (ignored in long mode, but set anyway)

        // Configure data segments
        sregs.ds.base = 0; // Base address for the segment (ignored in long mode)
        sregs.ds.limit = 0xffffffff; // Limit (also ignored in long mode)
        sregs.ds.selector = 2 << 3; // GDT index
        sregs.ds.present = 1; // Segment is present
        sregs.ds.type_ = 3; // Data: read/write, accessed
        sregs.ds.dpl = 0; // Kernel mode
        sregs.ds.db = 1; // 32-bit segment (ignored in 64-bit mode)
        sregs.ds.s = 1; // Code/data segment
        sregs.ds.g = 1; // Granularity = 4 KiB units

        // Replicate for the other segments.
        sregs.es = sregs.ds;
        sregs.fs = sregs.ds;
        sregs.gs = sregs.ds;
        sregs.ss = sregs.ds;

        // Enable long mode
        sregs.efer |= 0x500; // LME (Long Mode Enable) + LMA (Long Mode Active)
        sregs.cr0 |= 0x80000001; // PG (Paging) + PE (Protection Enable)
        sregs.cr4 |= 0x20; // PAE (Physical Address Extension)

        // Set special registers
        self.vcpu.set_sregs(&sregs)?;

        Ok(())
    }

    fn map_pages(&mut self) -> anyhow::Result<()> {
        unsafe {
            ptr::write_bytes(
                self.guest_mem.as_ptr().add(PML4_OFFSET),
                0,
                3 * PAGE_TABLE_SIZE,
            );

            let pml4 = self.guest_mem.as_ptr().add(PML4_OFFSET) as *mut u64;
            *pml4 = (GUEST_PHYS_ADDR + PML4_OFFSET as u64 + PAGE_TABLE_SIZE as u64) | 0x3;
            let pdpt = self.guest_mem.as_ptr().add(PML4_OFFSET + PAGE_TABLE_SIZE) as *mut u64;
            *pdpt = (GUEST_PHYS_ADDR + PML4_OFFSET as u64 + 2 * PAGE_TABLE_SIZE as u64) | 0x3;
            let pd = self
                .guest_mem
                .as_ptr()
                .add(PML4_OFFSET + 2 * PAGE_TABLE_SIZE) as *mut u64;

            (0..512).for_each(|i| *pd.add(i) = (i << 21) as u64 | 0x83);
        };

        let mut sregs = self.vcpu.get_sregs()?;
        sregs.cr3 = GUEST_PHYS_ADDR + PML4_OFFSET as u64;
        self.vcpu.set_sregs(&sregs)?;

        Ok(())
    }
}

fn main() {}

#[cfg(test)]
mod main_tests {
    use super::*;

    #[test]
    fn should_working() {
        assert!(UnderVm::new().is_ok());
    }
}
