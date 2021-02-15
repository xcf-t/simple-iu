use rand::Rng;
use std::path::Path;

pub fn generate_filename(lower: u64, upper: u64, ext: &String, output: &Path, counter: u8) -> String {
    if counter > 250 {
        panic!("Failed to find an id in 250 Steps, consider increasing the id range");
    }

    let id_num = rand::thread_rng().gen_range(lower..=upper);
    let id_str = format_radix(id_num, 36).to_uppercase();

    let file = format!("{}.{}", id_str, ext);

    return if output.join(file.clone()).exists() {
        generate_filename(lower, upper, ext, output, counter + 1)
    } else {
        id_str
    }
}

fn format_radix(mut x: u64, radix: u32) -> String {
    let mut result = Vec::new();

    loop {
        let m = x % radix as u64;
        x = x / radix as u64;

        result.push(std::char::from_digit(m as u32, radix).unwrap());

        if x == 0 {
            break;
        }
    }

    result.into_iter().rev().collect()
}