#[macro_use]
extern crate log;
#[macro_use]
extern crate enum_primitive_derive;
extern crate num_traits;

mod libfdp;

use std::convert::TryInto;
use std::error::Error;
use std::ffi::CString;

use num_traits::ToPrimitive;

use custom_error::custom_error;
use fdp_sys::{
    FDP_Register__FDP_CR0_REGISTER, FDP_Register__FDP_CR2_REGISTER, FDP_Register__FDP_CR3_REGISTER,
    FDP_Register__FDP_CR4_REGISTER, FDP_Register__FDP_CR8_REGISTER, FDP_Register__FDP_CS_REGISTER,
    FDP_Register__FDP_DS_REGISTER, FDP_Register__FDP_ES_REGISTER, FDP_Register__FDP_FS_REGISTER,
    FDP_Register__FDP_GDTRB_REGISTER, FDP_Register__FDP_GDTRL_REGISTER,
    FDP_Register__FDP_GS_REGISTER, FDP_Register__FDP_IDTRB_REGISTER,
    FDP_Register__FDP_IDTRL_REGISTER, FDP_Register__FDP_LDTRB_REGISTER,
    FDP_Register__FDP_LDTRL_REGISTER, FDP_Register__FDP_LDTR_REGISTER,
    FDP_Register__FDP_R10_REGISTER, FDP_Register__FDP_R11_REGISTER, FDP_Register__FDP_R12_REGISTER,
    FDP_Register__FDP_R13_REGISTER, FDP_Register__FDP_R14_REGISTER, FDP_Register__FDP_R15_REGISTER,
    FDP_Register__FDP_R8_REGISTER, FDP_Register__FDP_R9_REGISTER, FDP_Register__FDP_RAX_REGISTER,
    FDP_Register__FDP_RBP_REGISTER, FDP_Register__FDP_RBX_REGISTER, FDP_Register__FDP_RCX_REGISTER,
    FDP_Register__FDP_RDI_REGISTER, FDP_Register__FDP_RDX_REGISTER,
    FDP_Register__FDP_RFLAGS_REGISTER, FDP_Register__FDP_RIP_REGISTER,
    FDP_Register__FDP_RSI_REGISTER, FDP_Register__FDP_RSP_REGISTER, FDP_Register__FDP_SS_REGISTER,
    FDP_SHM,
};
use libfdp::LibFDP;

// Define simple FDP error
custom_error! {pub FDPError{} = "FDP error."}

// redefine FDP registers with Rust enum
#[derive(Primitive)]
#[allow(non_camel_case_types)]
pub enum RegisterType {
    RAX = FDP_Register__FDP_RAX_REGISTER as isize,
    RBX = FDP_Register__FDP_RBX_REGISTER as isize,
    RCX = FDP_Register__FDP_RCX_REGISTER as isize,
    RDX = FDP_Register__FDP_RDX_REGISTER as isize,
    RSI = FDP_Register__FDP_RSI_REGISTER as isize,
    RDI = FDP_Register__FDP_RDI_REGISTER as isize,
    RSP = FDP_Register__FDP_RSP_REGISTER as isize,
    RBP = FDP_Register__FDP_RBP_REGISTER as isize,
    RIP = FDP_Register__FDP_RIP_REGISTER as isize,
    RFLAGS = FDP_Register__FDP_RFLAGS_REGISTER as isize,
    CR0 = FDP_Register__FDP_CR0_REGISTER as isize,
    CR2 = FDP_Register__FDP_CR2_REGISTER as isize,
    CR3 = FDP_Register__FDP_CR3_REGISTER as isize,
    CR4 = FDP_Register__FDP_CR4_REGISTER as isize,
    CR8 = FDP_Register__FDP_CR8_REGISTER as isize,
    R8 = FDP_Register__FDP_R8_REGISTER as isize,
    R9 = FDP_Register__FDP_R9_REGISTER as isize,
    R10 = FDP_Register__FDP_R10_REGISTER as isize,
    R11 = FDP_Register__FDP_R11_REGISTER as isize,
    R12 = FDP_Register__FDP_R12_REGISTER as isize,
    R13 = FDP_Register__FDP_R13_REGISTER as isize,
    R14 = FDP_Register__FDP_R14_REGISTER as isize,
    R15 = FDP_Register__FDP_R15_REGISTER as isize,
    CS = FDP_Register__FDP_CS_REGISTER as isize,
    DS = FDP_Register__FDP_DS_REGISTER as isize,
    ES = FDP_Register__FDP_ES_REGISTER as isize,
    FS = FDP_Register__FDP_FS_REGISTER as isize,
    GS = FDP_Register__FDP_GS_REGISTER as isize,
    SS = FDP_Register__FDP_SS_REGISTER as isize,
    GDTR_BASE = FDP_Register__FDP_GDTRB_REGISTER as isize,
    GDTR_LIMIT = FDP_Register__FDP_GDTRL_REGISTER as isize,
    IDTR_BASE = FDP_Register__FDP_IDTRB_REGISTER as isize,
    IDTR_LIMIT = FDP_Register__FDP_IDTRL_REGISTER as isize,
    LDTR = FDP_Register__FDP_LDTR_REGISTER as isize,
    LDTR_BASE = FDP_Register__FDP_LDTRB_REGISTER as isize,
    LDTR_LIMIT = FDP_Register__FDP_LDTRL_REGISTER as isize,
}

