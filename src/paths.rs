#[cfg(unix)]
use std::path::PathBuf;


#[cfg(unix)]
pub fn state() -> Option<PathBuf> {
	let h = std::env::var("HOME");
	if h.is_err() {
		return None
	}
	let home = h.unwrap();
	let mut path = PathBuf::new();
	path.push(home);
	path.push(".local");
	path.push("state");
	path.push("nvim");
	Some(path)
}
