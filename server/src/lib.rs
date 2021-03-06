//! The OPC UA Server module contains all server side functionality - address space, service implementations, server
//! side authentications, sessions etc.

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;

#[macro_use]
extern crate log;

extern crate time;
extern crate chrono;
extern crate timer;

extern crate opcua_types;
extern crate opcua_core;

type DateTimeUTC = chrono::DateTime<chrono::UTC>;

mod services;
mod comms;
mod session;

pub mod server;
pub mod server_state;
pub mod subscriptions;
pub mod config;
pub mod address_space;
pub mod util;
pub mod continuation_point;

pub mod prelude {
    pub use opcua_core::prelude::*;
    pub use config::*;
    pub use server::*;
    pub use address_space::types::*;
    pub use subscriptions::*;
    pub use subscriptions::subscription::*;
    pub use subscriptions::monitored_item::*;
    pub use util::*;
}

/// Constants that govern the internal workings of the server impl.
pub mod constants {
    use opcua_types::Double;

    /// The default hello timeout period in seconds
    pub const DEFAULT_HELLO_TIMEOUT_SECONDS: u32 = 120;
    /// Default OPC UA server port
    pub const DEFAULT_OPC_UA_SERVER_PORT: u16 = 4855;
    /// Default maximum number of subscriptions in a session
    pub const DEFAULT_MAX_SUBSCRIPTIONS: u32 = 100;
    /// Default, "well known address for TCP discovery server
    //pub const DEFAULT_OPC_UA_DISCOVERY_SERVER_PORT: u16 = 4840;

    /// Sequence numbers wrap when they exceed this value
    pub const SEQUENCE_NUMBER_WRAPAROUND: u32 = 4294966271;

    // Internally controlled values

    /// Minimum publishing interval for subscriptions
    pub const MIN_PUBLISHING_INTERVAL: Double = 0.05f64;
    /// Minimum sampling interval in seconds allowed by clients on subscriptions or monitored_items
    pub const MIN_SAMPLING_INTERVAL: Double = 0.05f64;
    /// Default data change queue size
    pub const DEFAULT_DATA_CHANGE_QUEUE_SIZE: usize = 1;
    /// Minimum data change queue allowed by clients on monitored items
    pub const MIN_DATA_CHANGE_QUEUE_SIZE: usize = 1;
    /// Maximum data change queue allowed by clients on monitored items
    pub const MAX_DATA_CHANGE_QUEUE_SIZE: usize = 10;
    /// The default size of preallocated vecs of monitored items per subscription
    pub const DEFAULT_MONITORED_ITEM_CAPACITY: usize = 100;
    /// Sampling interval in MS used internally to poll subscriptions. The more finegrained this is
    /// the more often subscriptions will be checked to see if their subscription interval has elapsed
    /// therefore the value should be < min sampling interval
    pub const SUBSCRIPTION_TIMER_RATE_MS: i64 = 10;
    /// Time in MS that a session will timeout after with inactivity
    pub const SESSION_TIMEOUT: f64 = 50000f64;
    /// Maximum size in bytes that a request message is allowed to be
    pub const MAX_REQUEST_MESSAGE_SIZE: u32 = 32768;
    /// Maxmimum keep alive count
    pub const MAX_KEEP_ALIVE_COUNT: u32 = 30;
    /// Maximum browse continuation points
    pub const MAX_BROWSE_CONTINUATION_POINTS: usize = 10;
    /// Maximum history continuation points
    pub const MAX_HISTORY_CONTINUATION_POINTS: usize = 0;
    /// Maximum query continuation points
    pub const MAX_QUERY_CONTINUATION_POINTS: usize = 0;
}

#[cfg(test)]
mod tests;
