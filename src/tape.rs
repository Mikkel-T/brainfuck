/// Print the current state of the tape around the current pointer index
pub fn print_tape(tape: [u8; 30000], ptr: usize) {
    let start;
    let end;
    if ptr < 6 {
        start = 0;
        end = 11;
    } else if ptr > 29994 {
        start = 29989;
        end = 30000;
    } else {
        start = ptr - 5;
        end = ptr + 6;
    }

    let arrow_pos = 3 + (ptr - start) * 6;
    println!("{}|", " ".repeat(arrow_pos + 6));
    println!("{}V", " ".repeat(arrow_pos + 6));
    println!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+");
    print!("| IDX ");
    for i in start..end {
        print!("|");
        let num_len = i.to_string().len() as f32;
        let before = ((5. - num_len) / 2.).ceil() as usize;
        let after = ((5. - num_len) / 2.).floor() as usize;
        print!("{}{}{}", " ".repeat(before), i, " ".repeat(after));
    }
    println!("|");
    println!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+");
    print!("| VAL ");
    for num in tape.iter().take(end).skip(start) {
        print!("|");
        let num_len = num.to_string().len() as f32;
        let before = ((5. - num_len) / 2.).ceil() as usize;
        let after = ((5. - num_len) / 2.).floor() as usize;
        print!("{}{}{}", " ".repeat(before), num, " ".repeat(after));
    }
    println!("|");
    println!("+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+-----+");
    println!("{}^", " ".repeat(arrow_pos + 6));
    println!("{}|", " ".repeat(arrow_pos + 6));
}
