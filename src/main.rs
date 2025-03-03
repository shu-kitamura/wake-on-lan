use std::{
    net::UdpSocket,
    num::ParseIntError
};
use clap::Parser;

fn main() {
    // コマンドライン引数をパースし、MAC アドレスを取得する
    let args = Args::parse();
    let mac_address: String = args.mac_address;

    // Magic Packet を生成する
    let magic_packet: Vec<u8> = match create_magic_packet(&mac_address) {
        Ok(packet) => packet,
        Err(e) => { // エラーは表示して、プログラムを終了する
            eprintln!("Error: {}", e);
            return;
        }
    };
    
    // Magic Packet を送信する
    match send_magic_packet(magic_packet) {
        Ok(_) => println!("Magic Packet sent successfully"),
        Err(e) => eprintln!("Error: {}", e)
    }
}

// --- コマンドライン引数の定義 ---
#[derive(Debug, Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(value_name = "MAC_ADDRESS")]
    mac_address: String,
}

// MAC アドレスを受け取り、その MAC アドレスに対応する Magic Packet を生成する
fn create_magic_packet(mac_address: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut magic_packet: Vec<u8> = vec![0xff; 6];
    for _ in 0..16 {
        let parsed_address: Vec<u8> = parse_mac_address(mac_address)?;
        magic_packet.extend_from_slice(&parsed_address);
    }
    Ok(magic_packet)
}

// MAC アドレスを 16 進数のベクタに変換する
fn parse_mac_address(mac_address: &str) -> Result<Vec<u8>, ParseIntError> {
    let mut parsed: Vec<u8> = Vec::new();
    for octet in mac_address.split(":") {
        parsed.push(u8::from_str_radix(octet, 16)?);
    }
    Ok(parsed)
}

// Magic Packet をブロードキャストアドレス(udp の 9 番)に送信する
fn send_magic_packet(magic_packet: Vec<u8>) -> Result<usize, std::io::Error> {
    let socket: UdpSocket = UdpSocket::bind("0.0.0.0:0")?;
    socket.set_broadcast(true)?;
    socket.send_to(&magic_packet, "255.255.255.155:9")
}

// --- テストコード ---

#[cfg(test)]
mod tests {
    #[test]
    fn test_create_magic_packet() {
        let mac_address = "00:11:22:33:44:55";
        let magic_packet = super::create_magic_packet(mac_address).unwrap();
        assert_eq!(magic_packet.len(), 102);

        let expected: Vec<u8> = vec![
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
            0x00, 0x11, 0x22, 0x33, 0x44, 0x55,
        ];
        assert_eq!(magic_packet, expected);
    }

    #[test]
    fn test_parse_mac_address() {
        let mac_address = "00:11:22:33:44:55";
        let parsed = super::parse_mac_address(mac_address).unwrap();
        assert_eq!(parsed.len(), 6);

        let expected: Vec<u8> = vec![0x00, 0x11, 0x22, 0x33, 0x44, 0x55];
        assert_eq!(parsed, expected);
    }
}