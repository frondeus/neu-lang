use std::path::PathBuf;

pub enum Resources {
    Index,
    React,
    ReactDom,
    Js,
    Css
}

#[cfg(not(debug_assertions))]
impl Resources {
    pub fn load(&self) -> &str {
        match self {
            Self::Index => include_str!("index.html"),
            Self::React => include_str!("react.development.js"),
            Self::ReactDom => include_str!("react-dom.development.js"),
            Self::Js => include_str!("main.js"),
            Self::Css => include_str!("main.css"),
        }
    }
}

#[cfg(debug_assertions)]
impl Resources {
    pub fn load(&self) -> String {
        let path = match self {
            Self::Index => "index.html",
            Self::React => "react.development.js",
            Self::ReactDom => "react-dom.development.js",
            Self::Js => "main.js",
            Self::Css => "main.css",
        };
        let dir: PathBuf = file!().into();
        let path = dir.parent().expect("parent dir").join(path);
        log::info!("Loading {}", path.display());
        std::fs::read_to_string(path).expect("resource")
    }
}