#[derive(Debug)]
pub struct FDP {
    shm: *mut FDP_SHM,
    libfdp: LibFDP,
}

impl FDP {
    pub fn new(vm_name: &str) -> Result<Self, Box<dyn Error>> {
        let c_vm_name = CString::new(vm_name)?;
        let libfdp = unsafe { LibFDP::new()? };
        // create SHM
        info!("create SHM {}", vm_name);
        let shm = (libfdp.open_shm)(c_vm_name.into_raw());
        if shm.is_null() {
            return Err(Box::new(FDPError {}));
        }

        // init FDP
        info!("initialize FDP");
        let res = (libfdp.init)(shm);
        if !res {
            return Err(Box::new(FDPError {}));
        }
        Ok(FDP { shm, libfdp })
    }

    pub fn read_physical_memory(
        &self,
        paddr: u64,
        buffer: &mut [u8],
    ) -> Result<(), Box<dyn Error>> {
        let buf_size: u32 = buffer.len().try_into().unwrap();
        let success =
            (self.libfdp.read_physical_memory)(self.shm, buffer.as_mut_ptr(), buf_size, paddr);
        match success {
            false => Err(Box::new(FDPError {})),
            true => Ok(()),
        }
    }

    pub fn read_register(
        &self,
        vcpu_id: u32,
        register: RegisterType,
    ) -> Result<u64, Box<dyn Error>> {
        let mut value: u64 = 0;
        let success =
            (self.libfdp.read_register)(self.shm, vcpu_id, register.to_u16().unwrap(), &mut value);
        match success {
            false => Err(Box::new(FDPError {})),
            true => Ok(value),
        }
    }

    pub fn pause(&self) -> Result<(), Box<dyn Error>> {
        let success = (self.libfdp.pause)(self.shm);
        match success {
            false => Err(Box::new(FDPError {})),
            true => Ok(()),
        }
    }

    pub fn resume(&self) -> Result<(), Box<dyn Error>> {
        let success = (self.libfdp.resume)(self.shm);
        match success {
            false => Err(Box::new(FDPError {})),
            true => Ok(()),
        }
    }

    pub fn get_physical_memory_size(&self) -> Result<u64, Box<dyn Error>> {
        let mut max_addr: u64 = 0;
        let success = (self.libfdp.get_physical_memory_size)(self.shm, &mut max_addr);
        match success {
            false => Err(Box::new(FDPError {})),
            true => Ok(max_addr),
        }
    }
}
