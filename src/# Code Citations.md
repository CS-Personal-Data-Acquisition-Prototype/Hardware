# Code Citations

## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() ->
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(Atomic
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false
```


## License: Apache-2.0
https://github.com/ssnover/helles/blob/42a1c2b514d420d986e2e351a4129a80b9d26e48/examples/test-server.rs

```
;

fn main() -> std::io::Result<()> {
    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();

    ctrlc::set_handler(move || {
        r.store(false, Ordering:
```

