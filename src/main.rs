use midir::MidiInput::ports;
use midir::{Ignore, MidiInput};
use rodio;
use std::error::Error;
use std::io::{stdin, BufReader};
fn main() {
	println!("Hello, world!");
	let device = rodio::default_output_device().unwrap();
	let sink = rodio::Sink::new(&device);

	let file = std::fs::File::open("Processed/High_Tom08.wav").unwrap();
	sink.append(rodio::Decoder::new(BufReader::new(file)).unwrap());

	sink.sleep_until_end();
}

fn set_device() -> Result<(), Box<dyn Error>> {
	let mut input = String::new();
	let mut midi_in = MidiInput::new("nanoPAD2")?;
	midi_in.ignore(Ignore::None);
	// Get an input port (read from console if multiple are available)
	let in_ports = midi_in.ports();
	let in_port = match in_ports.len() {
		0 => return Err("no input port found".into()),
		1 => {
			println!(
				"Choosing the only available input port: {}",
				midi_in.port_name(&in_ports[0]).unwrap()
			);
			&in_ports[0]
		}
		_ => {
			println!("\nAvailable input ports:");
			let mut input = String::new();
			for (i, p) in in_ports.iter().enumerate() {
				if midi_in.port_name(p).unwrap() == "nanoPAD2" {
					input = i.to_string();
					break;
				}
			}
			// print!("Please select input port: ");
			// stdout().flush()?;
			// stdin().read_line(&mut input)?;
			in_ports
				.get(input.trim().parse::<usize>()?)
				.ok_or("invalid input port selected")?
		}
	};

	println!("\nOpening connection");
	let in_port_name = midi_in.port_name(in_port)?;

	// _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
	let _conn_in = midi_in.connect(
		in_port,
		"midir-read-input",
		move |stamp, message, _| {
			if message[0] == 144 && message[1] == 50 {
				// next();
			} else if message[0] == 144 && message[1] == 48 {
				// prev();
			}
		},
		(),
	)?;

	println!(
		"Connection open, reading input from '{}' (press enter to exit) ...",
		in_port_name
	);

	input.clear();
	stdin().read_line(&mut input)?; // wait for next enter key press

	println!("Closing connection");
	Ok(())
}
