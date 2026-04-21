# JSY-194-MK Multi Platform Rust Driver

## Testing

Some tests currently fail due to the hardware having to be plugged in as such they will fail when running this default command. Single threaded is required, due to hardware access.

```bash
cargo test --no-fail-fast -- --test-threads=1
```
