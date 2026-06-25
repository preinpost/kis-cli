use std::fmt;

#[derive(Debug)]
pub enum KisError {
    Auth(String),
    Api { message: String, rt_cd: String, msg_cd: String },
    Config(String),
    WebSocket(String),
}

impl fmt::Display for KisError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KisError::Auth(msg) => write!(f, "인증 오류: {msg}"),
            KisError::Api { message, rt_cd, msg_cd } => {
                write!(f, "API 오류 [rt_cd={rt_cd}, msg_cd={msg_cd}]: {message}")
            }
            KisError::Config(msg) => write!(f, "설정 오류: {msg}"),
            KisError::WebSocket(msg) => write!(f, "WebSocket 오류: {msg}"),
        }
    }
}

impl std::error::Error for KisError {}
