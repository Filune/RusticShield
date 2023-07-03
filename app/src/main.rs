use std::fs;
use std::env;
use std::fs::File;
use std::io::Write;

/// Encrypts a single character by shifting it by the specified amount.
///
/// If the character is an ASCII alphabetic character, it is shifted based on its case
/// (lowercase or uppercase). Non-alphabetic characters are left unchanged.
///
/// # Examples
///
/// ```
/// let encrypted_char = encrypt_character('a', 8);
/// assert_eq!(encrypted_char, 'i');
/// ```
fn encrypt_character(c: char, shift: u8) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
        let shifted_value = (c as u8 - base + shift) % 26 + base;
        shifted_value as char
    } else {
        c
    }
}

/// Decrypts a single character by shifting it in the opposite direction of the encryption.
///
/// If the character is an ASCII alphabetic character, it is shifted based on its case
/// (lowercase or uppercase). Non-alphabetic characters are left unchanged.
///
/// # Examples
///
/// ```
/// let decrypted_char = decrypt_character('i', 8);
/// assert_eq!(decrypted_char, 'a');
/// ```
fn decrypt_character(c: char, shift: u8) -> char {
    if c.is_ascii_alphabetic() {
        let base = if c.is_ascii_lowercase() { b'a' } else { b'A' };
        let shifted_value = (c as u8 - base + 26 - shift) % 26 + base;
        shifted_value as char
    } else {
        c
    }
}

/// Encrypts the given plaintext using a Caesar cipher with the specified shift value.
///
/// Each character in the plaintext is encrypted individually using the `encrypt_character` function.
///
/// # Examples
///
/// ```
/// let encrypted_text = encrypt("Hello, World!");
/// assert_eq!(encrypted_text, "Pmttw, Czggi!");
/// ```
fn encrypt(plaintext: &str) -> String {
    plaintext.chars().map(|c| encrypt_character(c, 8)).collect()
}

/// Decrypts the given ciphertext using a Caesar cipher with the specified shift value.
///
/// Each character in the ciphertext is decrypted individually using the `decrypt_character` function.
///
/// # Examples
///
/// ```
/// let decrypted_text = decrypt("Pmttw, Czggi!");
/// assert_eq!(decrypted_text, "Hello, World!");
/// ```
fn decrypt(ciphertext: &str) -> String {
    ciphertext.chars().map(|c| decrypt_character(c, 8)).collect()
}

/// Writes the given content to a file at the specified file path.
///
/// # Examples
///
/// ```
/// let content = "Hello, World!";
/// let file_path = "output.txt";
/// write_to_file(content, file_path).expect("Failed to write the file");
/// ```
fn write_to_file(content: &str, file_path: &str) -> std::io::Result<()> {
    let mut file = File::create(file_path)?;
    file.write_all(content.as_bytes())?;
    Ok(())
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Usage: ./program <file_path> <operation>");
        return;
    }

    let file_path = &args[1];
    let operation = &args[2];

    let contents = match fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(error) => {
            println!("Failed to read the file: {}", error);
            return;
        }
    };
    
    let result = match operation.as_str() {
        "encrypt" => encrypt(&contents),
        "decrypt" => decrypt(&contents),
        _ => {
            println!("Invalid operation");
            return;
        }
    };

    let output_file_path = format!("{}-output.txt", operation);
    if let Err(error) = write_to_file(&result, &output_file_path) {
        println!("Failed to write the result to the file: {}", error);
        return;
    }

    println!("Operation completed successfully. Result written to {}", output_file_path);
}