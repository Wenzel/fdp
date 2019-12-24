#[macro_use]
extern crate log;

use std::error::Error;
use std::ffi::CString;
use std::convert::TryInto;

use custom_error::custom_error;
use fdp_sys::{
    FDP_CreateSHM, FDP_Init, FDP_Pause, FDP_Resume, FDP_SHM,
    FDP_ReadPhysicalMemory
};


// Define simple FDP error
custom_error!{pub FDPError{} = "FDP error."}

#[derive(Debug)]
pub struct FDP {
    shm: *mut FDP_SHM,
}

impl FDP {
    pub fn new(vm_name: &str) -> Self {
        let c_vm_name = CString::new(vm_name).unwrap();
        // create SHM
        info!("create SHM {}", vm_name);
        let shm = unsafe {
            FDP_CreateSHM(c_vm_name.into_raw())
        };
        // init FDP
        info!("initialize FDP");
        let res = unsafe { FDP_Init(shm) };
        if res == false {
            panic!("Failed to init FDP");
        }
        FDP {
            shm,
        }
    }

    pub fn read_physical_memory(&self, paddr: u64, buffer: &mut [u8]) -> Result<(), Box<dyn Error>> {
        let buf_size: u32 = buffer.len().try_into().unwrap();
        match unsafe {
            FDP_ReadPhysicalMemory(self.shm, buffer.as_mut_ptr(), buf_size, paddr)
        } {
            false => Err(Box::new(FDPError{})),
            true => Ok(()),
        }
    }

    pub fn pause(&self) -> Result<(), Box<dyn Error>> {
        match unsafe { FDP_Pause(self.shm) } {
            false => Err(Box::new(FDPError{})),
            true => Ok(()),
        }
    }

    pub fn resume(&self) -> Result<(), Box<dyn Error>> {
        match unsafe { FDP_Resume(self.shm) } {
            false => Err(Box::new(FDPError{})),
            true => Ok(()),
        }
    }
}
