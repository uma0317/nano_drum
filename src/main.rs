use midir::{Ignore, MidiInput};
use rodio::decoder::DecoderError;
use rodio::{Decoder, Device, Sink};
use std::error::Error;
use std::io::{stdin, BufReader, Read, Seek};
use std::marker::Send;
fn main() {
	println!("Hello, world!");
	let device = rodio::default_output_device().unwrap();
	let sink = Sink::new(&device);
	let file = std::fs::File::open("Processed/High Tom08.wav").unwrap();
	let decoded = Decoder::new(file).unwrap();
	let out_put_device = OutputDevice::new(device, sink, decoded);

	run(out_put_device);
}

struct OutputDevice<R>
where
	R: Read + Seek,
{
	device: Device,
	sink: Sink,
	high_tom: Decoder<R>,
}

impl<R> OutputDevice<R>
where
	R: Read + Seek,
{
	pub fn new(device: Device, sink: Sink, high_tom: Decoder<R>) -> Self {
		OutputDevice {
			device,
			sink,
			high_tom,
		}
	}

	fn play_hige_tom(&self) {
		self.play(self.high_tom);
	}

	fn play(&self, sound: Decoder<R>) {
		self.sink.append(sound as &rodio::source::Source);
		self.sink.sleep_until_end();
	}
}

fn run<R>(out_put_device: OutputDevice<R>) -> Result<(), Box<dyn Error>>
where
	R: Read + Seek + Send,
{
	let mut input = String::new();
	let mut midi_in = MidiInput::new("nanoPAD2")?;
	midi_in.ignore(Ignore::None);
	// Get an input port (read from console if multiple are available)
	let in_ports = midi_in.ports();

	let mut input = String::new();
	for (i, p) in in_ports.iter().enumerate() {
		if midi_in.port_name(p).unwrap() == "nanoPAD2" {
			input = i.to_string();
			break;
		}
	}

	let in_port = in_ports
		.get(input.trim().parse::<usize>()?)
		.ok_or("invalid input port selected")?;

	println!("\nOpening connection");
	let in_port_name = midi_in.port_name(in_port)?;

	// _conn_in needs to be a named parameter, because it needs to be kept alive until the end of the scope
	let _conn_in = midi_in.connect(
		in_port,
		"midir-read-input",
		move |stamp, message, _| {
			if message[0] == 144 && message[1] == 50 {
				out_put_device.play_hige_tom();
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

fn play() {}
