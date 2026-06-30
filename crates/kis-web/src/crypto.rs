//! KIS 자격증명 봉투암호화 (AES-256-GCM).
//!
//! app_key/app_secret 를 {key,secret} JSON 으로 직렬화한 뒤 마스터키로 인증암호화한다.
//! 행마다 12바이트 nonce 를 새로 생성(GCM nonce 재사용 금지). DB엔 ciphertext+nonce 만 저장.

use aes_gcm::aead::{Aead, KeyInit, OsRng};
use aes_gcm::{Aes256Gcm, Key, Nonce};
use anyhow::{anyhow, Result};
use rand::RngCore;
use serde::{Deserialize, Serialize};

/// 암호화 대상 평문 (DB엔 절대 평문 저장 안 함, 클라이언트로도 미반환).
#[derive(Serialize, Deserialize)]
pub struct Secret {
    pub app_key: String,
    pub app_secret: String,
}

/// 봉투암호화 결과.
pub struct Sealed {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
}

/// 마스터키로 자격증명을 암호화.
pub fn seal(master_key: &[u8; 32], secret: &Secret) -> Result<Sealed> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));

    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = serde_json::to_vec(secret)?;
    let ciphertext = cipher
        .encrypt(nonce, plaintext.as_ref())
        .map_err(|e| anyhow!("암호화 실패: {e}"))?;

    Ok(Sealed {
        ciphertext,
        nonce: nonce_bytes.to_vec(),
    })
}

/// 마스터키로 자격증명을 복호화.
pub fn open(master_key: &[u8; 32], ciphertext: &[u8], nonce: &[u8]) -> Result<Secret> {
    if nonce.len() != 12 {
        return Err(anyhow!("nonce 길이 오류: {}", nonce.len()));
    }
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(master_key));
    let nonce = Nonce::from_slice(nonce);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|e| anyhow!("복호화 실패(마스터키 불일치?): {e}"))?;

    Ok(serde_json::from_slice(&plaintext)?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let key = [7u8; 32];
        let secret = Secret {
            app_key: "PSxxxxxxxxxxxxxxxxxx".to_string(),
            app_secret: "secret-value-여기".to_string(),
        };
        let sealed = seal(&key, &secret).unwrap();
        // ciphertext 안에 평문이 노출되지 않아야
        assert!(!sealed.ciphertext.windows(7).any(|w| w == b"PSxxxxx".as_ref()));
        let opened = open(&key, &sealed.ciphertext, &sealed.nonce).unwrap();
        assert_eq!(opened.app_key, secret.app_key);
        assert_eq!(opened.app_secret, secret.app_secret);
    }

    #[test]
    fn wrong_key_fails() {
        let sealed = seal(&[1u8; 32], &Secret {
            app_key: "a".into(),
            app_secret: "b".into(),
        })
        .unwrap();
        assert!(open(&[2u8; 32], &sealed.ciphertext, &sealed.nonce).is_err());
    }

    #[test]
    fn nonce_is_unique_per_seal() {
        let key = [3u8; 32];
        let s = Secret { app_key: "a".into(), app_secret: "b".into() };
        let a = seal(&key, &s).unwrap();
        let b = seal(&key, &s).unwrap();
        assert_ne!(a.nonce, b.nonce); // nonce 재사용 금지 확인
    }
}
