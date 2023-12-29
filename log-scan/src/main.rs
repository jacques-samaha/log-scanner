use clap::{Arg, App};
use std::process::Command;
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
        .get_matches();

    // Retrieve the args
    // If namespace argument was not provided, defaults to default 
    let job_name = String::from(args.value_of("job_name").unwrap());
    let namespace =  String::from(args.value_of("namespace").unwrap_or("default"));
    let keyword =  String::from(args.value_of("keyphrase").unwrap_or("Error"));
    let number_lines: u16 =  String::from(args.value_of("NumberLines").unwrap_or("20")).parse().unwrap();
    
    println!("");
    println!("Searching logs for job {job_name} in namespace {namespace} containing {keyword}");
    println!("");

    let pod_list = get_pods(namespace.clone());

    let pod_name = search_for_job(pod_list, job_name);

    scan_pod_logs(pod_name, namespace, keyword, number_lines)
}

fn search_for_job(pod_list: Vec<String>, job_name: String) -> String {
    let mut found_pod = String::from("");
    let mut name = String::from("");
    for pod in pod_list {
        name = pod.to_string();
        if name.contains(&job_name) {
            found_pod = name;
            break;
        }
    }

    return found_pod;
}

fn get_pods(namespace: String) -> Vec<String> {

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

    let mut pod_name = "";
    
    for (i, line) in result.split("/").enumerate() {
        if i == 0{
            continue
        }
        pod_name = line.split("\\").next().unwrap_or("Invalid Pod!"); 
        pod_list.push(String::from(pod_name));
    }

    pod_list

}

fn scan_pod_logs(pod_name: String, namespace: String, keyword: String, number_lines: u16) {
    let namespace_arg = format!("--namespace={namespace}");

    let get_logs = Command::new("kubectl")
        .arg("logs")
        .arg(pod_name)
        .arg(namespace_arg)
        .output().unwrap();

    let result  = format!("{:?}", String::from_utf8(get_logs.stdout).unwrap());
    let mut detect_count = 0;
    let mut error_detected = false;
    let mut mid_error = false;

    for (_, line) in result.split("\\n").enumerate(){
        if !mid_error && line.contains(&keyword) {
            error_detected = true;
            mid_error = true;
            println!("{}", "Log Output Detected".red().bold())
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