use std::os::raw::c_char;

use fdp_sys::FDP_SHM;
use libloading::os::unix::Symbol as RawSymbol;
use libloading::{Library, Symbol};

const LIBFDP_FILENAME: &'static str = "libFDP.so";
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
    pub get_physical_memory_size: RawSymbol<FnGetPhysicalMemorySize>,
}

impl LibFDP {
    pub unsafe fn new() -> Self {
        info!("Loading {}", LIBFDP_FILENAME);
        let lib = Library::new(LIBFDP_FILENAME).unwrap();
        // load symbols
        let create_shm_sym: Symbol<FnCreateSHM> = lib.get(b"FDP_CreateSHM\0").unwrap();
        let create_shm = create_shm_sym.into_raw();

        let open_shm_sym: Symbol<FnOpenSHM> = lib.get(b"FDP_OpenSHM\0").unwrap();
        let open_shm = open_shm_sym.into_raw();

        let init_sym: Symbol<FnInit> = lib.get(b"FDP_Init\0").unwrap();
        let init = init_sym.into_raw();

        let pause_sym: Symbol<FnPause> = lib.get(b"FDP_Pause\0").unwrap();
        let pause = pause_sym.into_raw();

        let resume_sym: Symbol<FnResume> = lib.get(b"FDP_Resume\0").unwrap();
        let resume = resume_sym.into_raw();

        let read_physical_memory_sym: Symbol<FnReadPhysicalMemory> =
            lib.get(b"FDP_ReadPhysicalMemory\0").unwrap();
        let read_physical_memory = read_physical_memory_sym.into_raw();

        let get_physical_memory_size_sym: Symbol<FnGetPhysicalMemorySize> =
            lib.get(b"FDP_GetPhysicalMemorySize\0").unwrap();
        let get_physical_memory_size = get_physical_memory_size_sym.into_raw();

        LibFDP {
            lib,
            create_shm,
            open_shm,
            init,
            pause,
            resume,
            read_physical_memory,
            get_physical_memory_size,
        }
    }
}
