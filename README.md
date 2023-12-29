# Log-Scan
Rust command line tool to scan your kubernetes pods logs for a given keyword and extract the relevant portions

Tales in one required and three optional parameters:
- Job Name (Required): The job you want to scan your pods for. Will match the first pod containing the provided name.
- Namespace: Namespace of pods to scan through; Defaults to default namespace
- Keyphrase: The phrase to scan for in the logs. Defaults to Error
- Num Lines: The number of lines following the one containing the error to print out. Defaults to 20

For further information run log-scan --help 
