use std::time::{Duration, Instant};

use failure::format_err;
use futures::Future;
use structopt::clap::AppSettings;
use structopt::StructOpt;
use tokio::timer::Delay;

use tsclientlib::data::{self, Channel, Client};
use tsclientlib::{
	ChannelId, ConnectOptions, Connection, DisconnectOptions, Reason,
};

#[derive(StructOpt, Debug)]
#[structopt(raw(global_settings = "&[AppSettings::ColoredHelp, \
	AppSettings::VersionlessSubcommands]"))]
struct Args {
	#[structopt(
		short = "a",
		long = "address",
		default_value = "localhost",
		help = "The address of the server to connect to"
	)]
	address: String,
	#[structopt(
		short = "v",
		long = "verbose",
		help = "Print the content of all packets",
		parse(from_occurrences)
	)]
	verbose: u8,
	// 0. Print nothing
	// 1. Print command string
	// 2. Print packets
	// 3. Print udp packets
}

/// `channels` have to be ordered.
fn print_channels(
	clients: &[&Client],
	channels: &[&Channel],
	parent: ChannelId,
	depth: usize,
)
{
	let indention = "  ".repeat(depth);
	for channel in channels {
		if channel.parent == parent {
			println!("{}- {}", indention, channel.name);
			// Print all clients in this channel
			for client in clients {
				if client.channel == channel.id {
					println!("{}  {}", indention, client.name);
				}
			}

			print_channels(clients, channels, channel.id, depth + 1);
		}
	}
}

fn print_channel_tree(con: &data::Connection) {
	let mut channels: Vec<_> = con.server.channels.values().collect();
	let mut clients: Vec<_> = con.server.clients.values().collect();
	channels.sort_by_key(|ch| ch.order);
	clients.sort_by_key(|c| c.talk_power);
	println!("{}", con.server.name);
	print_channels(&clients, &channels, ChannelId(0), 0);
}

fn main() -> Result<(), failure::Error> {
	// Parse command line options
	let args = Args::from_args();

	tokio::run(
		futures::lazy(|| {
			let con_config = ConnectOptions::new(args.address)
				.log_commands(args.verbose >= 1)
				.log_packets(args.verbose >= 2)
				.log_udp_packets(args.verbose >= 3);

			// Optionally set the key of this client, otherwise a new key is generated.
			let con_config = con_config.private_key_str(
				"MG0DAgeAAgEgAiAIXJBlj1hQbaH0Eq0DuLlCmH8bl+veTAO2+\
				k9EQjEYSgIgNnImcmKo7ls5mExb6skfK2Tw+u54aeDr0OP1ITs\
				C/50CIA8M5nmDBnmDM/gZ//4AAAAAAAAAAAAAAAAAAAAZRzOI").unwrap();

			// Connect
			Connection::new(con_config)
		})
		.map(|con| {
			// Listen to events
			con.add_on_event(
				"listener".into(),
				Box::new(|con, ev| {
					println!("Got events: {:?}", ev);
					print_channel_tree(&*con);
				}),
			);

			tokio::spawn(
				con.lock()
					.to_mut()
					.get_server()
					.set_subscribed(true)
					.map_err(|e| println!("Failed to subscribe ({:?})", e)),
			);
			con
		})
		.and_then(|con| {
			// Wait some time
			Delay::new(Instant::now() + Duration::from_secs(1))
				.map(move |_| con)
				.map_err(|e| format_err!("Failed to wait ({:?})", e).into())
		})
		.and_then(|con| {
			// Print channel tree
			{
				let con = con.lock();
				print_channel_tree(&*con);

				// Change name
				let con_mut = con.to_mut();
				let name = &con.server.clients[&con.own_client].name;
				tokio::spawn(con_mut.set_name(&format!("{}1", name)).map_err(
					|e| {
						println!("Failed to set client name: {:?}", e);
					},
				));
				tokio::spawn(con_mut.set_input_muted(true).map_err(|e| {
					println!("Failed to set muted: {:?}", e);
				}));
			}

			Ok(con)
		})
		.and_then(|con| {
			// Wait some time
			Delay::new(Instant::now() + Duration::from_secs(3))
				.map(move |_| con)
				.map_err(|e| format_err!("Failed to wait ({:?})", e).into())
		})
		.and_then(|con| {
			// Disconnect
			con.disconnect(
				DisconnectOptions::new()
					.reason(Reason::Clientdisconnect)
					.message("Is this the real world?"),
			)
		})
		.map_err(|e| panic!("An error occurred {:?}", e)),
	);

	Ok(())
}
