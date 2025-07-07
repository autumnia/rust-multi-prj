
# cargo 
    cargo fmt

# cllippy
    cargo clippy

# commit auto check
    .git/hooks/pre_commit 작성
        cargo fmt
        cargo clippy -- -D warnings

# test
### 임의의 크기의 파일 생성
    dd if=/dev/urandom bs=1024 count=128 of=myfile

### 임의의 파일을 읽어 파이브로 전달 후 새로운 파일 생성 및 비교
    cat myfile | target/debug/pipeviewer  > myfile2
    diff myfile file12

### 에코로 전달
    echo "a string" | PV_SILENT=sonething cargo run

### test 
    echo "apple" | cargo run -- -o fruit.txt 
    echo "hello there" | cargo run 
    yes | cargo run | head -n 10000000 > /dev/null

# clap
    [dependencies]
    clap = "2.33.0"
    
    cargo run -- -h

