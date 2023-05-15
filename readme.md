# mylogger

ロガーの初期設定

## Usage

```toml:Cargo toml
[dependencies]
mylogger = {git = "https://github.com/ryo2357/rs-mylogger"}
```

```rust
use mylogger::{init,debug, error, info, warn};

fn main() {
    init();

    debug!("debug message");
    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");

}
```
