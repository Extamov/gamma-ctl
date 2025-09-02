mod utils;

use std::{env, process::exit};

use utils::{Monitor, StrResult, generate_gamma_ramp};

struct Args {
	index: u8,
	brightness: f64,
	contrast: f64,
	gamma: f64,
}

fn parse_args() -> Args {
	match inner_parse_args() {
		Ok(x) => x,
		Err(_) => {
			eprintln!(
				"Usage: gamma-ctl <monitor-index [1-9]> <brightness [0-100]> <contrast [0-100]> <gamma [0.3-2.8]>"
			);
			exit(1);
		}
	}
}

fn inner_parse_args() -> StrResult<Args> {
	let mut raw_args = env::args().filter(|arg| arg.len() > 0);

	if raw_args.next().is_none() {
		return Err("");
	}

	let index = raw_args.next().ok_or("")?.parse().or(Err(""))?;
	let brightness = raw_args.next().ok_or("")?.parse().or(Err(""))?;
	let contrast = raw_args.next().ok_or("")?.parse().or(Err(""))?;
	let gamma = raw_args.next().ok_or("")?.parse().or(Err(""))?;

	if raw_args.next().is_some() {
		return Err("");
	}

	if index < 1 || index > 9 {
		return Err("");
	}
	if brightness < 0.0 || brightness > 100.0 {
		return Err("");
	}
	if contrast < 0.0 || contrast > 100.0 {
		return Err("");
	}
	if gamma < 0.3 || gamma > 2.8 {
		return Err("");
	}

	Ok(Args {
		index,
		brightness,
		contrast,
		gamma,
	})
}

fn main() -> StrResult<()> {
	let args: Args = parse_args();

	let monitor = Monitor::new(args.index)?;
	let gamma_ramp = generate_gamma_ramp(args.brightness, args.contrast, args.gamma);
	monitor.set_gamma_ramp(&gamma_ramp)?;
	println!("Done!");
	Ok(())
}
