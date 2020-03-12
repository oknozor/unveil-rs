use unveil_rs::unveil::UnveilProject;
use clap::{App, SubCommand, Arg};

fn main() {

    // TODO : clap
    // - init
    // - build (project.build())
    // - run (actix) + watch ???
    // - --new-slide

    let matches = App::new("Unveil.rs")
        .version("1.0")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A static presentation generator")
        .subcommand(SubCommand::with_name("init")
            .arg(Arg::with_name("name"))
        )
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("serve"))
        .subcommand(SubCommand::with_name("new"))
        .get_matches();

    let mut project = UnveilProject::default();
    if matches.is_present("init") {
        let project_name = matches.subcommand_matches("init")
            .unwrap()
            .value_of("name");

        project.init(project_name).unwrap();

    } else if matches.is_present("build") {

        project.build().unwrap();
    } else if matches.is_present("serve") {

        project.serve(None).unwrap()
    } else if matches.is_present("new") {
        project.new_slide("slide_name").unwrap()
    }
}