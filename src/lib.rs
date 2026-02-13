use wasm_bindgen::prelude::*;

// 导入 console.log 用于调试
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

// 定义一个宏来简化 console.log 的使用
macro_rules! console_log {
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

// 简单的 XOR 加密算法
fn xor_encrypt(text: &str, key: &str) -> String {
    let text_bytes = text.as_bytes();
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();
    
    for (i, &byte) in text_bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        result.push(byte ^ key_byte);
    }
    
    // 转换为十六进制字符串
    result.iter().map(|b| format!("{:02x}", b)).collect()
}

// XOR 解密算法
fn xor_decrypt(hex_string: &str, key: &str) -> Result<String, String> {
    // 将十六进制字符串转换为字节数组
    let mut bytes = Vec::new();
    for i in (0..hex_string.len()).step_by(2) {
        if i + 1 < hex_string.len() {
            let hex_pair = &hex_string[i..i + 2];
            match u8::from_str_radix(hex_pair, 16) {
                Ok(byte) => bytes.push(byte),
                Err(_) => return Err("Invalid hex string".to_string()),
            }
        } else {
            return Err("Invalid hex string length".to_string());
        }
    }
    
    let key_bytes = key.as_bytes();
    let mut result = Vec::new();
    
    for (i, &byte) in bytes.iter().enumerate() {
        let key_byte = key_bytes[i % key_bytes.len()];
        result.push(byte ^ key_byte);
    }
    
    match String::from_utf8(result) {
        Ok(s) => Ok(s),
        Err(_) => Err("Invalid UTF-8 sequence".to_string()),
    }
}

// 导出到 JavaScript 的函数
#[wasm_bindgen]
pub fn encrypt_string(text: &str, key: &str) -> String {
    console_log!("Encrypting: {}", text);
    xor_encrypt(text, key)
}

#[wasm_bindgen]
pub fn decrypt_string(encrypted_text: &str, key: &str) -> Result<String, JsValue> {
    console_log!("Decrypting: {}", encrypted_text);
    match xor_decrypt(encrypted_text, key) {
        Ok(result) => Ok(result),
        Err(e) => Err(JsValue::from_str(&e)),
    }
}

// 生成随机密钥的函数
#[wasm_bindgen]
pub fn generate_key(length: usize) -> String {
    // 使用简单的字符集生成密钥
    let chars = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789";
    let mut result = String::new();
    
    // 确保长度在合理范围内
    let actual_length = length.min(32).max(1);
    
    // 使用简单的伪随机方法
    let mut seed = 12345u64; // 简单种子
    for _ in 0..actual_length {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        let index = (seed % chars.len() as u64) as usize;
        result.push(chars.chars().nth(index).unwrap_or('a'));
    }
    
    result
}

// 当模块被加载时调用
#[wasm_bindgen(start)]
pub fn main() {
    console_log!("Rust WebAssembly crypto module loaded!");
}
