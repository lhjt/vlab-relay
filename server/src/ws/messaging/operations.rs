#![allow(clippy::pedantic)]
use std::net::SocketAddr;

use crate::{relay::ws_extensions::InitFrame, ws::models::Peer};

/// Handle a registration message from a peer.
pub(crate) async fn handle_registration(
    _peer: &mut Peer,
    _address: SocketAddr,
    _message: InitFrame,
) {
    // TODO: add logic
    todo!()
}
