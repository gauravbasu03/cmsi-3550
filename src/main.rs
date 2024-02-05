use ipconfig::get_adapter_information;
// uses ipconfign crate
fn main() {
    if let Ok(adapters) = get_adapter_information() {
        for adapter in adapters {
            // IPv4 
            for ip in &adapter.ip_addresses {
                if ip.is_ipv4() {
                    println!("IPv4: {}", ip);
                }
            }

            // IPv6
            for ip in &adapter.ip_addresses {
                if ip.is_ipv6() {
                    println!("IPv6: {}", ip);
                }
            }
        }
    } else {
        println!("idk what happened");
    }
}
