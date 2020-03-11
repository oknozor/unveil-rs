use unveil_rs::files::UnveilProject;

fn main() {
    let mut  project = UnveilProject {
        markdown: vec![]
    };

    project.build().unwrap();
}