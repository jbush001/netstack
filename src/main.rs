mod netif;
mod packet;
mod ip;
mod icmp;

fn print_binary(buffer: &[u8]) {
    for (i, byte) in buffer.iter().enumerate() {
        print!("{:02x} ", byte);
        if i % 16 == 15 {
            println!();
        }
    }

    println!();
}

fn main() {
    netif::init();

    loop  {
        let mut pkt = netif::recv_packet();
        println!("Received packet ({} bytes):", pkt.length);
        print_binary(&pkt.data[..pkt.length as usize]);
        ip::ip_recv(&mut pkt);
    }
}
