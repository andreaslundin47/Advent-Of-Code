fn main() {
    const PREFIX: &str = "bgvyzdsv";

    let mut i = 0;

    loop {
        let input = format!("{PREFIX}{i}");
        let digest = md5::compute(input);
        let hash: String = format!("{digest:x}");

        if hash.starts_with("00000") {
            println!("Part 1. Key = {}", i);
            break;
        }

        i += 1;
    }

    loop {
        let input = format!("{PREFIX}{i}");
        let digest = md5::compute(input);
        let hash: String = format!("{digest:x}");

        if hash.starts_with("000000") {
            println!("Part 2. Key = {}", i);
            break;
        }

        i += 1;
    }
}
