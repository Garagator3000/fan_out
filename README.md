# Fan-out UDP Sharder (Rust experiment)

This is a small experimental project built while learning Rust and systems programming patterns.

It is not a production-ready system. It is a prototype for understanding:

* UDP ingestion
* channel-based concurrency in Rust
* consistent hashing / sharding
* file-based sinks
* basic configuration handling with YAML
* structured logging

---

## What it does

The service:

1. Listens for UDP messages on a configured port
2. Receives raw packets
3. Hashes each message
4. Routes it to a shard (file)
5. Writes data into shard files

Each shard corresponds to a separate file.

---

## Architecture

```
UDP Transport
      ↓
channel (mpsc)
      ↓
worker thread
      ↓
hash-based sharder
      ↓
file sink (per shard)
```

---

## Configuration

The service is configured via YAML file.

Example:

```yaml
log_level: debug

app:
  port: 3000
  shards: 3
  file_pattern: "file_shard_{}.log"
  file_path: "./tests"
```

### Fields

* `log_level` — logging level (`debug`, `info`, `warn`, `error`)
* `app.port` — UDP listen port
* `app.shards` — number of output shards (files)
* `app.file_pattern` — file naming pattern, must include `{}` placeholder
* `app.file_path` — directory where shard files will be created

---

## Running

Build:

```bash
cargo build --release
```

Run:

```bash
cargo run -- -c configs/local.yaml
```

or:

```bash
./target/release/fan_out -c configs/local.yaml
```

---

## Sending test data

You can use netcat:

```bash
nc -u 127.0.0.1 3000
```

Or a simple Python script:

```python
import socket

sock = socket.socket(socket.AF_INET, socket.SOCK_DGRAM)

for i in range(100000):
    sock.sendto(str(i).encode(), ("127.0.0.1", 3000))
```

---

## Output

Messages are distributed across shard files:

```
file_shard_0.log
file_shard_1.log
file_shard_2.log
```

Distribution is based on a simple FNV-1a hash function.

---

## Limitations

This project is intentionally minimal:

* no backpressure handling
* no batching
* no zero-copy optimizations
* no persistence guarantees
* naive error handling
* no graceful shutdown yet

It is meant for learning, not production use.

---

## Purpose

This project exists purely as a learning exercise in Rust systems programming:

* ownership and borrowing
* concurrency with channels
* IO handling
* basic network programming
* structuring a small service

---

## Future ideas

* batching + buffered flush strategy
* TCP transport support
* metrics (Prometheus)
* graceful shutdown
* zero-copy buffers (`bytes`)
* dynamic sharding
* Kafka/Postgres sinks

---

## License

This is a personal learning project. No warranty.
