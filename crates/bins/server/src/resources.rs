use std::path::PathBuf;

pub enum Resources {
    Index,
    React,
    ReactDom,
    Js,
    Css,
    IconsEot,
    IconsTTF,
    IconsWOFF,
    IconsWOFF2,
}

#[cfg(not(debug_assertions))]
impl Resources {
    pub fn load(&self) -> &[u8] {
        match self {
            Self::Index => include_bytes!("index.html"),
            Self::React => include_bytes!("react.development.js"),
            Self::ReactDom => include_bytes!("react-dom.development.js"),
            Self::Js => include_bytes!("main.js"),
            Self::Css => include_bytes!("main.css"),
            Self::IconsEot => include_bytes!("Icons-Regular.eot"),
            Self::IconsTTF => include_bytes!("Icons-Regular.ttf"),
            Self::IconsWOFF => include_bytes!("Icons-Regular.woff"),
            Self::IconsWOFF2 => include_bytes!("Icons-Regular.woff2"),
        }
    }
}

#[cfg(debug_assertions)]
impl Resources {
    pub fn load(&self) -> Vec<u8> {
        let path = match self {
            Self::Index => "index.html",
            Self::React => "react.development.js",
            Self::ReactDom => "react-dom.development.js",
            Self::Js => "main.js",
            Self::Css => "main.css",
            Self::IconsEot => "Icons-Regular.eot",
            Self::IconsTTF => "Icons-Regular.ttf",
            Self::IconsWOFF => "Icons-Regular.woff",
            Self::IconsWOFF2 => "Icons-Regular.woff2",
        };
        let dir: PathBuf = file!().into();
        let path = dir.parent().expect("parent dir").join(path);
        log::info!("Loading {}", path.display());
        std::fs::read(path).expect("resource")
    }
}
