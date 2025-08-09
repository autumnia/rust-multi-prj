
# Rust Web Service

## 프로젝트 실행
```rust
* workspace root 에서 실행시
    cargo run -p --bin 프로젝트명
    cargo run -p 프로젝트명   <=  toml 파일에 default 설정시
```

## 테스트 명령어
```rust
    cd 프로젝트폴더 
    cargo test --bin 프로젝트명
    cargo test 테스트함수명 --bin 프로젝트명
```

## Database 설정
```dockerfile~~~~
    docker run -it --name postgres -h postgres -p 5432:5432 -e POSTGRES_USER=postgres -e POSTGRES_PASSWORD=0823 -e PGDATA=/var/lib/postgresql/data/pgdata -v /tmp:/var/lib/postgresql/data -d postgres:latest
```

