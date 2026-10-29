//! Future engine-command transport boundary. Payload encoding or matching DLLs
//! alone is not multiplayer support. No native command ID has been assigned.
use crate::protocol::{DecodeError, Envelope};
use crate::session::PeerFingerprint;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TransportError {
    NotRegistered,
    FingerprintMismatch,
    Payload(DecodeError),
    QueueRefused,
}

pub trait EngineTransport {
    fn registered_command_id(&self) -> Option<u32>;
    fn submit(&mut self, payload: &[u8]) -> Result<(), TransportError>;
}
/// Native factory/serialization/sender authority has not been established yet.
/// Keeping this explicit prevents a local file or GUI call impersonating it.
pub struct Unregistered;
impl EngineTransport for Unregistered {
    fn registered_command_id(&self) -> Option<u32> {
        None
    }
    fn submit(&mut self, _: &[u8]) -> Result<(), TransportError> {
        Err(TransportError::NotRegistered)
    }
}

pub fn send(
    transport: &mut impl EngineTransport,
    local: &PeerFingerprint,
    peers: &[PeerFingerprint],
    envelope: &Envelope,
) -> Result<(), TransportError> {
    if transport.registered_command_id().is_none() {
        return Err(TransportError::NotRegistered);
    }
    if peers.is_empty() || peers.iter().any(|p| !local.compatible(p)) {
        return Err(TransportError::FingerprintMismatch);
    }
    transport.submit(&envelope.encode().map_err(TransportError::Payload)?)
}
