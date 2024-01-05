use clap::{Arg, App};
use std::{process::Command, fs};
use colored::Colorize;


fn main() {
    let args = App::new("kubectl-error-fetcher")
        .version("1.0")
        .about("Fetches the error log from a pod")
        .arg(Arg::with_name("job_name")
            .help("The pod to search for")
            .takes_value(true)
            .required(true))
        .arg(Arg::with_name("namespace")
            .help("The namespace of the desired pod")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("keyphrase")
            .help("Keyword to scan the logs for")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("NumberLines")
            .help("Number of lines after message to print!")
            .takes_value(true)
            .required(false))
        .arg(Arg::with_name("DownloadLog")
        .help("Do you want to download the log as a .txt file")
        .takes_value(true)
        .required(false))
        .get_matches();

    // Retrieve the args
    // If namespace argument was not provided, defaults to default 
    let job_name = String::from(args.value_of("job_name").unwrap());
    let namespace =  String::from(args.value_of("namespace").unwrap_or("default"));
    let keyword =  String::from(args.value_of("keyphrase").unwrap_or("Error"));
    let number_lines: u16 =  String::from(args.value_of("NumberLines").unwrap_or("20")).parse().unwrap();
    let download_log = String::from(args.value_of("DownloadLog").unwrap_or("N"));
    
    println!("");
    println!("Searching logs for job {job_name} in namespace {namespace} containing {keyword}");
    println!("");

    let pod_list = get_pods(namespace.clone()).unwrap();

    let pod_name = search_for_job(pod_list, job_name).unwrap();

    scan_pod_logs(pod_name, namespace, keyword, number_lines, download_log)
}

fn search_for_job(pod_list: Vec<String>, job_name: String) -> Result<String, String>  {
    let mut found_pod = String::from("");
    let mut name: String;
    for pod in pod_list {
        name = pod.to_string();
        if name.contains(&job_name) {
            found_pod = name;
            break;
        }
    }

    return Ok(found_pod);
}

fn get_pods(namespace: String) -> Result<Vec<String>, String> {

    let namespace_arg = format!("--namespace={namespace}");
    let get_pods = Command::new("kubectl")
        .arg("get")
        .arg("pods")
        .arg("-o=name")
        .arg(namespace_arg)
        .output().unwrap();

    // get_pods.arg("get pods").output().unwrap();
    let result  = format!("{:?}", String::from_utf8(get_pods.stdout).unwrap());
    let mut pod_list: Vec<String> = Vec::new();

    let mut pod_name: &str;
    
    for (i, line) in result.split("/").enumerate() {
        if i == 0{
            continue
        }
        pod_name = line.split("\\").next().unwrap_or("Invalid Pod!"); 
        pod_list.push(String::from(pod_name));
    }

    Ok(pod_list)

}

fn scan_pod_logs(pod_name: String, namespace: String, keyword: String, number_lines: u16, download_log: String) {
    let namespace_arg = format!("--namespace={namespace}");

    let get_logs = Command::new("kubectl")
        .arg("logs")
        .arg(&pod_name)
        .arg(&namespace_arg)
        .output().unwrap();

    let get_status = Command::new("kubectl")
        .arg("get")
        .arg("pod")
        .arg(&pod_name)
        .arg(namespace_arg)
        .output().unwrap();

    let tmp = String::from_utf8(get_status.stdout).unwrap();
    let pod_metrics = tmp
            .split('"').collect::<Vec<_>>()[0]
            .split_whitespace().collect::<Vec<_>>();
    
    let len = pod_metrics.len();

    let pod_age = pod_metrics[len-1];
    let pod_status = pod_metrics[len-3];

    let result  = format!("{:?}", String::from_utf8(get_logs.stdout).unwrap());
    let mut detect_count = 0;
    let mut error_detected = false;
    let mut mid_error = false;

    if download_log != "N" {
        fs::write("output.txt", &result).expect("Unable to write file!");
    }

    let pod_str = format!("Identified Pod: {pod_name} | AGE: {pod_age} | Status: {pod_status}\n").magenta().bold();
    print!("{pod_str}");

    for (_, line) in result.split("\\n").enumerate(){
        if !mid_error && line.contains(&keyword) {
            error_detected = true;
            mid_error = true;
            println!("{}", "Log Output Detected in pod: ".red().bold())
        }

        if mid_error {
            println!("{line}");
            detect_count += 1;
        }

        if detect_count == number_lines {
            println!("");
            mid_error = false;
            detect_count = 0;
        }
        
    }

    if !error_detected {
        println!("{}", "No Log Records Matching Scan!".bright_cyan());
    }
}
