use anyhow::Result;

use crate::client::KisClient;

pub async fn run(client: &KisClient) -> Result<()> {
    println!("토큰 발급 중...");

    let token = client.token_manager.get_token().await?;
    println!("  토큰 타입: {}", token.token_type);
    println!("  만료 시간: {}", token.access_token_token_expired);
    println!(
        "  토큰 (앞 20자): {}...",
        &token.access_token[..20.min(token.access_token.len())]
    );

    println!("\nWebSocket approval key 발급 중...");
    let ws_token = client.token_manager.get_ws_token().await?;
    println!("  approval_key: {}", ws_token.approval_key);
    println!("  만료 시간:    {}", ws_token.approval_key_expired);

    println!("\n인증 완료!");
    Ok(())
}
