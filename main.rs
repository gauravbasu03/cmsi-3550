fn main() {
    let sampleinput = "XXXX:XXXX:XXXX:XXXX:XXXX:XXXX:XXXX:XXXX";
    let parts: Vec<&str> = sampleinput.split(':').collect();

    if parts.len() == 8 {
        let (part1, part2, part3, part4, part5, part6, part7, part8) =
            (parts[0], parts[1], parts[2], parts[3], parts[4], parts[5], parts[6], parts[7]);

        println!("Part 1: {}", part1);
        println!("Part 2: {}", part2);
        println!("Part 3: {}", part3);
        println!("Part 4: {}", part4);
        println!("Part 5: {}", part5);
        println!("Part 6: {}", part6);
        println!("Part 7: {}", part7);
        println!("Part 8: {}", part8);
    }
}
