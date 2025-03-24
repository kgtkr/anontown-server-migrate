use crate::models::client::ClientAPI;
use crate::ports::{auth::AuthPort, client_loader::ClientLoaderPort};

/// 指定されたidのclientを取得する
///
/// # 事前条件
/// * 指定されたidを持つclientが存在する
///
/// # 引数
/// * `id` - 取得するclientのid
///
/// # 返り値
/// * `ClientAPI`
///
/// # エラー
/// なし
pub async fn get_client(
    id: &str,
    client_loader: &mut impl ClientLoaderPort,
    auth_container: &impl AuthPort,
) -> Result<ClientAPI, Box<dyn std::error::Error>> {
    let client = client_loader.load(id).await?;
    Ok(client.to_api(auth_container.get_token_master_or_none()))
} 