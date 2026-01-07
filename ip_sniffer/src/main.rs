use std::env;
use std::io::{self, Write};
use std::net::IpAddr;
use std::net::TcpStream;
use std::str::FromStr;
use std::sync::mpsc::{Sender, channel};
use std::thread;

#[derive(Debug)]
enum Argument {
    Help,
    ProgramName(String),
    ThreadCount(u16),
    IpAddress(IpAddr),
    StartPort(u16),
    EndPort(u16),
    Unknown(String),
}

enum PortStatus {
    Open(u16),
    Closed(u16),
}

const MAX_PORT: u16 = 65535;

fn parse_arguments(args: &[String]) -> Result<Vec<Argument>, &'static str> {
    let mut skip_next = false;
    let mut parsed_args = Vec::new();

    for (i, arg) in args.iter().enumerate() {
        if i == 0 {
            parsed_args.push(Argument::ProgramName(arg.clone()));
            continue;
        }

        if skip_next {
            skip_next = false;
            continue;
        }

        if arg.contains("-h") || arg.contains("--help") {
            parsed_args.push(Argument::Help);
            continue;
        }

        if arg.contains("-t") || arg.contains("--thread-count") {
            skip_next = true;
            let next_arg = match args.get(i + 1) {
                Some(next_arg) => next_arg,
                None => return Err("no thread count provided"),
            };
            parsed_args.push(Argument::ThreadCount(next_arg.parse::<u16>().unwrap()));
            continue;
        }

        if arg.contains("-i") || arg.contains("--ip-address") {
            skip_next = true;
            let next_arg = match args.get(i + 1) {
                Some(next_arg) => next_arg,
                None => return Err("no IP address provided"),
            };
            parsed_args.push(Argument::IpAddress(IpAddr::from_str(next_arg).unwrap()));
            continue;
        }

        if arg.contains("-s") || arg.contains("--start-port") {
            skip_next = true;
            let next_arg = match args.get(i + 1) {
                Some(next_arg) => next_arg,
                None => return Err("no start port provided"),
            };
            parsed_args.push(Argument::StartPort(next_arg.parse::<u16>().unwrap()));
            continue;
        }

        if arg.contains("-e") || arg.contains("--end-port") {
            skip_next = true;
            let next_arg = match args.get(i + 1) {
                Some(next_arg) => next_arg,
                None => return Err("no end port provided"),
            };
            parsed_args.push(Argument::EndPort(next_arg.parse::<u16>().unwrap()));
            continue;
        }
        parsed_args.push(Argument::Unknown(arg.clone()));
    }

    Ok(parsed_args)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program_name = args[0].clone();

    let parsed_args = match parse_arguments(&args) {
        Ok(parsed_args) => parsed_args,
        Err(e) => {
            println!("Error: {e}");
            return;
        }
    };
    println!("parsed_args: {parsed_args:?}");

    let mut threads_to_use = 4;
    let mut ip_to_sniff = IpAddr::from_str("127.0.0.1").unwrap();
    let mut starting_port = 1;
    let mut ending_port = MAX_PORT;

    for arg in parsed_args {
        match arg {
            Argument::Help => {
                print_help(&program_name);
                return;
            }
            Argument::ProgramName(program_name) => println!("program name: {program_name}"),
            Argument::ThreadCount(thread_count) => threads_to_use = thread_count,
            Argument::IpAddress(ip_address) => ip_to_sniff = ip_address,
            Argument::StartPort(start_port) => starting_port = start_port,
            Argument::EndPort(end_port) => ending_port = end_port,
            Argument::Unknown(unknown) => {
                println!("Unknown argument: {unknown} \n");
                print_help(&program_name);
                return;
            }
        }
    }

    let (tx, rx) = channel();

    let num_of_threads = if (ending_port - starting_port) > threads_to_use {
        threads_to_use
    } else {
        ending_port - starting_port
    };

    println!("threads to use: {num_of_threads}");
    println!("ip to sniff: {ip_to_sniff}");

    for i in 0..num_of_threads {
        let tx = tx.clone();
        thread::spawn(move || {
            scan(
                tx,
                starting_port + i,
                ending_port,
                ip_to_sniff,
                num_of_threads,
            )
        });
    }

    let mut open_ports = Vec::new();
    let mut closed_ports = Vec::new();
    drop(tx);

    for port in rx {
        match port {
            PortStatus::Open(port) => open_ports.push(port),
            PortStatus::Closed(port) => closed_ports.push(port),
        }
    }

    open_ports.sort();
    closed_ports.sort();

    println!("open ports: {open_ports:?}");
    println!("closed ports: {closed_ports:?}");
}

fn print_help(program_name: &str) {
    println!("Usage: {} <ip_address> <thread_count>", program_name);
    println!("Options:");
    println!("  -h, --help: Show this help message");
    println!("  -t, --thread-count: Set the number of threads to use");
    println!("  -i, --ip-address: Set the IP address to sniff");
    println!("Example: {} -i 127.0.0.1 -t 4", program_name);
}

fn scan(
    tx: Sender<PortStatus>,
    starting_port: u16,
    ending_port: u16,
    ip_to_sniff: IpAddr,
    threads_to_use: u16,
) {
    let mut port = starting_port;
    loop {
        let addr = format!("{}:{}", ip_to_sniff, port);

        println!("scanning {addr}:{port}");

        match TcpStream::connect(addr) {
            Ok(_) => {
                println!("{port} open");
                io::stdout().flush().unwrap();
                tx.send(PortStatus::Open(port)).unwrap();
            }
            Err(_) => {
                println!("{port} closed");
                tx.send(PortStatus::Closed(port)).unwrap();
            }
        };

        if (ending_port - port) < threads_to_use {
            break;
        }

        port += threads_to_use;
    }
}
