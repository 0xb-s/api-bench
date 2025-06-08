# api-bench

**`api-bench`** is a fast, lightweight, and extensible HTTP load testing tool.
It allows you to benchmark REST APIs with custom headers, request bodies, and full per-request logging — with support for JSON/CSV export and real-time progress tracking.


## 🔥 Features
- 🔁 **Any HTTP method**: `GET`, `POST`, `PUT`, `DELETE`, `PATCH`, etc.
- 📦 **Custom headers and body file support**
- ⚙️ **Configurable concurrency and duration**
- 📉 **Latency statistics**: mean, p50, p95, p99
- 📄 **Per-request logging**: raw results saved in CSV or JSON

### Install

```bash
git clone https://github.com/0xb-s/api-bench
cd api-bench
cargo build --release
```

##  Basic usage
```bash
./target/release/api-bench https://httpbin.org/get -c 10 --requests 100 -o result.json
```

## 🛠 Options

- **`-c`, `--concurrency`**  
  Number of concurrent requests to send in parallel.  
  _Default: 10_

- **`-d`, `--duration`**  
  Duration of the benchmark run (e.g. `30s`, `1m`).  
  _Mutually exclusive with `--requests`._

- **`--requests`**  
  Total number of requests to send before stopping.  
  _Mutually exclusive with `--duration`._

- **`--method`**  
  HTTP method to use (e.g. `GET`, `POST`, `PUT`, `DELETE`, etc).  
  _Default: GET_

- **`--body-file`**  
  Path to a file containing the request body (useful for `POST`, `PUT`, etc).

- **`-H`, `--header`**  
  Custom HTTP headers. Can be repeated.  
  _Example: `-H "Content-Type: application/json"`_

- **`-o`, `--output`**  
  Path to the output file where results will be saved.

- **`-f`, `--output-format`**  
  Format of the output file:  
  - `json` (default)  
  - `csv`


