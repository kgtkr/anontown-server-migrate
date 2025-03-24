use crate::entities::profile::ProfileAPI;
use crate::ports::{auth::AuthPort, profile_loader::ProfileLoaderPort};

/// 指定されたidのprofileを取得する
///
/// # 事前条件
/// * 指定されたidを持つprofileが存在する
///
/// # 引数
/// * `id` - 取得するprofileのid
///
/// # 返り値
/// * `ProfileAPI`
///
/// # エラー
/// なし
pub async fn get_profile(
    id: &str,
    profile_loader: &mut impl ProfileLoaderPort,
    auth_container: &impl AuthPort,
) -> Result<ProfileAPI, Box<dyn std::error::Error>> {
    let profile = profile_loader.load(id).await?;
    Ok(profile.to_api(auth_container.get_token_or_none()))
} 