


fn print_row(row: u8) {
    for i in (0..=7).rev() {
        let n = row & (1 << i);
        match n {
            0 => print!("."),
            _ => print!("*"),
        }
            
    }
    print!("\n")
}

fn rule110(row: u8) {
    for i in (0..=7).rev() {
        let mut b: u8 = 0;
        b |= row & (1 << (i + 7) % 8);
        b |= row & (1 << i);
        b |= row & (1 << (i + 1) % 8);
        println!("{:#08b}", b);
        //match b {
        //    0 => print!("."),
        //    _ => print!("*"),
        //}
    }
    print!("\n")

}





fn main() {
    let row : u8 = 0b10100100u8;

   // for i in 0..8 {
   //     println!("Hello {} {} {}", i, (i + 1) % 8, (i + 7) % 8);
   // }

    print_row(row);
    rule110(row);
}
