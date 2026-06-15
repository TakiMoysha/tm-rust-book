use x86::msr::{rdmsr, wrmsr};

// MSR адреса
const IA32_PM_ENABLE: u32 = 0x770;
const IA32_HWP_CAPABILITIES: u32 = 0x771;
const IA32_HWP_REQUEST: u32 = 0x774;
const IA32_HWP_REQUEST_PKG: u32 = 0x775; // Package-level control

#[derive(Debug, Clone, Copy)]
pub struct HwpRequest {
    pub minimum_performance: u8,           // [7:0]
    pub maximum_performance: u8,           // [15:8]
    pub desired_performance: u8,           // [23:16] (0 = auto)
    pub energy_performance_preference: u8, // [31:24] EPP
    pub activity_window: u16,              // [41:32]
                                           // [63:42] - reserved
}

impl HwpRequest {
    pub fn from_msr(value: u64) -> Self {
        Self {
            minimum_performance: (value & 0xFF) as u8,
            maximum_performance: ((value >> 8) & 0xFF) as u8,
            desired_performance: ((value >> 16) & 0xFF) as u8,
            energy_performance_preference: ((value >> 24) & 0xFF) as u8,
            activity_window: ((value >> 32) & 0x3FF) as u16,
        }
    }

    pub fn to_msr(&self) -> u64 {
        (self.minimum_performance as u64)
            | ((self.maximum_performance as u64) << 8)
            | ((self.desired_performance as u64) << 16)
            | ((self.energy_performance_preference as u64) << 24)
            | ((self.activity_window as u64) << 32)
    }
}

pub struct SpeedShift {
    initial_request: u64, // restore after Drop
}

impl SpeedShift {
    pub fn new() -> Result<Self, &'static str> {
        let pm_enable = unsafe { rdmsr(IA32_PM_ENABLE) };
        if (pm_enable & 1) == 0 {
            return Err("Speed Shift (HWP) not enabled");
        }

        let initial = unsafe { rdmsr(IA32_HWP_REQUEST) };

        Ok(Self {
            initial_request: initial,
        })
    }

    /// Current HWP settings
    pub fn read_request(&self) -> HwpRequest {
        let value = unsafe { rdmsr(IA32_HWP_REQUEST) };
        HwpRequest::from_msr(value)
    }

    /// Get only EPP (0-255)
    pub fn get_epp(&self) -> u8 {
        self.read_request().energy_performance_preference
    }

    /// EPP (0 (performance) .. 255 (energy))
    pub fn set_epp(&self, epp: u8) {
        let current = unsafe { rdmsr(IA32_HWP_REQUEST) };

        // Очищаем старый EPP и устанавливаем новый
        let new_value = (current & !(0xFFu64 << 24)) | ((epp as u64) << 24);

        unsafe { wrmsr(IA32_HWP_REQUEST, new_value) };
    }

    pub fn read_capabilities(&self) -> HwpCapabilities {
        let value = unsafe { rdmsr(IA32_HWP_CAPABILITIES) };
        HwpCapabilities::from_msr(value)
    }
}

impl Drop for SpeedShift {
    fn drop(&mut self) {
        // restore initial value
        unsafe { wrmsr(IA32_HWP_REQUEST, self.initial_request) };
    }
}

#[derive(Debug)]
pub struct HwpCapabilities {
    pub highest_performance: u8,        // [7:0]
    pub guaranteed_performance: u8,     // [15:8]
    pub most_efficient_performance: u8, // [23:16]
    pub lowest_performance: u8,         // [31:24]
}

impl HwpCapabilities {
    pub fn from_msr(value: u64) -> Self {
        Self {
            highest_performance: (value & 0xFF) as u8,
            guaranteed_performance: ((value >> 8) & 0xFF) as u8,
            most_efficient_performance: ((value >> 16) & 0xFF) as u8,
            lowest_performance: ((value >> 24) & 0xFF) as u8,
        }
    }
}
