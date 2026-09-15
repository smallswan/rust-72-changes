// NOTE: example requires `getrandom` feature is enabled
use hex::encode;
use hex_literal::hex;
use pbkdf2::{pbkdf2_hmac, pbkdf2_hmac_array};
use sha2::Sha256;

fn main() -> Result<(), Box<dyn core::error::Error>> {
    let password = b"password";
    let salt = b"salt";
    // number of iterations
    let n = 600_000;
    // Expected value of generated key
    let expected = hex!("669cfe52482116fda1aa2cbe409b2f56c8e45637");

    let mut key1 = [0u8; 20];
    pbkdf2_hmac::<Sha256>(password, salt, n, &mut key1);
    assert_eq!(key1, expected);

    println!("key1: {:?}", encode(&key1));

    let key2 = pbkdf2_hmac_array::<Sha256, 20>(password, salt, n);
    assert_eq!(key2, expected);

    use aes_gcm::{
        Aes256Gcm,
        Nonce, // Or `Aes128Gcm`
        aead::{AeadCore, AeadInOut, Generate, Key, KeyInit},
    };
    use arrayvec::ArrayVec;

    let key = Key::<Aes256Gcm>::generate();
    let cipher = Aes256Gcm::new(&key);

    let nonce = Nonce::generate(); // MUST be unique per message
    let mut buffer: ArrayVec<u8, 128> = ArrayVec::new(); // Note: buffer needs 16-bytes overhead for auth tag
    buffer.try_extend_from_slice(b"18012341234").unwrap();

    // Encrypt `buffer` in-place, replacing the plaintext contents with ciphertext
    cipher.encrypt_in_place(&nonce, b"", &mut buffer)?;

    // `buffer` now contains the message ciphertext
    assert_ne!(buffer.as_ref(), b"18012341234");

    let hex_str = encode(buffer.as_slice());
    println!("{}", hex_str);

    use base64::engine::{Engine as _, general_purpose};
    let b64 = general_purpose::STANDARD.encode(buffer.as_slice());
    println!("{}", b64);

    // Decrypt `buffer` in-place, replacing its ciphertext context with the original plaintext
    cipher.decrypt_in_place(&nonce, b"", &mut buffer)?;
    assert_eq!(buffer.as_ref(), b"18012341234");

    Ok(())
}
