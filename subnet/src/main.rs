use std::env::args;

fn main() {

    for (i, arg) in args().enumerate() {
        println!("Arg {i}: {arg}");

        if i == 1 {
            calculate_subnet(arg);
        }
    }
}

fn calculate_subnet(target_subnet: String) {
    let parts = target_subnet.split("/");
    for part in parts {
        println!("Subnet part: {part}");
    }

    let [target_ip, cidr]: [&str; 2] = target_subnet
        .split("/")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    println!("Target IP: {target_ip}");
    println!("CIDR: {cidr}");

    let [o1, o2, o3, o4]: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    println!("1 Octet: {o1}");
    println!("2 Octet: {o2}");
    println!("3 Octet: {o3}");
    println!("4 Octet: {o4}");

    let octet_strings: [&str; 4] = target_ip
        .split(".")
        .collect::<Vec<&str>>()
        .try_into()
        .unwrap_or_default();

    for octet in octet_strings {
        let my_octet: i32 = octet.parse().unwrap();
        let octet_math = my_octet + 1;
        println!("{octet_math}");
    }
}