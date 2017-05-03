# k2i

Kernel Parameters Interface using HTTP

## Quick Install

```
cargo install k2i
```

## Run

```
k2i 
```

## HTTP request to k2i

```
$ curl 127.0.0.1:8000/k2i/api/v1/proc | jq . | head -n 25
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

