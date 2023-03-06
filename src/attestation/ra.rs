use std::{
	fs::{File, OpenOptions},
	io::{Read, Write},
};

use tracing::info;

pub fn _generate_quote() -> Vec<u8> {
	if !std::path::Path::new("/dev/attestation/user_report_data").exists() {
		info!("This is NOT inside an Enclave!");
		return "This is NOT inside an Enclave!".as_bytes().to_vec()
	}

	let mut f1 = OpenOptions::new()
		.write(true)
		.open("/dev/attestation/user_report_data")
		.unwrap(); // TODO: manage unwrap()
	info!("This is inside Enclave!");

	let mut f2 = File::open("/dev/attestation/attestation_type").unwrap(); // TODO: manage unwrap()
	let mut attest_type = String::new();
	f2.read_to_string(&mut attest_type).unwrap(); // TODO: manage unwrap()
	info!("attestation type is : {}", attest_type);

	let write_zero = [0u8; 64];
	f1.write_all(&write_zero)
		.expect("Error writing to /dev/attestation/user_report_data"); // TODO: manage expect()

	info!("Reading The Quote ...");
	let mut f3 = File::open("/dev/attestation/quote").unwrap(); // TODO: manage unwrap()
	let mut contents = vec![];
	f3.read_to_end(&mut contents).unwrap(); // TODO: manage unwrap()
										//println!("{:-#?}",contents);

	info!("Dumping the Quote");
	let mut f4 = File::create("/quote/enclave.quote").unwrap(); // TODO: manage unwrap()
	f4.write_all(&contents).unwrap(); // TODO: manage unwrap()

	contents
}
