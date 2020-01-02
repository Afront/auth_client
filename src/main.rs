use std::io;
use std::io::Write;
use std::process;

fn abort() {
	println!("AHHHHHHHHH");
	process::exit(0);
}

fn help() {
	println!("You need help? I don't think I'm the right person to ask. Try calling someone on your phone.");
	abort();
}

fn signin() {
	println!("You want to sign in? Well, not today.");
	abort();
}

fn signup() {
	let mut id = String::new();
	let mut email = String::new();
	let mut password = String::new();

	print!("Please enter your ID: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut id)
			.expect("Failed to read line");

	print!("Please enter your email: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut email)
			.expect("Failed to read line");

	print!("Please enter your password: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut password)
			.expect("Failed to read line");

	println!("You want to sign up? Well, not today.");

	abort();
}

fn main() {
	loop {
		print!("Hello! Would you like to (R)egister or (S)ign in? ");
		io::stdout().flush().unwrap();
		let mut input = String::new();
		io::stdin().read_line(&mut input)
				.expect("Failed to read line");
		match input.trim().to_uppercase().as_str() {
				"ABORT" | "EXIT" | "Q" | "QUIT" => abort(),
				"HELP" | "H" => help(),
				"SIGN UP" | "SIGNUP" | "REGISTER" | "R" => signup(),
				"SIGN IN" | "SIGNIN" | "LOGIN" | "LOG IN" | "S" => signin(),
				_  => continue,
		};		
	}

}
