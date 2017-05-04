use procps_sys::readproc::{openproc, readproc, closeproc, proc_t};
use rustc_serialize::json;
use std::ffi::CStr;
use std::ptr::null_mut;

#[derive(Debug, RustcDecodable, RustcEncodable)]
struct ProcpsElem {
    tid: ::std::os::raw::c_int,
    ppid: ::std::os::raw::c_int,
    maj_delta: ::std::os::raw::c_ulong,
    min_delta: ::std::os::raw::c_ulong,
    pcpu: ::std::os::raw::c_uint,
    state: ::std::os::raw::c_char,
    pad_1: ::std::os::raw::c_char,
    pad_2: ::std::os::raw::c_char,
    pad_3: ::std::os::raw::c_char,
    utime: ::std::os::raw::c_ulonglong,
    stime: ::std::os::raw::c_ulonglong,
    cutime: ::std::os::raw::c_ulonglong,
    cstime: ::std::os::raw::c_ulonglong,
    start_time: ::std::os::raw::c_ulonglong,
    signal: [::std::os::raw::c_char; 18usize],
    blocked: [::std::os::raw::c_char; 18usize],
    sigignore: [::std::os::raw::c_char; 18usize],
    sigcatch: [::std::os::raw::c_char; 18usize],
    _sigpnd: [::std::os::raw::c_char; 18usize],
    start_code: ::std::os::raw::c_ulong,
    end_code: ::std::os::raw::c_ulong,
    start_stack: ::std::os::raw::c_ulong,
    kstk_esp: ::std::os::raw::c_ulong,
    kstk_eip: ::std::os::raw::c_ulong,
    wchan: ::std::os::raw::c_ulong,
    priority: ::std::os::raw::c_long,
    nice: ::std::os::raw::c_long,
    rss: ::std::os::raw::c_long,
    alarm: ::std::os::raw::c_long,
    size: ::std::os::raw::c_long,
    resident: ::std::os::raw::c_long,
    share: ::std::os::raw::c_long,
    trs: ::std::os::raw::c_long,
    lrs: ::std::os::raw::c_long,
    drs: ::std::os::raw::c_long,
    dt: ::std::os::raw::c_long,
    vm_size: ::std::os::raw::c_ulong,
    vm_lock: ::std::os::raw::c_ulong,
    vm_rss: ::std::os::raw::c_ulong,
    vm_rss_anon: ::std::os::raw::c_ulong,
    vm_rss_file: ::std::os::raw::c_ulong,
    vm_rss_shared: ::std::os::raw::c_ulong,
    vm_data: ::std::os::raw::c_ulong,
    vm_stack: ::std::os::raw::c_ulong,
    vm_swap: ::std::os::raw::c_ulong,
    vm_exe: ::std::os::raw::c_ulong,
    vm_lib: ::std::os::raw::c_ulong,
    rtprio: ::std::os::raw::c_ulong,
    sched: ::std::os::raw::c_ulong,
    vsize: ::std::os::raw::c_ulong,
    rss_rlim: ::std::os::raw::c_ulong,
    flags: ::std::os::raw::c_ulong,
    min_flt: ::std::os::raw::c_ulong,
    maj_flt: ::std::os::raw::c_ulong,
    cmin_flt: ::std::os::raw::c_ulong,
    cmaj_flt: ::std::os::raw::c_ulong,
    environ: String,
    cmdline: String,
    cgroup: String,
    cgname: String,
    supgid: String,
    supgrp: String,
    pgrp: ::std::os::raw::c_int,
    session: ::std::os::raw::c_int,
    nlwp: ::std::os::raw::c_int,
    tgid: ::std::os::raw::c_int,
    tty: ::std::os::raw::c_int,
    euid: ::std::os::raw::c_int,
    egid: ::std::os::raw::c_int,
    ruid: ::std::os::raw::c_int,
    rgid: ::std::os::raw::c_int,
    suid: ::std::os::raw::c_int,
    sgid: ::std::os::raw::c_int,
    fuid: ::std::os::raw::c_int,
    fgid: ::std::os::raw::c_int,
    tpgid: ::std::os::raw::c_int,
    exit_signal: ::std::os::raw::c_int,
    processor: ::std::os::raw::c_int,
    oom_score: ::std::os::raw::c_int,
    oom_adj: ::std::os::raw::c_int,
    sd_mach: String,
    sd_ouid: String,
    sd_seat: String,
    sd_sess: String,
    sd_slice: String,
    sd_unit: String,
    sd_uunit: String,
    //lxcname: String,
    //ns: [::std::os::raw::c_long; 6usize],
    //euser: [::std::os::raw::c_char; 33usize],
    //ruser: [::std::os::raw::c_char; 33usize],
    //suser: [::std::os::raw::c_char; 33usize],
    //fuser: [::std::os::raw::c_char; 33usize],
    //rgroup: [::std::os::raw::c_char; 33usize],
    //egroup: [::std::os::raw::c_char; 33usize],
    //sgroup: [::std::os::raw::c_char; 33usize],
    //fgroup: [::std::os::raw::c_char; 33usize],
    //cmd: [::std::os::raw::c_char; 16usize],
    //ring: *mut proc_t,
    //next: *mut proc_t,
}

