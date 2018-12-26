# k2i ![crates.io](https://img.shields.io/crates/v/k2i.svg) [![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://raw.githubusercontent.com/matsumotory/k2i/master/LICENSE)


Kernel Parameters Interface using HTTP(S)

## Quick Install

```
cargo install k2i
```

## k2i Server Run

```
k2i 
```

## Support API

### k2i information response

```json
$ curl 127.0.0.1:8000/v1/proc/self
[{"tid":0,"ppid":5294,"maj_delta":0,"min_delta":0,"pcpu":0,"state":83,"pad_1":0,"pad_2":0,"pad_3":0,"utime":3,"stime":1,"cutime":0,"cstime":0,"start_time":9479566,"signal":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"blocked":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"sigignore":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"sigcatch":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"_sigpnd":[0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0],"start_code":94062006767616,"end_code":94062010354808,"start_stack":140724759217040,"kstk_esp":140724759213088,"kstk_eip":140628670241165,"wchan":0,"priority":20,"nice":0,"rss":484,"alarm":0,"size":0,"resident":0,"share":0,"trs":0,"lrs":0,"drs":0,"dt":0,"vm_size":0,"vm_lock":0,"vm_rss":0,"vm_data":0,"vm_stack":0,"vm_swap":0,"vm_exe":0,"vm_lib":0,"rtprio":0,"sched":0,"vsize":239472640,"rss_rlim":18446744073709551615,"flags":1077936128,"min_flt":2945,"maj_flt":2,"cmin_flt":0,"cmaj_flt":0,"environ":"","cmdline":[],"cgroup":[],"supgid":"","supgrp":"","euser":"","ruser":"","suser":"","fuser":"","rgroup":"","egroup":"","sgroup":"","fgroup":"","cmd":"k2i","pgrp":5507,"session":5294,"nlwp":4,"tgid":0,"tty":34816,"euid":0,"egid":0,"ruid":0,"rgid":0,"suid":0,"sgid":0,"fuid":0,"fgid":0,"tpgid":5507,"exit_signal":17,"processor":0,"ns":[0,0,0,0,0,0]}]
```

### root response

```
$ curl 127.0.0.1:8000/v1/
root response!
```

### test response

```
$ curl 127.0.0.1:8000/v1/hello
hello!
```
- current configuration response

```json
$ curl -s 127.0.0.1:8000/v1/config | jq .
{
  "hostname": "127.0.0.1",
  "port": 8000,
  "threads": 1
}
```

### all proc information response

```json
$ curl -s 127.0.0.1:8000/v1/proc | jq . | head -n 30
[
  {
    "tid": 1,
    "ppid": 0,
    "maj_delta": 0,
    "min_delta": 0,
    "pcpu": 0,
    "state": 83,
    "pad_1": 0,
    "pad_2": 0,
    "pad_3": 0,
    "utime": 113,
    "stime": 1498,
    "cutime": 63742,
    "cstime": 4669,
    "start_time": 12,

(snip)

    "sched": 0,
    "vsize": 9887744,
    "rss_rlim": 18446744073709552000,
    "flags": 4194304,
    "min_flt": 143,
    "maj_flt": 0,
    "cmin_flt": 0,
    "cmaj_flt": 0,
    "environ": "",
    "cmdline": [
      "less"
    ],
    "cgroup": [
      "11:cpuset:/",
      "10:net_cls,net_prio:/",
      "9:hugetlb:/",
      "8:devices:/user.slice",
      "7:perf_event:/",
      "6:memory:/user.slice",
      "5:blkio:/user.slice",
      "4:pids:/user.slice/user-1000.slice",
      "3:cpu,cpuacct:/user.slice",
      "2:freezer:/",
      "1:name=systemd:/user.slice/user-1000.slice/session-39.scope"
    ],
(snip)
```

### Specified PID proc information response

```json
$ curl -s 127.0.0.1:8000/v1/proc/19309 | jq . | head -n 30
[
  {
    "tid": 19309,
    "ppid": 19256,
    "maj_delta": 0,
    "min_delta": 0,
    "pcpu": 0,
    "state": 83,
    "pad_1": 0,
    "pad_2": 0,
    "pad_3": 0,
    "utime": 3,
    "stime": 15,
    "cutime": 0,
    "cstime": 0,
    "start_time": 2879872,
    "signal": [
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
(snip)
```

### Specified cmd name proc information response

```json
$ curl 127.0.0.1:8000/v1/proc/cmd/lxc | jq .
[
  {
    "tid": 756,
    "ppid": 1,
    "maj_delta": 0,
    "min_delta": 0,
    "pcpu": 0,
    "state": 83,
    "pad_1": 0,
    "pad_2": 0,
    "pad_3": 0,
    "utime": 12,
    "stime": 41,
    "cutime": 0,
    "cstime": 0,
    "start_time": 861,

(snip)

    "rtprio": 0,
    "sched": 0,
    "vsize": 468344832,
    "rss_rlim": 18446744073709552000,
    "flags": 1077936384,
    "min_flt": 555,
    "maj_flt": 5,
    "cmin_flt": 33,
    "cmaj_flt": 0,
    "environ": "",
    "cmdline": [
      "/usr/bin/lxcfs",
      "/var/lib/lxcfs/"
    ],
    "cgroup": [
      "11:cpuset:/",
      "10:net_cls,net_prio:/",
      "9:hugetlb:/",
      "8:devices:/system.slice/lxcfs.service",
      "7:perf_event:/",
      "6:memory:/system.slice/lxcfs.service",
      "5:blkio:/system.slice/lxcfs.service",
      "4:pids:/system.slice/lxcfs.service",
      "3:cpu,cpuacct:/system.slice/lxcfs.service",
      "2:freezer:/",
      "1:name=systemd:/system.slice/lxcfs.service"
    ],

(snip)

    "cmd": "lxcfs",
    "pgrp": 756,
    "session": 756,
    "nlwp": 7,
    "tgid": 756,
    "tty": 0,
    "euid": 0,
    "egid": 0,
    "ruid": 0,
    "rgid": 0,
    "suid": 0,
    "sgid": 0,
    "fuid": 0,
    "fgid": 0,
    "tpgid": -1,
    "exit_signal": 17,

(snip)

```

# License

This project is under the MIT License:

* http://www.opensource.org/licenses/mit-license.php

