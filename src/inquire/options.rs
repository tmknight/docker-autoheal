use crate::{report::logging::print_version, ALLOWED_CONNECTION_TYPES};
use getopts::Options;

pub struct OptionsList {
    pub apprise_url: Option<String>,
    pub connection_type: Option<String>,
    pub start_delay: Option<String>,
    pub interval: Option<String>,
    pub webhook_key: Option<String>,
    pub key_path: Option<String>,
    pub log_all: bool,
    pub monitor_all: bool,
    pub tcp_host: Option<String>,
    pub tcp_port: Option<String>,
    pub stop_timeout: Option<String>,
    pub tcp_timeout: Option<String>,
    pub verbose: bool,
    pub webhook_url: Option<String>,
    pub post_action: Option<String>,
}

pub fn get_opts(args: Vec<String>) -> OptionsList {
    let program = args[0].clone();

    // Establish usable arguments
    let mut opts = Options::new();
    opts.optopt("a", "apprise-url", "The apprise url", "<APPRISE_URL>");
    opts.optopt(
        "c",
        "connection-type",
        "One of local, socket, http, or ssl",
        "<CONNECTION_TYPE>",
    );
    opts.optopt(
        "d",
        "start-delay",
        "Time in seconds to wait for first check",
        "<START_DELAY>",
    );
    opts.optflag("h", "help", "Print help");
    opts.optopt(
        "i",
        "interval",
        "Time in seconds to check health",
        "<INTERVAL>",
    );
    opts.optopt(
        "j",
        "webhook-key",
        "The webhook json key string",
        "<WEBHOOK_KEY>",
    );
    opts.optopt(
        "k",
        "key-path",
        "The absolute path to requisite ssl PEM files",
        "<KEY_PATH>",
    );
    opts.optflag(
        "l",
        "log-all",
        "Enable logging of unhealthy containers where restart is disabled (WARNING, this could be chatty)",
    );
    opts.optflag(
        "m",
        "monitor-all",
        "Enable monitoring off all containers that have a healthcheck",
    );
    opts.optopt(
        "n",
        "tcp-host",
        "The hostname or IP address of the Docker host (when -c http or ssl)",
        "<TCP_HOST>",
    );
    opts.optopt(
        "p",
        "tcp-port",
        "The tcp port number of the Docker host (when -c http or ssl)",
        "<TCP_PORT>",
    );
    opts.optopt(
        "s",
        "stop-timeout",
        "Time in seconds to wait for action to complete",
        "<STOP_TIMEOUT>",
    );
    opts.optopt(
        "t",
        "tcp-timeout",
        "Time in seconds to wait for connection to complete",
        "<TCP_TIMEOUT>",
    );
    opts.optflag("v", "verbose", "Enable external logging and reporting of historical data");
    opts.optopt("w", "webhook-url", "The webhook url", "<WEBHOOK_URL>");
    opts.optopt(
        "P",
        "post-action",
        "The absolute path to a script that should be executed after container restart",
        "<SCRIPT_PATH>",
    );
    opts.optflag("V", "version", "Print version information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            println!("{}", opts.usage(&program));
            std::process::exit(1);
        }
    };

    // Process matching arguments
    if matches.opt_present("V") {
        print_version();
        std::process::exit(0);
    } else if matches.opt_present("h") {
        println!("{}", opts.usage(&program));
        std::process::exit(0);
    }

    // Ensure acceptable connection type arguments
    match matches.opt_str("c").is_some() {
        true => {
            let opt_connection_type = matches.opt_str("c").unwrap();
            match ALLOWED_CONNECTION_TYPES.contains(&opt_connection_type.as_str()) {
                true => {}
                false => {
                    println!("Unexpected connection-type: {}", opt_connection_type);
                    println!("{}", opts.usage(&program));
                    std::process::exit(1);
                }
            }
        }
        false => {}
    };

    OptionsList {
        apprise_url: matches.opt_str("a"),
        connection_type: matches.opt_str("c"),
        start_delay: matches.opt_str("d"),
        interval: matches.opt_str("i"),
        webhook_key: matches.opt_str("j"),
        key_path: matches.opt_str("k"),
        log_all: matches.opt_present("l"),
        monitor_all: matches.opt_present("m"),
        tcp_host: matches.opt_str("n"),
        tcp_port: matches.opt_str("p"),
        stop_timeout: matches.opt_str("s"),
        tcp_timeout: matches.opt_str("t"),
        verbose: matches.opt_present("v"),
        webhook_url: matches.opt_str("w"),
        post_action: matches.opt_str("P"),
    }
}
