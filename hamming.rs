use std::io;
use std::io::Write;
use std::convert::TryInto;

fn main() {
	//interact with user
	println!("\n 1. Encode message \n 2. Decode message \n"); 
	print!("Choose an option: ");
	io::stdout().flush().unwrap();
	let mut user_choice = String::new();
	io::stdin()
		.read_line(&mut user_choice)
		.expect("Couldnt read from stdio");
	let mut choice = 0;
	match user_choice.trim().parse::<u32>(){
		Ok(i) => {choice = i},
		Err(..) => println!("Incorrect input"),
	};
	let mut user_input = String::new();
	println!("Input message: ");
	user_input.clear();
	io::stdin()
		.read_line(&mut user_input)
		.expect("Couldnt read from stdio");
		
	if choice == 1 {
		print!("Encoded message: {:?}\n", encode(26, 5, user_input)); //read message encode it and print code
	}
	else if choice == 2 {
		let msg: Vec<u32> = user_input.split_whitespace()
			.map(|x| x.parse().expect("Not an integer!"))
			.collect();
		print!("Decoded message: {:?}\n", decode(26, 5, msg));	//read code decode it and print the message
	}
	//println!("Encode message: {}", encode());

	
}


fn encode(length_of_word: u32,  num_of_bits: u32, msg: String) -> Vec<u32> {
	let str_ = msg.as_str();
	let mut letter_matrix: Vec<Vec<u32>>;
	let mut res = vec![0; msg.chars().count()];
	for (index, char) in str_.chars().enumerate(){
		letter_matrix = make_matrix(char as u32, length_of_word, num_of_bits, false);
		for i in 1..num_of_bits + 1{ //calculate control bits for current letter
			let mut pos: u32 = 2;
			let mut sum = 0;
			pos = pos.pow(i - 1) - 1;
			for j in 0..length_of_word + num_of_bits{
				sum += letter_matrix[0][j as usize] * 
									letter_matrix[i as usize][j as usize];	
			}
			letter_matrix[0][pos as usize] = sum % 2;
		}
		for i in 0..length_of_word + num_of_bits{ //translate binary into decimal
			let pos: u32 = 2;
			res[index] += pos.pow(i) * letter_matrix[0][i as usize];
		}
			
	}

	res
}

fn decode(length_of_word: u32, num_of_bits: u32, msg: Vec<u32>) -> Vec<u32> {
	let mut res = vec![0; msg.len()];
	let mut contr_bits = vec![0; num_of_bits as usize];
	let pos: u32 = 2;
	let mut letter_matrix: Vec<Vec<u32>> = vec![vec![0; 1]];

	for i in 0..num_of_bits{ //positions of control bits
		contr_bits[i as usize] = pos.pow(i);
	}

	for (index, elem) in msg.iter().enumerate(){
		letter_matrix = make_matrix(*elem, length_of_word, num_of_bits, true);
		let mut error: u32 = 0;
		for i in 1..num_of_bits + 1{ //calculate error code for current letter
			let mut sum = 0;
			for j in 0..length_of_word + num_of_bits{
				sum += letter_matrix[0][j as usize] * 
									letter_matrix[i as usize][j as usize];	
			}
			sum %= 2;
			error += pos.pow(i - 1) * sum;
		}
		if error != 0{
			println!("Error detected, at {} bit!", error);
			letter_matrix[0][error as usize] = 
						if letter_matrix[0][error as usize] == 0 {1} else {0};
		}
		let mut tmp = 0;
		for i in 0..length_of_word + num_of_bits{ //translate binary into decimal
			let pos: u32 = 2;
			if contr_bits.contains(&(i + 1)){
				continue;
			}
			println!("TMP: {}, i: {}, letter: {}", tmp, i,
										letter_matrix[0][i as usize]);
			res[index] += pos.pow(tmp) * letter_matrix[0][i as usize];
			tmp += 1;
		}
	}
	for(i, row) in letter_matrix.iter().enumerate(){
		for(j, col) in row.iter().enumerate(){
			print!("{} ", col);
		
		}
		print!("\n");
	}
	res
}



//mode = decoding(true) or encoding(false)
fn make_matrix(letter: u32, length_of_word: u32, num_of_bits: u32, mode: bool) -> Vec<Vec<u32>> {
	let mut mat = vec![vec![0; (num_of_bits + length_of_word).try_into().unwrap()];
						 (num_of_bits + 1).try_into().unwrap()];
	
	let mut bits_pos = vec![0; num_of_bits as usize];
	let pos: u32 = 2;
	for i in 0..num_of_bits{ //positions of control bits
		bits_pos[i as usize] = pos.pow(i);
	}
	
	//build matrix of transformation
	for i in 1..num_of_bits + 1 {
		let positions = bits_pos[(i - 1) as usize];
		let mut length = positions - 1;
		let mut put = false;
		if length == 0 {
			length += 1;
			put = true;
		}
		for j in 0..length_of_word + num_of_bits{
			if put {mat[i as usize][j as usize] = 1;}
			length = length - 1;
			if length == 0{
				length = positions;
				put = !put;
			}
		}
	}
	//translate letter to binary and put it into matrix
	let mut pos = 0;
	let mut letter_code = letter;
	while letter_code > 0{
		if !mode {  //skip positions of the control bits for encoding
			if bits_pos.contains(&(pos + 1)){
				pos += 1;
				continue;
			}
		}
		mat[0][pos as usize] = letter_code % 2;
		pos = pos + 1;
		letter_code /= 2;
	}
	mat
}

