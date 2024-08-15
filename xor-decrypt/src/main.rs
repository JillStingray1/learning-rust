use base64::prelude::*;

fn main() {
    let encoded_str = BASE64_STANDARD.decode("KxgLAE4HCE4XARtOCgEASRpOBQABGU4aBgtOBQsXQk4dDwgLThoBTg8dHRsDC04HGkkdTgEAC04MFxoLTgIBAAlA").unwrap();
    for key in 0..u8::MAX {
        let string = String::from_utf8(
            encoded_str
                .iter()
                .map(|byte| byte ^ key)
                .collect::<Vec<u8>>(),
        );
        match string {
            Ok(x) => println!("key:{:02X?}, {}", key, x),
            Err(_) => (),
        }
    }
}
