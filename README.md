# Log-Scan
Rust command line tool to scan your kubernetes pods logs for a given keyword and extract the relevant portions. Requires installation of the most recent Rust compiler (https://www.rust-lang.org/tools/install)

To compile binary: 
- Clone repo
- Navigate to the cloned folder "log-scan"
- run `cargo build --release`
- Binary will be generated in `target/release/log-scan`

# Adding to Command Line
Assumes you have followed the above compilation steps or downloaded the binary directly 
- Create a folder somewhere to contain all your rust binaires
- Copy that folder path into your `$PATH$` directory
- Can now call from anywhere in command line with `log-scan job_name ARGS`

Tales in one required and three optional parameters:
- Job Name (Required): The job you want to scan your pods for. Will match the first pod containing the provided name.
- Namespace: Namespace of pods to scan through; Defaults to default namespace
- Keyphrase: The phrase to scan for in the logs. Defaults to Error
- Num Lines: The number of lines following the one containing the error to print out. Defaults to 20

For further information `run log-scan --help`