fn os_raw_cchar_to_string(cchar: *mut *mut ::std::os::raw::c_char) -> String {
    unsafe {
        if cchar == null_mut() {
            "".to_string()
        } else {
            CStr::from_ptr(*cchar).to_string_lossy().into_owned()
        }
    }
}

fn os_raw_cchar_to_string2(cchar: *mut ::std::os::raw::c_char) -> String {
    unsafe {
        if cchar == null_mut() {
            "".to_string()
        } else {
            CStr::from_ptr(cchar).to_string_lossy().into_owned()
        }
    }
}

pub fn procps_json_encode(flags: ::std::os::raw::c_int, pid: ::std::os::raw::c_int) -> String {
    let mut procps_list = Vec::new();

    unsafe {
        let proctab = openproc(flags, pid);
        let mut procinfo = proc_t::default();

        while readproc(proctab, &mut procinfo) != null_mut() {
            let procps_elem = ProcpsElem {
                tid: procinfo.tid,
                ppid: procinfo.ppid,
                maj_delta: procinfo.maj_delta,
                min_delta: procinfo.min_delta,
                pcpu: procinfo.pcpu,
                state: procinfo.state,
                pad_1: procinfo.pad_1,
                pad_2: procinfo.pad_2,
                pad_3: procinfo.pad_3,
                utime: procinfo.utime,
                stime: procinfo.stime,
                cutime: procinfo.cutime,
                cstime: procinfo.cstime,
                start_time: procinfo.start_time,
                signal: procinfo.signal,
                blocked: procinfo.blocked,
                sigignore: procinfo.sigignore,
                sigcatch: procinfo.sigcatch,
                _sigpnd: procinfo._sigpnd,
                start_code: procinfo.start_code,
                end_code: procinfo.end_code,
                start_stack: procinfo.start_stack,
                kstk_esp: procinfo.kstk_esp,
                kstk_eip: procinfo.kstk_eip,
                wchan: procinfo.wchan,
                priority: procinfo.priority,
                nice: procinfo.nice,
                rss: procinfo.rss,
                alarm: procinfo.alarm,
                size: procinfo.size,
                resident: procinfo.resident,
                share: procinfo.share,
                trs: procinfo.trs,
                lrs: procinfo.lrs,
                drs: procinfo.drs,
                dt: procinfo.dt,
                vm_size: procinfo.vm_size,
                vm_lock: procinfo.vm_lock,
                vm_rss: procinfo.vm_rss,
                vm_rss_anon: procinfo.vm_rss_anon,
                vm_rss_file: procinfo.vm_rss_file,
                vm_rss_shared: procinfo.vm_rss_shared,
                vm_data: procinfo.vm_data,
                vm_stack: procinfo.vm_stack,
                vm_swap: procinfo.vm_swap,
                vm_exe: procinfo.vm_exe,
                vm_lib: procinfo.vm_lib,
                rtprio: procinfo.rtprio,
                sched: procinfo.sched,
                vsize: procinfo.vsize,
                rss_rlim: procinfo.rss_rlim,
                flags: procinfo.flags,
                min_flt: procinfo.min_flt,
                maj_flt: procinfo.maj_flt,
                cmin_flt: procinfo.cmin_flt,
                cmaj_flt: procinfo.cmaj_flt,
                environ: os_raw_cchar_to_string(procinfo.environ),
                cmdline: os_raw_cchar_to_string(procinfo.cmdline),
                cgroup: os_raw_cchar_to_string(procinfo.cgroup),
                cgname: os_raw_cchar_to_string2(procinfo.cgname),
                supgid: os_raw_cchar_to_string2(procinfo.supgid),
                supgrp: os_raw_cchar_to_string2(procinfo.supgrp),
                pgrp: procinfo.pgrp,
                session: procinfo.session,
                nlwp: procinfo.nlwp,
                tgid: procinfo.tgid,
                tty: procinfo.tty,
                euid: procinfo.euid,
                egid: procinfo.egid,
                ruid: procinfo.ruid,
                rgid: procinfo.rgid,
                suid: procinfo.suid,
                sgid: procinfo.sgid,
                fuid: procinfo.fuid,
                fgid: procinfo.fgid,
                tpgid: procinfo.tpgid,
                exit_signal: procinfo.exit_signal,
                processor: procinfo.processor,
                oom_score: procinfo.oom_score,
                oom_adj: procinfo.oom_adj,
                sd_mach: os_raw_cchar_to_string2(procinfo.sd_mach),
                sd_ouid: os_raw_cchar_to_string2(procinfo.sd_ouid),
                sd_seat: os_raw_cchar_to_string2(procinfo.sd_seat),
                sd_sess: os_raw_cchar_to_string2(procinfo.sd_sess),
                sd_slice: os_raw_cchar_to_string2(procinfo.sd_slice),
                sd_unit: os_raw_cchar_to_string2(procinfo.sd_unit),
                sd_uunit: os_raw_cchar_to_string2(procinfo.sd_uunit),
            };
            procps_list.push(procps_elem);
        }
        closeproc(proctab);
    }

    json::encode(&procps_list).unwrap()
}
