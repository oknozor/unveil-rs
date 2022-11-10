use notify::{Config, RecommendedWatcher, Watcher};
use std::{path::PathBuf, sync::mpsc::channel};
use std::collections::HashSet;
use std::path::Path;
use notify::EventKind::{Create, Modify, Remove};

pub fn trigger_on_change<F>(closure: F)
    where
        F: Fn(&HashSet<PathBuf>),
{
    use notify::{RecursiveMode::*};

    // Create a channel to receive the events.
    let (tx, rx) = channel();
    let mut watcher = RecommendedWatcher::new(tx, Config::default()).unwrap();

    // Add the source directory to the watcher
    if let Err(e) = watcher.watch(Path::new("slides"), Recursive) {
        eprintln!("Error while watching slides directory/ :\n    {:?}", e);
        std::process::exit(1);
    };

    // Watch changes on unveil.toml config and default CSS
    let _ = watcher.watch(Path::new("public/unveil.css"), NonRecursive);
    let _ = watcher.watch(Path::new("unveil.toml"), NonRecursive);

    println!("Listening for changes...");
    let mut paths = HashSet::new();

    for events in rx {
        match events {
            Err(e) => eprintln!("Failed to reload project: {}", e),
            Ok(event) => {
                let is_style = event.paths.iter().any(|path| path.ends_with("unveil.css"));
                let is_config = event.paths.iter().any(|path| path.ends_with("unveil.toml"));
                match event.kind {
                    // Since we are not watching for the whole project directory,
                    // file watchers are dropped on modification so we have to recreate them.
                    Remove(_) if is_config || is_style => {
                        let css = is_style;
                        let config = is_config;
                        if css {
                            let _ = watcher.watch(Path::new("public/unveil.css"), NonRecursive);
                            paths.extend(event.paths)
                        } else if config {
                            let _ = watcher.watch(Path::new("unveil.toml"), NonRecursive);
                            paths.extend(event.paths)
                        }
                    }
                    Create(_) | Modify(_) | Remove(_) => paths.extend(event.paths),
                    _ => {}
                }
            }
        }

        if !paths.is_empty() {
            closure(&paths);
        }
    }
}
