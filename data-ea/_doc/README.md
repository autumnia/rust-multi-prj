# Rust Data Engineering and Analytics

## rust version
```txt
* 버전: 러스트 버전 1.81 이상 필요
* 참고: https://github.com/databora/elusion

// 조회
rustup show

// 변경
rustup default stable-x86_64-pc-windows-msvc

// 설치
rustup install stable-x86_64-pc-windows-msvc
```

## cargo.toml 설정 추가
```toml
[dependencies]
#elusion="3.3.0"
elusion = { version = "3.11.0", features = ["all"] }
tokio={version = "1.42.0", features = ["rt-multi-thread"]}
```

## cargo expand 설치 필요
```txt
cargo-expand v1.0.113 
cargo run --features odbc
```

## 초기 메인 구조
```rust
use elusion::prelude::*;

#[tokio::main]
async fn main() -> ElusionResult<()> {
    println!("Hello Elusion!");
    Ok(())
} 

```