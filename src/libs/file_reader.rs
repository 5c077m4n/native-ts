extern crate tokio;

use std::path::Path;
use tokio::fs;

#[allow(dead_code)]
pub async fn read_content(file_path: impl AsRef<Path>) -> String {
    fs::read_to_string(file_path)
        .await
        .expect("The requested file does not exist.")
}

#[cfg(test)]
mod tests {
    extern crate tokio;
    use super::*;
    use tokio::runtime::Runtime;

    #[test]
    #[should_panic(expected = "The requested file does not exist.")]
    fn sanity() {
        let _ = Runtime::new()
            .unwrap()
            .block_on(read_content(Path::new("./no/such/file.txt")));
    }
}
