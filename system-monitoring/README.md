# 중요설정

## rust 환경설정
```txt
* windows, linux 
rustup show
rustup install 필요한것
rustup default stable-x86_64-pc-windows-gnu    <= 리눅스
rustup default stable-x86_64-pc-windows-msvc   <= 윈도우

* static build
for linux
rustup target add x86_64-pc-windows-gnu
cargo build --release --target x86_64-pc-windows-gnu  <= 위가 실패하면

cargo build --release  <== 리눅스에서 직접 빌드 

for winodws
rustup target add x86_64-pc-windows-msvc
cargo build --release --target x86_64-pc-windows-msvc
```

## toml 내용
```toml
[profile.dev]
lto = false

[profile.release]
lto = false

[target.x86_64-pc-windows-msvc]
linker = "rust-lld"

[dependencies]
sysinfo = "0.29"
tokio = { version = "1", features = ["full"] }
serde_json = "1.0"

lettre = { version = "0.10", default-features = false, features = ["builder", "smtp-transport", "tokio1-native-tls"] }
ureq = { version = "2.5", default-features = false, features = ["tls"] }
```

## 환경설정
```txt

* windows
set CPU_THRESHOLD=85
set MEM_THRESHOLD=60
set DISK_THRESHOLD=75
```


## 이메일 설정
```txt
- 앱 비밀번호 생성 (Google 계정에서 직접 해야 함)
-  Google 계정 보안 설정 페이지 접속
- "2단계 인증" 활성화 (이미 활성화되어 있어야 함)
- "앱 비밀번호(App Passwords)" 섹션 클릭
- 앱 이름: Rust Mailer (예시), 기기: 기타
- 생성된 16자리 비밀번호를 비밀번호.to_string() 자리에 사용
```

