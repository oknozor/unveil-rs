use notify::Watcher;
use std::{path::PathBuf, sync::mpsc::channel, thread::sleep, time::Duration};

pub fn trigger_on_change<F>(closure: F)
where
    F: Fn(Vec<PathBuf>),
{
    use notify::{DebouncedEvent::*, RecursiveMode::*};

    // Create a channel to receive the events.
    let (tx, rx) = channel();

    let mut watcher = match notify::watcher(tx, Duration::from_secs(1)) {
        Ok(w) => w,
        Err(e) => {
            eprintln!("Error while trying to watch the files:\n\n\t{:?}", e);
            std::process::exit(1)
        }
    };

    // Add the source directory to the watcher
    if let Err(e) = watcher.watch("slides", Recursive) {
        eprintln!("Error while watching slides directory/ :\n    {:?}", e);
        std::process::exit(1);
    };

    // Watch changes on unveil.toml config and default CSS
    let _ = watcher.watch("public/unveil.css", NonRecursive);
    let _ = watcher.watch("unveil.toml", NonRecursive);

    println!("Listening for changes...");

    loop {
        let first_event = rx.recv().unwrap();
        sleep(Duration::from_millis(50));
        let other_events = rx.try_iter();

        let all_events = std::iter::once(first_event).chain(other_events);

        let paths = all_events
            .filter_map(|event| {
                println!("Received filesystem event: {:?}", event);

                match event {
                    Create(path) | Write(path) | Remove(path) | Rename(_, path) => Some(path),
                    // Since we are not watching for the whole project directory,
                    // file watchers are dropped on modification so we have to recreate them.
                    NoticeRemove(path) => {
                        let path_str = path.to_str().unwrap();

                        if path_str.contains("unveil.css") {
                            let _ = watcher.watch("public/unveil.css", NonRecursive);
                            Some(path)
                        } else if path_str.contains("unveil.toml") {
                            let _ = watcher.watch("unveil.toml", NonRecursive);
                            Some(path)
                        } else {
                            None
                        }
                    }
                    _ => None,
                }
            })
            .collect::<Vec<_>>();

        if !paths.is_empty() {
            closure(paths);
        }
    }
}
