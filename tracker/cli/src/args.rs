use std::env;

/// Wrapper for arguments passed to the program.
pub struct Args {
    filename: String,
}

impl Args {
    /// Convert from std::env::Args to Args.
    /// Ensuring that a filename is passed as the first argument.
    pub fn new(mut args: env::Args) -> Result<Self, &'static str> {
        args.next(); // ignore 1st arg (program name)

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err("Didn't provide a filename"),
        };

        Ok(Self { filename })
    }

    /// Get access to the filename as a immutable reference.
    pub fn filename(&self) -> &str {
        &self.filename
    }
}
