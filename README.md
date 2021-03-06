# microcall

Experiment fast workload invocation with various ways.

## Benchmark

The collected statistics is under wrk as http benchmarking.

```bash
# Under 100 connections, 12 threads
./wrk -c100 -t12 -d10 --latency <Http Endpoint>
```

**Node.js express**

```bash
Latency Average
  8.00ms
Latency Distribution
  50%    7.28ms
  75%    7.52ms
  90%    8.89ms
  99%   21.64ms
Requests/sec
  12193.67
```

**Elixir plug**

```bash
Latency Average
  9.19ms
Latency Distribution
  50%    1.82ms
  75%    4.38ms
  90%   28.01ms
  99%  102.90ms
Requests/sec
  34954.33
```

**Go standard http**

```bash
Latency Average
  1.43ms
Latency Distribution
  50%    1.18ms
  75%    1.48ms
  90%    2.00ms
  99%    8.52ms
Requests/sec
  75803.01
```

**Rust hyper**

```bash
Latency Average
  1.82ms
Latency Distribution
  50%    1.75ms
  75%    1.87ms
  90%    2.41ms
  99%    2.59ms
Requests/sec
  52456.32
```

**Invoker(http-based param propagation)**

```bash
Latency Average
  16.55ms
Latency Distribution
  50%   15.15ms
  75%   16.63ms
  90%   18.95ms
  99%   36.09ms
Requests/sec
  5819.30
```

**Invoker(Unix Domain Socket, TCP based param propagation)**

```bash
# Note : the implementation still have huge space to optimize, i.e. JSON serialization.
Latency Average
  8.27ms
Latency Distribution
  50%    8.29ms
  75%    9.12ms
  90%    9.71ms
  99%   10.50ms
Requests/sec
  11590.92

```


## License

This project is licensed under MIT license. ([LICENSE MIT](https://github.com/tz70s/tiny-invoker/blob/master/LICENSE) or http://opensource.org/licenses/MIT)
