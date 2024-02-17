use crate::{report::logging::print_version, ALLOWED_CONNECTION_TYPES};
use getopts::Options;

pub struct OptionsList {
    pub connection_type: Option<String>,
    pub container_label: Option<String>,
    pub stop_timeout: Option<String>,
    pub interval: Option<String>,
    pub start_delay: Option<String>,
    pub tcp_host: Option<String>,
    pub tcp_port: Option<String>,
    pub tcp_timeout: Option<String>,
    pub key_path: Option<String>,
    pub apprise_url: Option<String>,
    pub webhook_key: Option<String>,
    pub webhook_url: Option<String>,
    pub post_action: Option<String>,
}

pub fn get_opts(args: Vec<String>) -> OptionsList {
    let program = args[0].clone();

    // Establish usable arguments
    let mut opts = Options::new();
    opts.optopt(
        "c",
        "connection-type",
        "One of local, socket, http, or ssl",
        "<CONNECTION_TYPE>",
    );
    opts.optopt(
        "l",
        "container-label",
        "Container label to monitor (e.g. autoheal)",
        "<CONTAINER_LABEL>",
    );
    opts.optopt(
        "s",
        "stop-timeout",
        "Time in seconds to wait for action to complete",
        "<STOP_TIMEOUT>",
    );
    opts.optopt(
        "i",
        "interval",
        "Time in seconds to check health",
        "<INTERVAL>",
    );
    opts.optopt(
        "d",
        "start-delay",
        "Time in seconds to wait for first check",
        "<START_DELAY>",
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
        "t",
        "tcp-timeout",
        "Time in seconds to wait for connection to complete",
        "<TCP_TIMEOUT>",
    );
    opts.optopt(
        "k",
        "key-path",
        "The fully qualified path to requisite ssl PEM files",
        "<KEY_PATH>",
    );
    opts.optopt("a", "apprise-url", "The apprise url", "<APPRISE_URL>");
    opts.optopt(
        "j",
        "webhook-key",
        "The webhook json key string",
        "<WEBHOOK_KEY>",
    );
    opts.optopt("w", "webhook-url", "The webhook url", "<WEBHOOK_URL>");
    opts.optopt(
        "",
        "post-action",
        "The fully qualified path to a script that should be executed after container restart",
        "<SCRIPT_PATH>",
    );
    opts.optflag("h", "help", "Print help");
    opts.optflag("v", "version", "Print version information");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(e) => {
            println!("{}", e);
            println!("{}", opts.usage(&program));
            std::process::exit(1);
        }
    };

    // Process matching arguments
    if matches.opt_present("v") {
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
        connection_type: matches.opt_str("c"),
        container_label: matches.opt_str("l"),
        stop_timeout: matches.opt_str("s"),
        interval: matches.opt_str("i"),
        start_delay: matches.opt_str("d"),
        tcp_host: matches.opt_str("n"),
        tcp_port: matches.opt_str("p"),
        tcp_timeout: matches.opt_str("t"),
        key_path: matches.opt_str("k"),
        apprise_url: matches.opt_str("a"),
        webhook_key: matches.opt_str("j"),
        webhook_url: matches.opt_str("w"),
        post_action: matches.opt_str("post-action"),
    }
}
