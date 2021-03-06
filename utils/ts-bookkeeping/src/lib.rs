use std::fmt;
use std::net::SocketAddr;

use derive_more::From;
use failure::{Fail, ResultExt};

pub mod data;
pub mod events;
pub mod messages;

// Reexports
pub use tsproto_types::errors::Error as TsError;
pub use tsproto_types::versions::Version;
pub use tsproto_types::{
	ChannelGroupId, ChannelId, ChannelType, ClientDbId, ClientId,
	ClientType, Codec, CodecEncryptionMode, GroupNamingMode, GroupType,
	HostBannerMode, HostMessageMode, IconHash, Invoker, InvokerRef,
	LicenseType, LogLevel, MaxClients, Permission, PermissionType, PluginTargetMode, Reason, ServerGroupId, TalkPowerRequest,
	TextMessageTargetMode, TokenType, Uid, UidRef,
};

type Result<T> = std::result::Result<T, Error>;

#[derive(Fail, Debug, From)]
pub enum Error {
	#[fail(display = "{}", _0)]
	Base64(#[cause] base64::DecodeError),
	#[fail(display = "{}", _0)]
	Ts(#[cause] TsError),
	#[fail(display = "{}", _0)]
	Utf8(#[cause] std::str::Utf8Error),
	#[fail(display = "{}", _0)]
	ParseInt(#[cause] std::num::ParseIntError),
	#[fail(display = "{}", _0)]
	ParseError(#[cause] messages::ParseError),
	#[fail(display = "{}", _0)]
	Other(#[cause] failure::Compat<failure::Error>),

	#[doc(hidden)]
	#[fail(display = "Not an error – non exhaustive enum")]
	__NonExhaustive,
}

impl From<failure::Error> for Error {
	fn from(e: failure::Error) -> Self {
		let r: std::result::Result<(), _> = Err(e);
		Error::Other(r.compat().unwrap_err())
	}
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ServerAddress {
	SocketAddr(SocketAddr),
	Other(String),
}

impl From<SocketAddr> for ServerAddress {
	fn from(addr: SocketAddr) -> Self { ServerAddress::SocketAddr(addr) }
}

impl From<String> for ServerAddress {
	fn from(addr: String) -> Self { ServerAddress::Other(addr) }
}

impl<'a> From<&'a str> for ServerAddress {
	fn from(addr: &'a str) -> Self { ServerAddress::Other(addr.to_string()) }
}

impl fmt::Display for ServerAddress {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			ServerAddress::SocketAddr(a) => fmt::Display::fmt(a, f),
			ServerAddress::Other(a) => fmt::Display::fmt(a, f),
		}
	}
}

/// All possible targets to send messages.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MessageTarget {
	Server,
	Channel,
	Client(ClientId),
	Poke(ClientId),
}

/// The configuration to create a new connection.
///
/// # Example
///
/// ```no_run
/// # extern crate tokio;
/// # extern crate tsclientlib;
/// #
/// # use tokio::prelude::Future;
/// # use tsclientlib::{Connection, ConnectOptions};
/// # fn main() {
/// #
/// let con_config = ConnectOptions::new("localhost");
///
/// tokio::run(
///     Connection::new(con_config)
///     .map(|connection| ())
///     .map_err(|_| ())
/// );
/// # }
/// ```
pub struct ConnectOptions {
	address: ServerAddress,
	local_address: Option<SocketAddr>,
	name: String,
	version: Version,
	log_commands: bool,
	log_packets: bool,
	log_udp_packets: bool,
}

impl ConnectOptions {
	/// Start creating the configuration of a new connection.
	///
	/// # Arguments
	/// The address of the server has to be supplied. The address can be a
	/// [`SocketAddr`], a string or directly a [`ServerAddress`]. A string
	/// will automatically be resolved from all formats supported by TeamSpeak.
	/// For details, see [`resolver::resolve`].
	///
	/// [`SocketAddr`]: ../../std/net/enum.SocketAddr.html
	/// [`ServerAddress`]: enum.ServerAddress.html
	/// [`resolver::resolve`]: resolver/method.resolve.html
	#[inline]
	pub fn new<A: Into<ServerAddress>>(address: A) -> Self {
		Self {
			address: address.into(),
			local_address: None,
			name: String::from("TeamSpeakUser"),
			version: Version::Linux_3_2_1,
			log_commands: false,
			log_packets: false,
			log_udp_packets: false,
		}
	}

	/// The address for the socket of our client
	///
	/// # Default
	/// The default is `0.0.0:0` when connecting to an IPv4 address and `[::]:0`
	/// when connecting to an IPv6 address.
	#[inline]
	pub fn local_address(mut self, local_address: SocketAddr) -> Self {
		self.local_address = Some(local_address);
		self
	}

	/// The name of the user.
	///
	/// # Default
	/// `TeamSpeakUser`
	#[inline]
	pub fn name(mut self, name: String) -> Self {
		self.name = name;
		self
	}

	/// The displayed version of the client.
	///
	/// # Default
	/// `3.2.1 on Linux`
	#[inline]
	pub fn version(mut self, version: Version) -> Self {
		self.version = version;
		self
	}

	/// If the content of all commands should be written to the logger.
	///
	/// # Default
	/// `false`
	#[inline]
	pub fn log_commands(mut self, log_commands: bool) -> Self {
		self.log_commands = log_commands;
		self
	}

	/// If the content of all packets in high-level form should be written to
	/// the logger.
	///
	/// # Default
	/// `false`
	#[inline]
	pub fn log_packets(mut self, log_packets: bool) -> Self {
		self.log_packets = log_packets;
		self
	}

	/// If the content of all udp packets in byte-array form should be written
	/// to the logger.
	///
	/// # Default
	/// `false`
	#[inline]
	pub fn log_udp_packets(mut self, log_udp_packets: bool) -> Self {
		self.log_udp_packets = log_udp_packets;
		self
	}
}

impl fmt::Debug for ConnectOptions {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		// Error if attributes are added
		let ConnectOptions {
			address,
			local_address,
			name,
			version,
			log_commands,
			log_packets,
			log_udp_packets,
		} = self;
		write!(
			f,
			"ConnectOptions {{ address: {:?}, local_address: {:?}, \
			 name: {}, version: {}, \
			 log_commands: {}, log_packets: {}, log_udp_packets: {} }}",
			address,
			local_address,
			name,
			version,
			log_commands,
			log_packets,
			log_udp_packets,
		)?;
		Ok(())
	}
}

pub struct DisconnectOptions {
	reason: Option<Reason>,
	message: Option<String>,
}

impl Default for DisconnectOptions {
	#[inline]
	fn default() -> Self {
		Self {
			reason: None,
			message: None,
		}
	}
}

impl DisconnectOptions {
	#[inline]
	pub fn new() -> Self { Self::default() }

	/// Set the reason for leaving.
	///
	/// # Default
	///
	/// None
	#[inline]
	pub fn reason(mut self, reason: Reason) -> Self {
		self.reason = Some(reason);
		self
	}

	/// Set the leave message.
	///
	/// You also have to set the reason, otherwise the message will not be
	/// displayed.
	///
	/// # Default
	///
	/// None
	#[inline]
	pub fn message<S: Into<String>>(mut self, message: S) -> Self {
		self.message = Some(message.into());
		self
	}
}
