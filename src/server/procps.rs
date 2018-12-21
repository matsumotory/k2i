use procps_sys::readproc::*;
use rustc_serialize::json;
use std::ffi::CStr;
use std::os::raw::*;
use std::ptr::null_mut;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProcpsElem {
    tid: c_int,
    ppid: c_int,
    maj_delta: c_ulong,
    min_delta: c_ulong,
    pcpu: c_uint,
    state: c_char,
    pad_1: c_char,
    pad_2: c_char,
    pad_3: c_char,
    utime: c_ulonglong,
    stime: c_ulonglong,
    cutime: c_ulonglong,
    cstime: c_ulonglong,
    start_time: c_ulonglong,
    signal: [c_char; 18usize],
    blocked: [c_char; 18usize],
    sigignore: [c_char; 18usize],
    sigcatch: [c_char; 18usize],
    _sigpnd: [c_char; 18usize],
    start_code: c_ulong,
    end_code: c_ulong,
    start_stack: c_ulong,
    kstk_esp: c_ulong,
    kstk_eip: c_ulong,
    wchan: c_ulong,
    priority: c_long,
    nice: c_long,
    rss: c_long,
    alarm: c_long,
    size: c_long,
    resident: c_long,
    share: c_long,
    trs: c_long,
    lrs: c_long,
    drs: c_long,
    dt: c_long,
    vm_size: c_ulong,
    vm_lock: c_ulong,
    vm_rss: c_ulong,
    vm_data: c_ulong,
    vm_stack: c_ulong,
    vm_swap: c_ulong,
    vm_exe: c_ulong,
    vm_lib: c_ulong,
    rtprio: c_ulong,
    sched: c_ulong,
    vsize: c_ulong,
    rss_rlim: c_ulong,
    flags: c_ulong,
    min_flt: c_ulong,
    maj_flt: c_ulong,
    cmin_flt: c_ulong,
    cmaj_flt: c_ulong,
    environ: String,
    cmdline: Vec<String>,
    cgroup: Vec<String>,
    supgid: String,
    supgrp: String,
    euser: String,
    ruser: String,
    suser: String,
    fuser: String,
    rgroup: String,
    egroup: String,
    sgroup: String,
    fgroup: String,
    cmd: String,
    //ring: *mut proc_t,
    //next: *mut proc_t,
    pgrp: c_int,
    session: c_int,
    nlwp: c_int,
    tgid: c_int,
    tty: c_int,
    euid: c_int,
    egid: c_int,
    ruid: c_int,
    rgid: c_int,
    suid: c_int,
    sgid: c_int,
    fuid: c_int,
    fgid: c_int,
    tpgid: c_int,
    exit_signal: c_int,
    processor: c_int,
    ns: [c_long; 6usize],
}

fn os_raw_cchar_to_vec(cchar: *mut *mut c_char) -> Vec<String> {
    unsafe {
        let mut optional = Some(0);
        let mut vec = Vec::new();

        if !cchar.is_null() {
            while let Some(i) = optional {
                if !(*cchar.offset(i)).is_null() {
                    let mut s = CStr::from_ptr(*cchar.offset(i)).to_string_lossy().into_owned();
                    if !s.is_empty() {
                        vec.push(s);
                    }
                    optional = Some(i + 1);
                } else {
                    optional = None;
                }
            }
        }
        vec
    }
}

fn os_raw_cchar_to_string(cchar: *mut *mut c_char) -> String {
    unsafe {
        let mut optional = Some(1);
        let mut char_string = "".to_string();

        while let Some(i) = optional {
            if !cchar.is_null() {
                if !(*cchar.offset(i)).is_null() {
                    char_string = format!(
                        "{}{}",
                        char_string,
                        CStr::from_ptr(*cchar.offset(i))
                            .to_string_lossy()
                            .into_owned()
                    );
                    optional = Some(i + 1);
                } else {
                    optional = None;
                }
            } else {
                optional = None;
            }
        }
        char_string
    }
}

fn os_raw_cchar_to_string2(cchar: *mut c_char) -> String {
    unsafe {
        if cchar == null_mut() {
            "".to_string()
        } else {
            CStr::from_ptr(cchar).to_string_lossy().into_owned()
        }
    }
}

fn os_raw_cchar_to_string3(cchar: *const c_char) -> String {
    unsafe {
        if cchar == null_mut() {
            "".to_string()
        } else {
            CStr::from_ptr(cchar).to_string_lossy().into_owned()
        }
    }
}

