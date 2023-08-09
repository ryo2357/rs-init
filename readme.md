# mylogger

ロガーの初期設定ライブラリ

## Usage

```toml:Cargo toml
[dependencies]
mylogger = {git = "https://github.com/ryo2357/rs-mylogger"}
```

```rust
use log::{debug, error, info, trace, warn};

fn main() {
    mylogger::init();

    trace!("trace message");
    debug!("debug message");
    info!("informational message");
    warn!("warning message");
    error!("this is an error {}", "message");
}

```

## Specification

### ログファイルの出力先

./log

### レベルフィルター

dev-標準出力:debug

dev-ファイル出力:trace

release-標準出力:warn

release-ファイル出力:info

### 使い分け

- trace:開発中の詳細なログ
- debug：開発時に標準出力に表示して検証する
- info：稼働中に収集するログ
- warn：稼働中に出力する警告
- error：強制終了直前に出力
