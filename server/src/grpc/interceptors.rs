use tonic::metadata::MetadataMap;

use crate::USER_MANAGER;

/// Gets the zid from the token from a `gRPC` request metadata map.
pub(crate) async fn get_zid(meta: &MetadataMap) -> Option<String> {
    let auth_data = meta.get("Authorization")?.to_str().ok()?;

    if !auth_data.starts_with("Bearer ") {
        return None;
    }

    let token = auth_data.replace("Bearer ", "");

    let manager = USER_MANAGER.get().unwrap();
    let user = manager.get_by_token(&token).await;

    match user {
        Some(u) => Some(u.zid),
        None => None,
    }
}