pub fn procps_json_encode(flags: c_int, pid: &c_int) -> String {
    let mut procps_list = Vec::new();

    unsafe {
        let proctab = openproc(flags, pid);
        let mut optional = Some(readproc(proctab, null_mut()));

        while let Some(procinfo) = optional {
            if (procinfo).is_null() {
                optional = None;
            } else {
                let procps_elem = ProcpsElem {
                    tid: (*procinfo).tid,
                    ppid: (*procinfo).ppid,
                    maj_delta: (*procinfo).maj_delta,
                    min_delta: (*procinfo).min_delta,
                    pcpu: (*procinfo).pcpu,
                    state: (*procinfo).state,
                    pad_1: (*procinfo).pad_1,
                    pad_2: (*procinfo).pad_2,
                    pad_3: (*procinfo).pad_3,
                    utime: (*procinfo).utime,
                    stime: (*procinfo).stime,
                    cutime: (*procinfo).cutime,
                    cstime: (*procinfo).cstime,
                    start_time: (*procinfo).start_time,
                    signal: (*procinfo).signal,
                    blocked: (*procinfo).blocked,
                    sigignore: (*procinfo).sigignore,
                    sigcatch: (*procinfo).sigcatch,
                    _sigpnd: (*procinfo)._sigpnd,
                    start_code: (*procinfo).start_code,
                    end_code: (*procinfo).end_code,
                    start_stack: (*procinfo).start_stack,
                    kstk_esp: (*procinfo).kstk_esp,
                    kstk_eip: (*procinfo).kstk_eip,
                    wchan: (*procinfo).wchan,
                    priority: (*procinfo).priority,
                    nice: (*procinfo).nice,
                    rss: (*procinfo).rss,
                    alarm: (*procinfo).alarm,
                    size: (*procinfo).size,
                    resident: (*procinfo).resident,
                    share: (*procinfo).share,
                    trs: (*procinfo).trs,
                    lrs: (*procinfo).lrs,
                    drs: (*procinfo).drs,
                    dt: (*procinfo).dt,
                    vm_size: (*procinfo).vm_size,
                    vm_lock: (*procinfo).vm_lock,
                    vm_rss: (*procinfo).vm_rss,
                    vm_data: (*procinfo).vm_data,
                    vm_stack: (*procinfo).vm_stack,
                    vm_swap: (*procinfo).vm_swap,
                    vm_exe: (*procinfo).vm_exe,
                    vm_lib: (*procinfo).vm_lib,
                    rtprio: (*procinfo).rtprio,
                    sched: (*procinfo).sched,
                    vsize: (*procinfo).vsize,
                    rss_rlim: (*procinfo).rss_rlim,
                    flags: (*procinfo).flags,
                    min_flt: (*procinfo).min_flt,
                    maj_flt: (*procinfo).maj_flt,
                    cmin_flt: (*procinfo).cmin_flt,
                    cmaj_flt: (*procinfo).cmaj_flt,
                    environ: os_raw_cchar_to_string((*procinfo).environ),
                    cmdline: os_raw_cchar_to_vec((*procinfo).cmdline),
                    cgroup: os_raw_cchar_to_vec((*procinfo).cgroup),
                    supgid: os_raw_cchar_to_string2((*procinfo).supgid),
                    supgrp: os_raw_cchar_to_string2((*procinfo).supgrp),
                    euser: os_raw_cchar_to_string3((*procinfo).euser.as_ptr()),
                    ruser: os_raw_cchar_to_string3((*procinfo).ruser.as_ptr()),
                    suser: os_raw_cchar_to_string3((*procinfo).suser.as_ptr()),
                    fuser: os_raw_cchar_to_string3((*procinfo).fuser.as_ptr()),
                    rgroup: os_raw_cchar_to_string3((*procinfo).rgroup.as_ptr()),
                    egroup: os_raw_cchar_to_string3((*procinfo).egroup.as_ptr()),
                    sgroup: os_raw_cchar_to_string3((*procinfo).sgroup.as_ptr()),
                    fgroup: os_raw_cchar_to_string3((*procinfo).fgroup.as_ptr()),
                    cmd: os_raw_cchar_to_string3((*procinfo).cmd.as_ptr()),
                    //ring: (*procinfo).ring,
                    //next: (*procinfo).next,
                    pgrp: (*procinfo).pgrp,
                    session: (*procinfo).session,
                    nlwp: (*procinfo).nlwp,
                    tgid: (*procinfo).tgid,
                    tty: (*procinfo).tty,
                    euid: (*procinfo).euid,
                    egid: (*procinfo).egid,
                    ruid: (*procinfo).ruid,
                    rgid: (*procinfo).rgid,
                    suid: (*procinfo).suid,
                    sgid: (*procinfo).sgid,
                    fuid: (*procinfo).fuid,
                    fgid: (*procinfo).fgid,
                    tpgid: (*procinfo).tpgid,
                    exit_signal: (*procinfo).exit_signal,
                    processor: (*procinfo).processor,
                    ns: (*procinfo).ns,
                };
                procps_list.push(procps_elem);
                optional = Some(readproc(proctab, null_mut()));
            }
            //freeproc(procinfo);
        }
        closeproc(proctab);
    }

    json::encode(&procps_list).unwrap()
}
