pub mod pig {
	pub fn pig (input: String) -> String{
		let bowels = String::from("aeiouAEIOU");
		let ints = input.as_bytes();
		for bowel in bowels.as_bytes() {
			// defered bowel as if not its a poointer to the u8
			if *bowel == ints[0] {
				return format!("{}-h{}y", input, ints[0] as char);
			}
		}
		let mut out_string = String::from("");
		for letter in ints {
			out_string.push(*letter as char);
		}
		let end_string = format!("-{}ay", ints[0] as char);
		out_string.push_str(&end_string);
		return out_string;
	}
}