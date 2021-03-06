use std::result::Result;

use opcua_types::*;

use opcua_core::comms::prelude::*;
use opcua_core::crypto::{X509, SecurityPolicy};

struct SecureChannelState {
    // Issued flag
    pub issued: bool,
    // Renew count, debugging
    pub renew_count: usize,
    // Last secure channel id
    last_secure_channel_id: UInt32,
    /// Last token id number
    last_token_id: UInt32,
}

impl SecureChannelState {
    pub fn new() -> SecureChannelState {
        SecureChannelState {
            last_secure_channel_id: 0,
            issued: false,
            renew_count: 0,
            last_token_id: 0,
        }
    }

    pub fn create_secure_channel_id(&mut self) -> UInt32 {
        self.last_secure_channel_id += 1;
        self.last_secure_channel_id
    }

    pub fn create_token_id(&mut self) -> UInt32 {
        self.last_token_id += 1;
        self.last_token_id
    }
}

pub struct SecureChannelService {
    // Secure channel info for the session
    secure_channel_state: SecureChannelState,
}

impl SecureChannelService {
    pub fn new() -> SecureChannelService {
        SecureChannelService {
            secure_channel_state: SecureChannelState::new(),
        }
    }

    pub fn open_secure_channel(&mut self, secure_channel: &mut SecureChannel, security_header: &SecurityHeader, client_protocol_version: UInt32, message: &SupportedMessage) -> Result<SupportedMessage, StatusCode> {
        let request = match *message {
            SupportedMessage::OpenSecureChannelRequest(ref request) => {
                trace!("Got secure channel request {:?}", request);
                request
            }
            _ => {
                error!("message is not an open secure channel request, got {:?}", message);
                return Err(BAD_UNEXPECTED_ERROR);
            }
        };

        let security_header = match *security_header {
            SecurityHeader::Asymmetric(ref security_header) => {
                security_header
            }
            _ => {
                error!("Secure channel request message does not have asymmetric security header");
                return Err(BAD_UNEXPECTED_ERROR);
            }
        };

        // Must compare protocol version to the one from HELLO
        if request.client_protocol_version != client_protocol_version {
            error!("Client sent a different protocol version than it did in the HELLO - {} vs {}", request.client_protocol_version, client_protocol_version);
            return Ok(ServiceFault::new_supported_message(&request.request_header, BAD_PROTOCOL_VERSION_UNSUPPORTED));
        }

        // Test the request type
        match request.request_type {
            SecurityTokenRequestType::Issue => {
                trace!("Request type == Issue");
                if self.secure_channel_state.renew_count > 0 {
                    // TODO check to see if renew has been called before or not
                    // error
                    error!("Asked to issue token on session that has called renew before");
                }
            }
            SecurityTokenRequestType::Renew => {
                trace!("Request type == Renew");

                // Check for a duplicate nonce. It is invalid for the renew to use the same nonce
                // as was used for last issue/renew
                if request.client_nonce.as_ref() == &secure_channel.client_nonce[..] {
                    return Ok(ServiceFault::new_supported_message(&request.request_header, BAD_NONCE_INVALID));
                }

                if !self.secure_channel_state.issued {
                    // TODO check to see if the secure channel has been issued before or not
                    error!("Asked to renew token on session that has never issued token");
                    return Err(BAD_UNEXPECTED_ERROR);
                }
                self.secure_channel_state.renew_count += 1;
            }
        }

        // Check the requested security mode
        debug!("Message security mode == {:?}", request.security_mode);
        match request.security_mode {
            MessageSecurityMode::None | MessageSecurityMode::Sign | MessageSecurityMode::SignAndEncrypt => {
                // TODO validate NONCE
            }
            _ => {
                error!("Security mode is invalid");
                return Ok(ServiceFault::new_supported_message(&request.request_header, BAD_SECURITY_MODE_REJECTED));
            }
        }

        // Process the request
        self.secure_channel_state.issued = true;

        secure_channel.security_mode = request.security_mode;

        // Create a new secure channel info
        secure_channel.token_id = self.secure_channel_state.create_token_id();
        secure_channel.secure_channel_id = self.secure_channel_state.create_secure_channel_id();

        if !security_header.sender_certificate.is_null() {
            secure_channel.their_cert = Some(X509::from_byte_string(&security_header.sender_certificate)?);
        }

        let nonce_result = secure_channel.set_remote_nonce(&request.client_nonce);
        if nonce_result.is_ok() {
            secure_channel.create_random_nonce();
        } else {
            error!("Was unable to set their nonce, check logic");
            return Ok(ServiceFault::new_supported_message(&request.request_header, nonce_result.unwrap_err()));
        }

        if secure_channel.security_policy != SecurityPolicy::None && (secure_channel.security_mode == MessageSecurityMode::Sign || secure_channel.security_mode == MessageSecurityMode::SignAndEncrypt) {
            secure_channel.derive_keys();
        }

        let response = OpenSecureChannelResponse {
            response_header: ResponseHeader::new_good(&request.request_header),
            server_protocol_version: 0,
            security_token: ChannelSecurityToken {
                channel_id: secure_channel.secure_channel_id,
                token_id: secure_channel.token_id,
                created_at: DateTime::now(),
                revised_lifetime: request.requested_lifetime,
            },
            server_nonce: ByteString::from(&secure_channel.server_nonce),
        };
        Ok(SupportedMessage::OpenSecureChannelResponse(response))
    }

    pub fn close_secure_channel(&mut self, _: &SupportedMessage) -> Result<SupportedMessage, StatusCode> {
        info!("CloseSecureChannelRequest received, session closing");
        Err(BAD_CONNECTION_CLOSED)
    }
}