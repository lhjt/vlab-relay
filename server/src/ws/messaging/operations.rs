use tracing::{instrument, warn};

use crate::{relay::ws_extensions::InitFrame, ws::models::Peer, USER_MANAGER};

/// Handle a registration message from a peer.
#[instrument]
pub(crate) async fn handle_registration(peer: &mut Peer, message: InitFrame) {
    // get the zid and token of the peer
    let zid = message.zid;
    let token = message.token;

    // check if this is a valid combo of zid and token
    let user = USER_MANAGER.get().unwrap().get_by_zid(&zid).await;
    match user {
        Some(user) => {
            // check if the token matches
            if user.token == token {
                // if the token matches, register the peer
                peer.register(zid);
            } else {
                // if the token does not match, close the connection
                warn!("[ws] invalid token");
                peer.close_with_policy();
            }
        },
        None => {
            // the user does not exist, so we will reject
            warn!("[ws] user does not exist");
            peer.close_with_policy();
        },
    }
}
