#[macro_use]
extern crate log;

mod libfdp;

use std::convert::TryInto;
use std::error::Error;
use std::ffi::CString;

use custom_error::custom_error;
use fdp_sys::FDP_SHM;
use libfdp::LibFDP;

// Define simple FDP error
custom_error! {pub FDPError{} = "FDP error."}

#[derive(Debug)]
pub struct FDP {
    shm: *mut FDP_SHM,
    libfdp: LibFDP,
}

impl FDP {
    pub fn new(vm_name: &str) -> Self {
        let c_vm_name = CString::new(vm_name).unwrap();
        let libfdp = unsafe { LibFDP::new() };
        // create SHM
        info!("create SHM {}", vm_name);
        let shm = (libfdp.open_shm)(c_vm_name.into_raw());

        // init FDP
        info!("initialize FDP");
        let res = (libfdp.init)(shm);
        if res == false {
            panic!("Failed to init FDP");
        }
        FDP { shm, libfdp }
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
