use std::os::raw::c_char;

use fdp_sys::{FDP_Register, FDP_SHM};
#[cfg(unix)]
use libloading::os::unix::Symbol as RawSymbol;
#[cfg(windows)]
use libloading::os::windows::Symbol as RawSymbol;
use libloading::{library_filename, Library, Symbol};
use std::error::Error;

const LIBFDP_BASENAME: &str = "libFDP";
// libFDP function signatures type alises
// FDP_CreateSHM
type FnCreateSHM = extern "C" fn(shm_name: *mut c_char) -> *mut FDP_SHM;
// FDP_OpenSHM
type FnOpenSHM = extern "C" fn(shm_name: *const ::std::os::raw::c_char) -> *mut FDP_SHM;
// FDP_Init
type FnInit = extern "C" fn(p_shm: *mut FDP_SHM) -> bool;
// FDP_Pause
type FnPause = extern "C" fn(p_shm: *mut FDP_SHM) -> bool;
// FDP_Resume
type FnResume = extern "C" fn(p_shm: *mut FDP_SHM) -> bool;
// FDP_ReadPhysicalMemory
type FnReadPhysicalMemory = extern "C" fn(
    p_shm: *mut FDP_SHM,
    p_dst_buffer: *mut u8,
    read_size: u32,
    physical_address: u64,
) -> bool;
// FDP_ReadRegister
type FnReadRegister = extern "C" fn(
    p_shm: *mut FDP_SHM,
    cpu_id: u32,
    register_id: FDP_Register,
    p_register_value: *mut u64,
) -> bool;
// FDP_GetPhysicalMemorySize
type FnGetPhysicalMemorySize =
    extern "C" fn(p_shm: *mut FDP_SHM, p_physical_memory_size: *mut u64) -> bool;

#[derive(Debug)]
pub struct LibFDP {
    lib: Library,
    pub create_shm: RawSymbol<FnCreateSHM>,
    pub open_shm: RawSymbol<FnOpenSHM>,
    pub init: RawSymbol<FnInit>,
    pub pause: RawSymbol<FnPause>,
    pub resume: RawSymbol<FnResume>,
    pub read_physical_memory: RawSymbol<FnReadPhysicalMemory>,
    pub read_register: RawSymbol<FnReadRegister>,
    pub get_physical_memory_size: RawSymbol<FnGetPhysicalMemorySize>,
}

impl LibFDP {
    pub unsafe fn new() -> Result<Self, Box<dyn Error>> {
        let libfdp_filename = library_filename(LIBFDP_BASENAME);
        info!("Loading {}", libfdp_filename.to_str().unwrap());
        let lib = Library::new(libfdp_filename)?;
        // load symbols
        let create_shm_sym: Symbol<FnCreateSHM> = lib.get(b"FDP_CreateSHM\0")?;
        let create_shm = create_shm_sym.into_raw();

        let open_shm_sym: Symbol<FnOpenSHM> = lib.get(b"FDP_OpenSHM\0")?;
        let open_shm = open_shm_sym.into_raw();

        let init_sym: Symbol<FnInit> = lib.get(b"FDP_Init\0")?;
        let init = init_sym.into_raw();

        let pause_sym: Symbol<FnPause> = lib.get(b"FDP_Pause\0")?;
        let pause = pause_sym.into_raw();

        let resume_sym: Symbol<FnResume> = lib.get(b"FDP_Resume\0")?;
        let resume = resume_sym.into_raw();

        let read_physical_memory_sym: Symbol<FnReadPhysicalMemory> =
            lib.get(b"FDP_ReadPhysicalMemory\0")?;
        let read_physical_memory = read_physical_memory_sym.into_raw();

        let read_register_sym: Symbol<FnReadRegister> = lib.get(b"FDP_ReadRegister\0")?;
        let read_register = read_register_sym.into_raw();

        let get_physical_memory_size_sym: Symbol<FnGetPhysicalMemorySize> =
            lib.get(b"FDP_GetPhysicalMemorySize\0")?;
        let get_physical_memory_size = get_physical_memory_size_sym.into_raw();

        Ok(LibFDP {
            lib,
            create_shm,
            open_shm,
            init,
            pause,
            resume,
            read_physical_memory,
            read_register,
            get_physical_memory_size,
        })
    }
}
