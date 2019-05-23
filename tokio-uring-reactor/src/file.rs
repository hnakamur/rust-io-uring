use std::fs;
use std::os::unix::io::{RawFd, AsRawFd, IntoRawFd};

#[derive(Debug)]
pub struct File(fs::File);

impl From<fs::File> for File {
	fn from(f: fs::File) -> Self {
		File(f)
	}
}

impl AsRawFd for File {
	fn as_raw_fd(&self) -> RawFd {
		self.0.as_raw_fd()
	}
}

impl IntoRawFd for File {
	fn into_raw_fd(self) -> RawFd {
		self.0.into_raw_fd()
	}
}

impl crate::io::FileRead for File {}
impl crate::io::FileWrite for File {}
