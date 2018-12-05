# k2i

Kernel Parameters Interface using HTTP

## Quick Install

```
cargo install k2i
```

## k2i Server Run

```
k2i 
```

## Support API

- root response

```
$ curl 127.0.0.1:8000/v1/
root response!
```

- test response

```
$ curl 127.0.0.1:8000/v1/hello
hello!
```
- current configuration response

```
$ curl -s 127.0.0.1:8000/v1/config | jq .
{
  "hostname": "127.0.0.1",
  "port": 8000,
  "threads": 1
}
```

- all proc information response

```
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
.
.
(snip)
.
    "vsize": 38801408,
    "rss_rlim": 18446744073709552000,
    "flags": 4194560,
    "min_flt": 9842,
    "maj_flt": 142,
    "cmin_flt": 3321118,
    "cmaj_flt": 1580,
    "environ": "",
    "cmdline": [],
    "cgroup": [
      "10:perf_event:/",
      "9:cpuset:/",
      "8:blkio:/init.scope",
      "7:net_cls,net_prio:/",
      "6:cpu,cpuacct:/init.scope",
      "5:pids:/init.scope",
      "4:hugetlb:/",
      "3:memory:/init.scope",
      "2:devices:/init.scope",
      "1:name=systemd:/init.scope"
    ],
    "supgid": "",
    "supgrp": "",
    "euser": "",
    "ruser": "",
    "suser": "",
    "fuser": "",
    "rgroup": "",
    "egroup": "",
    "sgroup": "",
    "fgroup": "",
    "cmd": "systemd",
    "pgrp": 1,

```

- Specified PID proc information response

```
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
.
.
(snip)
.
```


## HTTP request to k2i

```
$ curl 127.0.0.1:8000/v1/proc | jq . | head -n 25
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
    "utime": 107,
    "stime": 1498,
    "cutime": 11285,
    "cstime": 1529,
    "start_time": 12,
    "signal": [
      0,
      0,
      0,
      0,
      0,
      0,
      0,
      0,
```

# License

This project is under the MIT License:

* http://www.opensource.org/licenses/mit-license.php

