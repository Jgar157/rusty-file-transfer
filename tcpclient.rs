use std::cmp::min;
use std::env;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Result, Write};
use std::net::{SocketAddrV4, Ipv4Addr, TcpStream};
use std::path::Path;

const CHUNK_SPLITS: usize = 1000;

fn main() -> Result<()> {

    // Get the port from arguments
    let args: Vec<String> = env::args().collect();

    let port: u16 = args[1].parse().unwrap();

    // If the improper port is used, exit
    if port != 1337 {
        println!("Improper port #, please use 1337.");

    } else {
        let local_host = Ipv4Addr::new(127, 0, 0, 1);

        // Address
        let address = SocketAddrV4::new(local_host, port);

        // TCP connection //
        let mut inc_stream = TcpStream::connect(address)?;
        let read_stream = inc_stream.try_clone()?;
        let mut reader = BufReader::new(read_stream);

        println!("Connected to server!");

        // Begin looping and wait for user input to see whether to upload or download
        loop {
            // Get user input
            let mut input = String::new();
            println!("Options: \n\
                \t1) upload\n\
                \t2) exit\n\
                \t3) get\n");

            std::io::stdin().read_line(&mut input).unwrap();
            input = input.trim().to_string();

            // Split the input into command and file
            let split = input.split(" ");
            let vec: Vec<&str> = split.collect();

            // Need to convert inputs to string from &str
            let mut command: String = vec[0].to_string();


            // Look for user input to exit
            if "exit" == command {
                inc_stream.write(command.as_bytes())?;
                inc_stream.flush().unwrap();
                println!("Ending Connection to Server");
                break;
            }

            // Upload route
            if "upload" == command {

                // Do not continue if the file name does not exist in the directory
                let file_name: String = vec[1].to_string();

                if Path::new(&file_name).exists() {

                    // Add a delimiter for easy reading
                    // (the file upload is mixing with telling the server what we are trying to do)
                    command = command.to_owned() + "\n";

                    inc_stream.write(command.as_bytes())?;
                    inc_stream.flush().unwrap();

                    // Send file name over
                    inc_stream.write(file_name.as_bytes())?;
                    inc_stream.flush().unwrap();

                    // Send \n as delimiter
                    inc_stream.write(b"\n")?;
                    inc_stream.flush().unwrap();

                    // Verify that file name exists in the directory


                    // Open up the file we want to send
                    let mut file = File::open(file_name)?;

                    // Get length of upload file so send before writing the file
                    let file_size = file.metadata().unwrap().len();
                    println!("{}", file_size);
                    let file_size_bytes = (file_size as u64).to_ne_bytes();
                    inc_stream.write_all(&file_size_bytes).unwrap();
                    inc_stream.flush().unwrap();

                    // Make a buffer to hold the split data
                    let mut chunk = [0; CHUNK_SPLITS];

                    while let Ok(bytes) = file.read(&mut chunk) {
                        // If there are no more bytes to read then exit
                        if bytes == 0 {
                            break;
                        }
                        // Write to stream
                        // On final write, exclude the elements not read using
                        // Example: If there is one element left to write,
                        // we do not want the remaining 999 elements to be written.
                        inc_stream.write(&chunk[..bytes]).unwrap();
                    }

                    inc_stream.flush().unwrap();
                } else {
                    println!("File name {} does not exist in the directory. Please try again. \n", file_name)
                }
            }

            // Get - Same as upload on server.rs
            else if "get" == command {
                let file_name: String = vec[1].to_string();

                // Add a delimiter for easy reading
                // (the file upload is mixing with telling the server what we are trying to do)
                command = command.to_owned() + "\n";

                inc_stream.write(command.as_bytes())?;
                inc_stream.flush().unwrap();

                // Send file name over
                inc_stream.write(file_name.as_bytes())?;
                inc_stream.flush().unwrap();

                // Send \n as delimiter
                inc_stream.write(b"\n")?;
                inc_stream.flush().unwrap();

                // Check if file exists on the server side before continuing
                // Server will send a flag bit: 0 if the file does not exist, 1 if it does
                let mut file_exists = [0];
                reader.read_exact(&mut file_exists)?;

                if file_exists[0] == 1 {

                    // We already know file name since it is input by the user
                    let new_file_name = "new".to_owned() + &file_name;

                    // Create a buffer to receive the file size
                    let mut file_size = [0; 8];
                    inc_stream.read_exact(&mut file_size).unwrap();
                    let file_size = u64::from_ne_bytes(file_size);

                    println!("File Size: {}", file_size);

                    // Read and output the client input
                    let file = File::create(new_file_name)?;
                    let mut writer = BufWriter::new(file);

                    // io::copy(&mut reader, &mut writer)?;

                    let mut bytes_remaining = file_size;
                    let mut chunk = [0; CHUNK_SPLITS];

                    while bytes_remaining > 0 {

                        // If there are no more bytes to read then exit
                        if bytes_remaining == 0 {
                            break;
                        }

                        // Get the amount of bytes that are left to be read
                        // If there are enough bytes to fill chunk, fill it
                        // Otherwise only fill with however many are left
                        let bytes_in = min(bytes_remaining, chunk.len() as u64) as usize;

                        // Read exactly bytes_in from stream
                        reader.read_exact(&mut chunk[..bytes_in]).unwrap();
                        // Write to stream
                        writer.write_all(&chunk[..bytes_in]).unwrap();
                        writer.flush().unwrap();

                        // Reduce bytes remaining by bytes_in
                        bytes_remaining -= bytes_in as u64;
                    }
                } else {
                    println!("File does not exist! Please Try again. \n")
                }
            }

            // Error route
            else {
                println!("Improper command used");
            }
        }

    }
    Ok(())
}
