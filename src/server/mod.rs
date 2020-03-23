use crate::unveil::UnveilProject;
use anyhow::Result;
use iron::{
    headers,
    status,
    AfterMiddleware,
    Chain,
    Iron,
    IronError,
    IronResult,
    Request,
    Response,
    Set,
};
use std::{ffi::OsStr, fs, path::PathBuf};

mod watcher;

pub struct Server {
    pub(crate) http_port: i32,
    pub(crate) ws_port: i32,
    pub(crate) hostname: String,
    public_dir: PathBuf,
    slide_dir: PathBuf,
}

impl Default for Server {
    fn default() -> Self {
        Server {
            http_port: 7878,
            ws_port: 3000,
            hostname: "localhost".to_string(),
            public_dir: PathBuf::from("public"),
            slide_dir: PathBuf::from("slides"),
        }
    }
}

impl Server {
    pub fn serve(&self) -> Result<()> {
        let address = format!("{}:{}", self.hostname, self.http_port);
        let ws_adress = format!("{}:{}", self.hostname, self.ws_port);

        let mut chain = Chain::new(staticfile::Static::new(&self.public_dir));
        chain.link_after(NoCache);
        chain.link_after(ErrorRecover);
        let _iron = Iron::new(chain).http(&*address)?;

        let ws_server = ws::WebSocket::new(|_| |_| Ok(()))?;

        let broadcaster = ws_server.broadcaster();
        std::thread::spawn(move || {
            ws_server
                .listen(ws_adress)
                .expect("Error Opening websocket");
        });

        let serving_url = format!("http://{}", address);
        println!("Serving on: {}", serving_url);

        let mut paths = vec![];
        let entries = fs::read_dir(&self.slide_dir)?;

        for entry in entries {
            let entry = entry?;
            paths.push(entry.path());
        }

        paths.push(PathBuf::from("unveil.toml"));
        paths.push(PathBuf::from("public/unveil.css"));

        open(serving_url);

        watcher::trigger_on_change(|paths| {
            println!("Files changed: {:?}", paths);
            println!("Building presentation...");

            let mut project = UnveilProject::default();
            let result = project.build(&self);

            if let Err(e) = result {
                eprintln!("Unable to load the presentation : {}", e);
            } else {
                let _ = broadcaster.send("reload");
            }
        });

        Ok(())
    }

    pub fn with_http_port(
        mut self,
        port: Option<i32>,
    ) -> Server {
        if let Some(port) = port {
            self.http_port = port;
        }
        self
    }

    pub fn with_ws_port(
        mut self,
        port: Option<i32>,
    ) -> Server {
        if let Some(port) = port {
            self.ws_port = port;
        }
        self
    }

    pub fn with_hostname(
        mut self,
        hostname: Option<&str>,
    ) -> Server {
        if let Some(hostname) = hostname {
            self.hostname = hostname.to_owned();
        }
        self
    }
}

struct ErrorRecover;

struct NoCache;

impl AfterMiddleware for NoCache {
    fn after(
        &self,
        _: &mut Request,
        mut res: Response,
    ) -> IronResult<Response> {
        res.headers.set(headers::CacheControl(vec![
            headers::CacheDirective::NoStore,
            headers::CacheDirective::MaxAge(0u32),
        ]));

        Ok(res)
    }
}

impl AfterMiddleware for ErrorRecover {
    fn catch(
        &self,
        _: &mut Request,
        err: IronError,
    ) -> IronResult<Response> {
        match err.response.status {
            // each error will result in 404 response
            Some(_) => Ok(err.response.set(status::NotFound)),
            _ => Err(err),
        }
    }
}

fn open<P: AsRef<OsStr>>(path: P) {
    if let Err(e) = open::that(path) {
        eprintln!("Error opening web browser: {}", e);
    }
}
