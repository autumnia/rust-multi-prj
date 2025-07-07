use crate::CHUNK_SIZE;
use crossbeam::channel::Sender;
use std::fs::File;
use std::io::{self, BufReader, Read, Result};

pub fn read_loop(infile: &str, stat_tx: Sender<usize>, write_tx: Sender<Vec<u8>>) -> Result<()> {
    let mut reader: Box<dyn Read> = if !infile.is_empty() {
        let read_file = File::open(infile)?;
        let buffer_reader = BufReader::new(read_file);
        Box::new(buffer_reader)
    } else {
        let std_in = io::stdin();
        let buffer_reader = BufReader::new(std_in);
        Box::new(buffer_reader)
    };

    let mut buffer = [0; CHUNK_SIZE];
    loop {
        let num_read = match reader.read(&mut buffer) {
            Ok(0) => break,
            Ok(value) => value,
            Err(_) => break,
        };
        let _ = stat_tx.send(num_read);
        if write_tx.send(Vec::from(&buffer[..num_read])).is_err() {
            break;
        }
    }
    let _ = stat_tx.send(0);
    let _ = write_tx.send(Vec::new());

    Ok(())
}
