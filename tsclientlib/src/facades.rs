#![allow(dead_code)]

use std::ops::Deref;

use futures::Future;
use ts_bookkeeping::*;
use ts_bookkeeping::data::*;

use crate::{Error, MessageTarget};

include!(concat!(env!("OUT_DIR"), "/b2mdecls.rs"));
include!(concat!(env!("OUT_DIR"), "/facades.rs"));

impl ServerMut<'_> {
	/// Create a new channel.
	///
	/// # Arguments
	/// All initial properties of the channel can be set through the
	/// [`ChannelOptions`] argument.
	///
	/// # Examples
	/// ```rust,no_run
	/// use tsclientlib::data::ChannelOptions;
	/// # use futures::Future;
	/// # let connection: tsclientlib::Connection = panic!();
	///
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Send a message
	/// tokio::spawn(con_mut.get_server().add_channel(ChannelOptions::new("My new channel"))
	///	    .map_err(|e| println!("Failed to create channel ({:?})", e)));
	/// ```
	///
	/// [`ChannelOptions`]: struct.ChannelOptions.html
	#[must_use = "futures do nothing unless polled"]
	pub fn add_channel(
		&self,
		options: ChannelOptions,
	) -> impl Future<Item = (), Error = Error>
	{
		self.connection.send_packet(self.inner.add_channel(options))
	}

	/// Send a text message in the server chat.
	///
	/// # Examples
	/// ```rust,no_run
	/// # use futures::Future;
	/// # let connection: tsclientlib::Connection = panic!();
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Send a message
	/// tokio::spawn(con_mut.get_server().send_textmessage("Hi")
	///	    .map_err(|e| println!("Failed to send text message ({:?})", e)));
	/// ```
	#[must_use = "futures do nothing unless polled"]
	pub fn send_textmessage(
		&self,
		message: &str,
	) -> impl Future<Item = (), Error = Error>
	{
		self.connection.send_packet(self.inner.send_textmessage(message))
	}

	/// Subscribe or unsubscribe from all channels.
	pub fn set_subscribed(
		&self,
		subscribed: bool,
	) -> impl Future<Item = (), Error = Error>
	{
		self.connection.send_packet(self.inner.set_subscribed(subscribed))
	}
}

impl ConnectionMut<'_> {
	/// A generic method to send a text message or poke a client.
	///
	/// # Examples
	/// ```rust,no_run
	/// # use futures::Future;
	/// # use tsclientlib::MessageTarget;
	/// # let connection: tsclientlib::Connection = panic!();
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Send a message
	/// tokio::spawn(con_mut.send_message(MessageTarget::Server, "Hi")
	///	    .map_err(|e| println!("Failed to send message ({:?})", e)));
	/// ```
	#[must_use = "futures do nothing unless polled"]
	pub fn send_message(
		&self,
		target: MessageTarget,
		message: &str,
	) -> impl Future<Item = (), Error = Error>
	{
		self.connection.send_packet(self.inner.send_message(target, message))
	}
}

impl ClientMut<'_> {
	// TODO
	/*/// Move this client to another channel.
	/// This function takes a password so it is possible to join protected
	/// channels.
	///
	/// # Examples
	/// ```rust,no_run
	/// # use futures::Future;
	/// # let connection: tsclientlib::Connection = panic!();
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Get our own client in mutable form
	/// let client = con_mut.get_server().get_client(&con_lock.own_client).unwrap();
	/// // Switch to channel 2
	/// tokio::spawn(client.set_channel_with_password(ChannelId(2), "secure password")
	///	    .map_err(|e| println!("Failed to switch channel ({:?})", e)));
	/// ```*/

	/// Send a text message to this client.
	///
	/// # Examples
	/// Greet a user:
	/// ```rust,no_run
	/// # use futures::Future;
	/// # let connection: tsclientlib::Connection = panic!();
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Get our own client in mutable form
	/// let client = con_mut.get_server().get_client(&con_lock.own_client).unwrap();
	/// // Send a message
	/// tokio::spawn(client.send_textmessage("Hi me!")
	///	    .map_err(|e| println!("Failed to send me a text message ({:?})", e)));
	/// ```
	#[must_use = "futures do nothing unless polled"]
	pub fn send_textmessage(
		&self,
		message: &str,
	) -> impl Future<Item = (), Error = Error>
	{
		self.connection.send_packet(self.inner.send_textmessage(message))
	}

	/// Poke this client with a message.
	///
	/// # Examples
	/// ```rust,no_run
	/// # use futures::Future;
	/// # let connection: tsclientlib::Connection = panic!();
	/// let con_lock = connection.lock();
	/// let con_mut = con_lock.to_mut();
	/// // Get our own client in mutable form
	/// let client = con_mut.get_server().get_client(&con_lock.own_client).unwrap();
	/// // Send a message
	/// tokio::spawn(client.poke("Hihihi")
	///	    .map_err(|e| println!("Failed to poke me ({:?})", e)));
	/// ```
	#[must_use = "futures do nothing unless polled"]
	pub fn poke(&self, message: &str) -> impl Future<Item = (), Error = Error> {
		self.connection.send_packet(self.inner.poke(message))
	}
}
