# File Duplication Finder

## Overview

The File Duplication Finder scans the specified directory and identifies any duplicate files present. Once the scan completes, it outputs the results in a file named `copies.txt`. Not only does it identify the duplicates, but it also clearly labels which file is the original. Users are provided with the option to either delete the identified duplicate files or move them to a dedicated directory named "doublons" after the scan.

The program provides real-time feedback on its progress using an intuitive spinner, ensuring that the user is always aware of the status of the ongoing scan.

## Features

- Scans specified directories for duplicate files.
- Outputs results in a readable format with clear labels for original and duplicate files.
- Real-time progress spinner with a counter for analyzed files.
- Options to:
  - Delete identified duplicate files after the scan.
  - Move duplicates to a "doublons" directory for easy identification and segregation.

## Dependencies

- [indicatif](https://crates.io/crates/indicatif) for progress feedback.
- Standard Rust libraries for filesystem and I/O operations.

## Usage

1. Build the Rust project.
2. Run the generated executable.
3. Specify the directory you want to scan when prompted.
4. Once the scan completes, check the `copies.txt` file for results.
5. Decide based on the prompt whether you want to:
   - Delete the identified duplicates.
   - Move the duplicates to the "doublons" directory.

## Contributing

Contributions to enhance the program are always welcome. Please feel free to submit pull requests or open issues to discuss potential improvements or report bugs.

## License

This project is open source. Please refer to the associated license file for rights and limitations.
