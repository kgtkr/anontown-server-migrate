use crate::models::history::HistoryAPI;
use crate::ports::{auth::AuthPort, history_loader::HistoryLoaderPort};

/// 指定されたidのhistoryを取得する
///
/// # 事前条件
/// * 指定されたidを持つhistoryが存在する
///
/// # 引数
/// * `id` - 取得するhistoryのid
///
/// # 返り値
/// * `HistoryAPI`
///
/// # エラー
/// なし
pub async fn get_history(
    id: &str,
    history_loader: &mut impl HistoryLoaderPort,
    auth_container: &impl AuthPort,
) -> Result<HistoryAPI, Box<dyn std::error::Error>> {
    let history = history_loader.load(id).await?;
    Ok(history.to_api(auth_container.get_token_or_none()))
} 