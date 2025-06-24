# Axum

## 공식문서 
```
https://crates.io
https://crates.io/crates/axum
https://crates.io/crates/cargo-watch
https://crates.io/crates/sqlx
https://crates.io/crates/serde
https://crates.io/crates/web-sys
https://crates.io/crates/gloo-console
https://crates.io/crates/reqwest

https://yew.rs/docs/getting-started/
https://yew.rs/docs/concepts/router
```

## 신규 프로젝트 생성
```
cargo new --bin server --vcs none
cargo run
```

## cargo watch 추가
```
소스 수정 후 서버 기동 없이 반영됨 ( 개발용 )
1.  cargo install cargo-watch
2.  cargo add cargo-watch
3.  cargo watch -x run
    cargo watch -q -c -w src/ -x 'run -q'
```

## 의존성 추가
```
[dependencies]
tokio = { version = "1.37.0", features = ["full"] }

tracing = "0.1.40"
tracing-subscriber = "0.3.18"

axum = "0.7.5"

// 개발용
cargo-watch = "8.5.2"

// cors
tower = "0.4.13"
tower-http = { version = "0.5.2", features = ["full"] }

// database


```

## 기본 구성
```rust
use axum::{
    routing::{get},
    Router
    // http::StatusCode,
    // Json
};

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // router
    let app = Router::new()
        .route("/", get(root));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();
    axum::serve(listener, app)
        .await.unwrap();

    tracing::debug!( "listening on port {}", "0.0.0.0:3000" );
    println!("listening on port {}", "0.0.0.0:3000");
}

async fn root() -> &'static str {
    "Hello, Axum api"
}

```

```dockerfile
docker run -it \
--name postgres -h postgres \
-p 5432:5432 \
-e POSTGRES_USER=postgres \
-e POSTGRES_PASSWORD=0823 \
-e POSTGRES_DB=postgres \
-e PGDATA=/var/lib/postgresql/data/pgdata \
-v D/gitroot/data/postgres:/var/lib/postgresql/data \
-d postgres:latest
```

## yew 
```rust
    rustup target add wasm32-unknown-unknown
    cargo install trunk

    [dependencies]
    yew = { version="0.20.0", features=["csr"] }

[templete]
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <h1> Yew app</h1>
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

[실행]
cargo run --target wasm32-unknown-unknown flag
trunk serve
```