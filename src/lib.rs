use std::ffi::CString;

use fdp_sys::{FDP_SHM, FDP_CreateSHM, FDP_Init};


#[derive(Debug)]
pub struct FDP {
    shm: *mut FDP_SHM,
}

impl FDP {
    pub fn new(vm_name: &str) -> Self {
        let c_vm_name = CString::new(vm_name).unwrap();
        // create SHM
        let shm = unsafe {
            FDP_CreateSHM(c_vm_name.into_raw())
        };
        // init FDP
        let res = unsafe { FDP_Init(shm) };
        if res == false {
            panic!("Failed to init FDP");
        }
        FDP {
            shm,
        }
    }
}
