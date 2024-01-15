use crate::impl_error_wrapper;
use thiserror::Error;

impl_error_wrapper!(SmtpAddressError, lettre::address::AddressError);
impl_error_wrapper!(SmtpTransportError, lettre::transport::smtp::Error);
impl_error_wrapper!(LettreError, lettre::error::Error);
#[derive(Error, Debug)]
pub enum Error {
    #[error("ServerAddressParse({0})")]
    ServerAddressParse(SmtpAddressError),
    #[error("SmtpTransportRelayBuild({0})")]
    SmtpTransportRelayBuild(SmtpTransportError),
    #[error("MessageBuilder({0})")]
    MessageBuilder(LettreError),
    #[error("MessageSend({0})")]
    MessageSend(SmtpTransportError),
    #[error("RecipientAddressParse({0})")]
    RecipientAddressParse(SmtpAddressError),
}
