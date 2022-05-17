use itertools::Itertools;

fn main() {
    //build_serial(&bruteforce())
}

fn bruteforce() -> Vec<usize> {
    let mut result = Vec::with_capacity(19);
    for j in 0..4 {
        // make all pattern which is of size 5 bytes and between 0x41 to 0x7A.
        if j != 3 {
            let comb = (0x41..0x7A).combinations_with_replacement(5).find(|f | {
                find_valid(f, j, 5)
            });
            result.append(&mut comb.unwrap());
        } else {
            // the last pattern is of size 4 bytes.
            let comb = (0x41..0x7A).combinations_with_replacement(4).find(|f | {
                find_valid(f, j, 4)
            });
            result.append(&mut comb.unwrap());
        }
    }
    result
}

fn find_valid(f: &Vec<usize>, start: usize, array_size: usize) -> bool {
    let factor = [0x5F5, 0x4E8, 0x185C, 0x1CAD]; // Numbers to be used in calculation
    let encrypted_key = [0x999, 0x640, 0x0DCF, 0x686]; // The result of the calculation below must be matched with this.
    let mut valid_key = 0;
    for i in start..19 as usize {
        let v13 = ((i ^ (factor[i % 4] / 4)) ^ (f[i % array_size] * f[i % array_size]) + 0xFFFFFFF) % 0x3E8;
        valid_key += v13;
    }
    if valid_key == encrypted_key[start as usize] {
        return true;
    }
    false
}

// order the input in relevant order.
// 0, 4, 8, 12, 16
// 1, 5, 9, 13, 17
// 2, 6, 10, 14, 18
// 3, 7, 11, 15
fn build_serial(serial_key_array_unordered: &Vec<usize>) {
    println!("{:?}", serial_key_array_unordered);
    let mut serial_key_array: [usize; 19] = Default::default();
    for i in 0..19 {
        serial_key_array[(i % 5) * 4 + i / 5] = serial_key_array_unordered[i];
    }
    println!("{:?}", serial_key_array);
    let serial = serial_key_array.map(|f| {
        f as u8 as char
    });
    println!("serial code: {:?}", serial.iter().join(""));
}
