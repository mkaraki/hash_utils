use std::io::Read;

fn read_data(filename: &str) -> Vec<u8> {
    if filename == "-" {
        let mut data = Vec::new();
        std::io::stdin().read_to_end(&mut data).unwrap();
        data
    } else {
        std::fs::read(filename).unwrap()
    }
}

fn main() {
    let mut filename: String = String::new();
    let mut skip_hash_size_optimize = false;
    {
        let mut parser = argparse::ArgumentParser::new();
        parser.set_description("Hasher CLI");
        parser.refer(&mut filename)
            .add_argument("filename", argparse::Store, "File to hash");
        parser.refer(&mut skip_hash_size_optimize)
            .add_option(&["--skip-hash-hex-size-optimize"], argparse::StoreTrue, "Skip hash hex size optimization (if 3bit hash, it will show 1 byte hex)");
        parser.parse_args_or_exit();
    }

    if filename == "" {
        eprintln!("Filename is required");
        std::process::exit(1);
    }

    let data = read_data(&*filename);

    let hashers = hash_utils::all_hashers();
    let max_hasher_name = hashers.iter().map(|h| h.hash_name().len()).max().unwrap();
    for hasher in hashers {
        let padded_hasher_name = format!("{:width$}", hasher.hash_name(), width = max_hasher_name);
        let hash = hasher.hash(&data);
        let mut hash_string = hash.iter().map(|b| format!("{:02X}", b)).collect::<String>();

        if !skip_hash_size_optimize {
            let active_bits = hasher.active_bits();
            let mut active_hex_size = active_bits / 4;
            if active_bits % 4 != 0 {
                active_hex_size += 1;
            }
            
            if (hash_string.len() as i64) - (active_hex_size as i64) > 0 {
                let skip_hex_size = hash_string.len() - active_hex_size as usize;
                hash_string = hash_string[skip_hex_size..].to_string();
            }
        }

        println!("{}: {}", padded_hasher_name, hash_string);
    }
}