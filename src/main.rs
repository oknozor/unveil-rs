use clap::{App, Arg, SubCommand};
use unveil_rs::unveil::UnveilProject;

fn main() {
    let matches = App::new("Unveil.rs")
        .version("1.0")
        .author("Paul D. <paul.delafosse@protonmail.com>")
        .about("A assets presentation generator")
        .subcommand(SubCommand::with_name("init").arg(Arg::with_name("PROJECT_NAME")))
        .subcommand(SubCommand::with_name("build"))
        .subcommand(SubCommand::with_name("clean"))
        .subcommand(SubCommand::with_name("serve"))
        .subcommand(SubCommand::with_name("new").arg(Arg::with_name("SLIDE_NAME").required(true)))
        .get_matches();

    let mut project = UnveilProject::default();
    if matches.is_present("init") {
        let project_name = matches.subcommand_matches("init").unwrap().value_of("name");
        project.init(project_name).unwrap();
    } else if matches.is_present("build") {
        project.build().unwrap();
    } else if matches.is_present("clean") {
        UnveilProject::clean().unwrap();
    } else if matches.is_present("serve") {
        project.serve(None).unwrap()
    } else if matches.is_present("new") {
        let slide_name = matches
            .subcommand_matches("new")
            .unwrap()
            .value_of("SLIDENAME")
            .unwrap();

        project.new_slide(slide_name).unwrap()
    }
}
