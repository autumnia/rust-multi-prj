use sysinfo::{System, SystemExt, DiskExt, CpuExt};
use std::{thread, time::Duration, env, io};
use std::collections::VecDeque;
use std::str::FromStr;
use std::net::{UdpSocket, IpAddr};

#[tokio::main]
async fn main() {
    println!( "[모니터링 시스템 시작]");

    dotenv::dotenv().ok();

    let mut cpu_history = VecDeque::with_capacity(get_env_num("ALERT_DURATION", 60));
    let mut mem_history = VecDeque::with_capacity(get_env_num("ALERT_DURATION", 60));
    let mut disk_history = VecDeque::with_capacity(get_env_num("ALERT_DURATION", 60));

    loop {
        let mut sys = System::new_all();
        sys.refresh_all();

        // CPU (전체 평균)
        let cpu_usage = get_cpu_usage(&mut sys);

        // 메모리 사용률
        let mem_usage = get_mem_usage(&mut sys);

        // 디스크 사용률 (전체 평균)
        let disk_usage = get_disk_usage(&mut sys);

        println!("");
        
        // 기록 추가
        push_and_limit(&mut cpu_history, cpu_usage > get_env_num("CPU_THRESHOLD", 80.0));
        push_and_limit(&mut mem_history, mem_usage > get_env_num("MEM_THRESHOLD", 50.0));
        push_and_limit(&mut disk_history, disk_usage > get_env_num("DISK_THRESHOLD", 70.0));

        // 임계치 연속 초과 확인
        if cpu_history.len() == get_env_num("ALERT_DURATION", 60) && cpu_history.iter().all(|&x| x) {
            send_alert("CPU", cpu_usage).await;
            cpu_history.clear();
        }
        if mem_history.len() == get_env_num("ALERT_DURATION", 60) && mem_history.iter().all(|&x| x) {
            send_alert("Memory", mem_usage).await;
            mem_history.clear();
        }
        if disk_history.len() == get_env_num("ALERT_DURATION", 60) && disk_history.iter().all(|&x| x) {
            send_alert("Disk", disk_usage).await;
            disk_history.clear();
        }

        thread::sleep( Duration::from_secs( get_env_num("CHECK_INTERVAL_SECS", 60) ) );
    }
}

fn get_disk_usage(sys: &mut System) -> f32 {
    let disk_usage = sys.disks().iter()
        .map(|d| {
            let total = d.total_space() as f32;
            let used = (d.total_space() - d.available_space()) as f32;
            (used / total) * 100.0
        })
        .fold(0.0, |acc, x| acc + x) / (sys.disks().len().max(1) as f32);
    println!("현재 disk 사용률: {:.2}%", disk_usage);
    disk_usage
}

fn get_mem_usage(sys: &mut System) -> f32 {
    let total_mem = sys.total_memory() as f32;
    let used_mem = sys.used_memory() as f32;
    let mem_usage = (used_mem / total_mem) * 100.0;
    println!("현재 mem 사용률: {:.2}%", mem_usage);
    mem_usage
}

fn get_cpu_usage(sys: &mut System) -> f32 {
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    println!("현재 cpu 사용률: {:.2}%", cpu_usage);
    cpu_usage
}

fn get_hostname() -> io::Result<String> {
    // hostname 을 가져옴
    let hostname = hostname::get().map(|os_str| os_str.to_string_lossy().into_owned());
    // println!("hostname: {}", hostname?);
    hostname
}

fn get_local_ip() -> io::Result<IpAddr> {
    // 임시로 외부 주소와 연결 시도 (UDP 소켓은 실제로 데이터를 보내지 않음)
    // 이를 통해 로컬 IP를 확인
    let socket = UdpSocket::bind("0.0.0.0:0")?;
    socket.connect("8.8.8.8:80")?;
    let local_addr = socket.local_addr()?;
    println!("local_addr: {}", local_addr);
    Ok(local_addr.ip())
}

fn get_env_str<T>(var_name: &str, default_str: &str) -> T
where T: FromStr + std::fmt::Display, <T as FromStr>::Err: std::fmt::Debug, {
    // 기본값 파싱 (프로그램 시작 시 실패하면 panic)
    let default = default_str.parse::<T>().expect("기본값 파싱 실패");

    match env::var(var_name) {
        Ok(val_str) => match val_str.parse::<T>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "⚠️ 환경 변수 '{}' 의 값 '{}' 은 올바른 형으로 변환할 수 없습니다. 기본값 {} 을(를) 사용합니다.",
                    var_name, val_str, default
                );
                default
            }
        },
        Err(_) => default,
    }
}

fn get_env_num<T>(var_name: &str, default: T) -> T
where T: FromStr + std::fmt::Display, <T as FromStr>::Err: std::fmt::Debug, {
    match env::var(var_name) {
        Ok(val_str) => match val_str.parse::<T>() {
            Ok(val) => val,
            Err(_) => {
                eprintln!(
                    "⚠️ 환경 변수 '{}' 의 값 '{}' 은 올바른 형으로 변환할 수 없습니다. 기본값 {} 을(를) 사용합니다.",
                    var_name, val_str, default
                );
                default
            }
        },
        Err(_) => default,
    }
}

fn push_and_limit(queue: &mut VecDeque<bool>, value: bool) {
    if queue.len() == get_env_num("ALERT_DURATION", 60) {
        queue.pop_front();
    }
    queue.push_back(value);
}

async fn send_alert(metric: &str, value: f32) {
    let hostname = get_hostname().unwrap();
    let ip = get_local_ip().unwrap();

    let msg = format!("⚠️ IP:{}, Hostname:{},  {} usage exceeded threshold: {:.2}%", ip, hostname, metric, value);
    println!("send alert: {}", msg);

    // 슬랙 알림 보내기
    // if let Err(e) = send_slack(&msg).await {
    //     eprintln!("Slack 전송 실패: {:?}", e);
    // }

    // 이메일 알림 보내기
    if let Err(e) = send_email(&msg).await {
        eprintln!("이메일 전송 실패: {:?}", e);
    }
    
    println!("");
}

async fn send_slack(message: &str) -> Result<(), ureq::Error> {
    let webhook_url = "https://hooks.slack.com/services/xxx/yyy/zzz"; // 본인의 Webhook으로 교체

    let payload = serde_json::json!({
        "text": message,
    });

    ureq::post(webhook_url)
        .set("Content-Type", "application/json")
        .send_string(&payload.to_string())?;

    Ok(())
}

async fn send_email(message: &str) -> Result<(), Box<dyn std::error::Error>> {
    use lettre::transport::smtp::authentication::Credentials;
    use lettre::{SmtpTransport, Transport};
    use lettre::Message;
    
    let email = Message::builder()
        .from(get_env_str("MAIL_FROM", "kke@mz.co.kr"))
        .to(get_env_str("MAIL_TO", "autumnya@gmail.com"))
        .subject("🔔 System Alert")
        .body(message.to_string())?;

    let creds = Credentials::new("kke@mz.co.kr".to_string(), "wqer qfzp ekks prhq".to_string());

    let mailer = SmtpTransport::relay("smtp.gmail.com")?
        .credentials(creds)
        .build();

    mailer.send(&email)?;

    Ok(())
}
