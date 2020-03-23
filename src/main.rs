use clap::{App, Arg, SubCommand, AppSettings};
use unveil_rs::unveil::UnveilProject;
use unveil_rs::server::Server;

fn main() {
    let matches = App::new("Unveil.rs")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A markdown presentation generator")
        .settings(&[
            AppSettings::VersionlessSubcommands,
            AppSettings::SubcommandRequiredElseHelp,
            AppSettings::ColorAuto,
            AppSettings::ColoredHelp,
        ])
        .subcommands(vec![
            SubCommand::with_name("init")
                .display_order(1)
                .about("Initialise a presentation project in the current directory")
                .arg(Arg::with_name("PROJECT_NAME")
                    .help("The name of your project, default `unveil`")
                ),
            SubCommand::with_name("build")
                .display_order(2)
                .about("Build the project static site files in `public` directory"),
            SubCommand::with_name("serve")
                .display_order(3)
                .about("Serve your project with live-reload")
                .args(&[Arg::with_name("host")
                    .required(false)
                    .long("hostname")
                    .short("h")
                    .takes_value(true)
                    .help("Hostname to serve on, default `localhost`"),
                    Arg::with_name("http-port")
                        .required(false)
                        .long("http-port")
                        .short("p")
                        .takes_value(true)
                        .help("Http port to serve on, default `7878`"),
                    Arg::with_name("ws-port")
                        .required(false)
                        .long("ws-port")
                        .short("w")
                        .takes_value(true)
                        .help("Web socket port used for live-reload, default `3000`")
                ]),
            SubCommand::with_name("add")
                .display_order(4)
                .about("Add a markdown slide to the presentation")
                .arg(Arg::with_name("SLIDE_NAME").required(true)),
            SubCommand::with_name("clean")
                .display_order(5)
                .about("Remove all static files and the `public` directory"),
        ])
        .get_matches();

    let mut project = UnveilProject::default();
    match matches.subcommand_name().unwrap() {
        "init" => {
            let project_name = matches
                .subcommand_matches("init")
                .unwrap()
                .value_of("PROJECT_NAME");
            project.init(project_name).unwrap();
        }
        "build" => project.build(&Server::default()).unwrap(),
        "serve" => {
            let serve = matches.subcommand_matches("serve")
                .unwrap();
            let http_port = serve.value_of("http-port")
                .map(|value| value.parse::<i32>().expect("Error : invalid http port"));
            let ws_port = serve.value_of("ws-port")
                .map(|value| value.parse::<i32>().expect("Error : invalid websocket port"));
            let hostname = serve.value_of("hostname");

            project.serve(hostname, http_port, ws_port).unwrap()
        }
        "add" => {
            let slide_name = matches
                .subcommand_matches("add")
                .unwrap()
                .value_of("SLIDE_NAME")
                .unwrap();

            project.new_slide(slide_name).unwrap()
        }
        "clean" => UnveilProject::clean().unwrap(),
        _ => ()
    }
}
