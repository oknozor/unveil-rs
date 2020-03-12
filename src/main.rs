use unveil_rs::unveil::UnveilProject;
use unveil_rs::generated::CSS;
use unveil_rs::generated::JS;

fn main() {

    // TODO : clap
    // - init
    // - build (project.build())
    // - run (actix) + watch ???

    println!("Unveil js : {}", JS);
    println!("Unveil css : {}", CSS);

    let mut  project = UnveilProject {
        markdown: vec![]
    };

    project.build().unwrap();
}