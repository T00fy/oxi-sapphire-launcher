use anyhow::anyhow;
use base64::Engine;
use base64::engine::general_purpose::URL_SAFE;
use log::debug;
use physis::blowfish::Blowfish;
use sysinfo::System;

/// Heavily inspired by astra's implementation
pub(crate) fn encrypt_game_arg(arg: &str) -> anyhow::Result<String> {
    let ticks = tick_count();
    debug!("Raw ticks: {:?}", ticks);
    let key = ticks & 0xFFFF0000;

    // Format key as an 8-character hexadecimal string, similar to `sprintf(buffer, "%08x", key);`
    let key_str = format!("{:08x}", key);
    debug!("Key Hex: {}", key_str);

    // Convert the key string to bytes
    let key_bytes = key_str.as_bytes();

    // Initialize Blowfish with the key
    let blowfish = Blowfish::new(key_bytes);

    // Prepare data to encrypt
    let to_encrypt = format!(" /T ={}{}", ticks, arg);
    debug!("Data to Encrypt: {}", to_encrypt);
    let to_encrypt_bytes = to_encrypt.as_bytes();
    // Encrypt the data using Blowfish
    let encrypted_data = blowfish.encrypt(to_encrypt_bytes)
        .ok_or_else(|| anyhow!("Blowfish encryption failed"))?;

    debug!("Encrypted Data: ");
    for byte in encrypted_data.iter() {
        debug!("{:02x}", byte);
    }

    let mut base64_output = String::new();

    // Encode encrypted data in Base64
    URL_SAFE.encode_string(&encrypted_data, &mut base64_output);
    debug!("Base64 Output: {}", base64_output);

    // Calculate checksum
    let checksum = get_checksum(key);
    debug!("Checksum: {}", checksum);

    // Format the final encrypted string
    let s = format!("//**sqex0003{}{}**//", base64_output, checksum);
    debug!("final output: {}", s);
    Ok(s)
}

fn tick_count() -> u32 {
    (System::uptime() * 1000) as u32
}


fn get_checksum(key: u32) -> char {
    // Define the checksum table
    let checksum_table: [char; 16] = ['f', 'X', '1', 'p', 'G', 't', 'd', 'S', '5', 'C', 'A', 'P', '4', '_', 'V', 'L'];

    // Compute the index into the checksum table
    let index = (key & 0x000F0000) >> 16;

    // Return the checksum character
    checksum_table[index as usize]
}
