use tokio::{self, io::AsyncWriteExt};
pub struct OutFile {
    file_name: String,
    file_buf_out: tokio::io::BufWriter<tokio::fs::File>,
}

impl OutFile {
    pub async fn new(file_name: &str) -> OutFile {
        let f = tokio::fs::File::create(file_name)
            .await
            .expect(&format!("Unable to create file {}", file_name));
        let file_buf_out = tokio::io::BufWriter::new(f);
        OutFile {
            file_name: file_name.to_string(),
            file_buf_out: file_buf_out,
        }
    }
    pub async fn write(&mut self, text: String) {
        let _ = self
            .file_buf_out
            .write_all(format!("{}\n", text).as_bytes())
            .await
            .expect("Unable to write data");
        self.file_buf_out
            .flush()
            .await
            .expect("Unable to flush to disk");
    }
}