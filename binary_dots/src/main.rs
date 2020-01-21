fn main() {
    let name = "JUSTINBROWNE";
    for (i, c) in name.as_bytes().iter().enumerate() {
        let binary = format!("{:08b}", c);
        for (j, b) in binary.chars().enumerate() {
            match b {
                '0' => { }
                '1' => println!("x=\"{}\" y=\"{}\"", (i as i32) - 6, (j as i32) - 4),
                _ => unreachable!(),
            }
        }
    }
}
