mod encryption;
mod files;

use encryption::*;
use files::*;

use argparse::{ArgumentParser, Store, StoreTrue};
use std::io::Write;
use std::path::Path;

// TODO: Fix the compression: Divide the file into smaller of 1GB
#[tokio::main]
async fn main() {
    let mut folder_path = String::new();
    let mut output_path = String::new();
    let mut file_path = String::new();
    let mut encrypted = false;

    {
        let mut ap = ArgumentParser::new();
        ap.set_description("SecureFolder V0.1: Encrypt and package a given folder into one file.");

        ap.refer(&mut folder_path).add_option(
            &["--folderpath"],
            Store,
            "Path to the folder to be encrypted.",
        );

        ap.refer(&mut file_path).add_option(
            &["--combined-file"],
            Store,
            "Path to the combined file to be accessed; the contents will be placed in the current directory.",
        );

        ap.refer(&mut output_path)
            .add_option(
                &["-o", "--output"],
                Store,
                "Path to the output file; Beware if a file exists with the same name, that the file will be shredded.",
            );

        ap.refer(&mut encrypted).add_option(
            &["-e", "--encrypt"],
            StoreTrue,
            "Use this flag if you wanna encrypt the combined file. A keypair will be generated if you choose to encrypt the file.",
        );
        ap.parse_args_or_exit();
    }

    // If a file with the same name exists, shred it.
    if Path::new(&output_path).exists() {
        reading::shred_file(&output_path);
    }

    // If a combined file is to be created, do so
    // if not, then recreate the file structure from the combined file.
    if file_path.is_empty() {
        if encrypted {
            // Create a new keypair
            let mut keypair = Keys::new();

            // Get the path to which the keypair is to be written on the disk
            let mut keypair_path = String::new();

            print!("Path to save keypair: ");
            // To take input on the same line as the print
            std::io::stdout().flush().expect("Failed to flush stdio.");
            std::io::stdin()
                .read_line(&mut keypair_path)
                .expect("Failed to read from stdin");

            // Create a combined file from the folder with encryption
            reading::create_combined_file(&folder_path, &output_path, Some(&mut keypair));

            // write the keypair to disk
            keypair.save_keypair(format!("{}.sfkp", keypair_path.trim()));
        } else {
            // Create a combined file from the folder without encryption
            // TODO: Implement file streaming

            reading::create_combined_file(&folder_path, &output_path, None);
        }
    } else if encrypted {
        // Create a keypair from the provided keys
        let mut keys_path = String::new();

        print!("Path to keys: ");
        std::io::stdout().flush().expect("Failed to flush stdout.");
        std::io::stdin()
            .read_line(&mut keys_path)
            .expect("Failed to read from stdin");

        let keys = Keys::from(keys_path.trim());

        // Recreate the file structure that was combined
        let combined_data = reading::read_combined_file(file_path, Some(&keys));
        reading::recreate_files(combined_data, Some(&keys)).await;
    } else {
        // Recreate the file structure that was combined
        let combined_data = reading::read_combined_file(file_path, None);
        reading::recreate_files(combined_data, None).await;
    }
}
