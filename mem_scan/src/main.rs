use winapi::{
    shared::{
        minwindef::{DWORD, LPVOID},
        ntdef::PVOID,
    },
    um::{
        memoryapi::VirtualQueryEx,
        processthreadsapi::{GetCurrentProcess, GetCurrentProcessId},
        sysinfoapi::{GetSystemInfo, SYSTEM_INFO},
        winnt::{HANDLE, MEMORY_BASIC_INFORMATION},
    },
};

fn main() {
    let this_pid: DWORD;
    let this_proc: HANDLE;
    let min_addr: LPVOID;
    let max_addr: LPVOID;
    let mut base_addr: PVOID;
    let mut proc_info: SYSTEM_INFO;
    let mut mem_info: MEMORY_BASIC_INFORMATION;
    const MEMINFO_SIZE: usize = std::mem::size_of::<MEMORY_BASIC_INFORMATION>();

    unsafe {
        base_addr = std::mem::zeroed();
        proc_info = std::mem::zeroed();
        mem_info = std::mem::zeroed();
    }

    unsafe {
        this_pid = GetCurrentProcessId();
        this_proc = GetCurrentProcess();
        GetSystemInfo(&mut proc_info);
    }

    min_addr = proc_info.lpMinimumApplicationAddress;
    max_addr = proc_info.lpMaximumApplicationAddress;

    println!("pid: {:?} @ proc: {:p}", this_pid, this_proc);
    println!("Page size: {:?}", proc_info.dwPageSize);
    println!("Number of processors: {:?}", proc_info.dwNumberOfProcessors);
    println!("min: {:p}, max: {:p}", min_addr, max_addr);

    loop {
        let rc = unsafe { VirtualQueryEx(this_proc, base_addr, &mut mem_info, MEMINFO_SIZE) };

        if rc == 0 {
            break;
        }
        println!("==========");
        println!("Type: {:?}", mem_info.Type);
        println!("AllocationBase: {:?}", mem_info.AllocationBase);
        println!("AllocationProtect: {:?}", mem_info.AllocationProtect);
        println!("BaseAddress: {:?}", mem_info.BaseAddress);
        println!("Protect: {:?}", mem_info.Protect);
        println!("RegionSize: {:?}", mem_info.RegionSize);
        println!("State: {:?}", mem_info.State);
        base_addr = ((base_addr as usize) + mem_info.RegionSize) as PVOID;
    }
}
