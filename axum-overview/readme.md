``` sh
# Re-compile on change 
cargo watch -q -c -w src/ -x run 
# Re-run test on change
cargo watch -q -c -w examples/ -x 'run --example quick_dev
```
