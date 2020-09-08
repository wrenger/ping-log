use std::env;
use std::path::PathBuf;

mod css {
    use regex::{Captures, Regex};
    use std::fs;
    use std::path::{Path, PathBuf};

    const CSS_DIR: &str = "static/css";
    const CSS_MAIN: &str = "style.css";
    const CSS_OUT: &str = "style.css";

    fn get_path(file: &str) -> PathBuf {
        let file = Path::new(file);
        if file.is_absolute() {
            file.strip_prefix("/").unwrap().to_path_buf()
        } else {
            PathBuf::from(CSS_DIR).join(file)
        }
    }

    fn inline_imports(input: &str) -> String {
        let import = Regex::new("@import\\s*\"(.+)\"\\s*;").unwrap();
        import
            .replace_all(input, |caps: &Captures| {
                let file = get_path(&caps[1]);
                println!("cargo:rerun-if-changed={:?}", file);
                let content = fs::read_to_string(file).unwrap();
                inline_imports(&content)
            })
            .into_owned()
    }

    fn inline_images(input: &str) -> String {
        // TODO: add support for different formats
        let image_svg = Regex::new("url\\((.+\\.svg)\\)").unwrap();
        image_svg
            .replace_all(input, |caps: &Captures| {
                let file = get_path(&caps[1]);
                let content = fs::read_to_string(file).unwrap();
                let content = base64::encode(content.trim());
                format!("url(\"data:image/svg+xml;base64,{}\")", content)
            })
            .into_owned()
    }

    fn inline(input: &str) -> String {
        let result = inline_imports(input);
        let result = inline_images(&result);
        result
    }

    pub fn build(outdir: &Path) {
        let file: PathBuf = [CSS_DIR, CSS_MAIN].iter().collect();
        println!("cargo:rerun-if-changed={:?}", &file);

        let input = fs::read_to_string(file).unwrap();
        let result = inline(&input);
        fs::write(outdir.join(CSS_OUT), &result).unwrap();
    }
}

mod js {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    const JS_FILES: [&str; 3] = [
        "static/js/moment.min.js",
        "static/js/Chart.min.js",
        "static/js/ping-web.js",
    ];

    const JS_OUT: &str = "app.js";

    pub fn build(outdir: &Path) {
        let mut bundle = String::new();
        for path in &JS_FILES {
            println!("cargo:rerun-if-changed={}", path);

            bundle += &format!("\n// {}:\n", path);
            let mut file = fs::File::open(path).unwrap();
            file.read_to_string(&mut bundle).unwrap();
        }
        fs::write(outdir.join(JS_OUT), &bundle).unwrap();
    }
}

mod html {
    use std::fs;
    use std::path::Path;

    const HTML_FILE: &str = "static/index.html";
    const HTML_OUT: &str = "index.html";

    pub fn build(outdir: &Path) {
        println!("cargo:rerun-if-changed={}", HTML_FILE);

        let content = fs::read_to_string(HTML_FILE).unwrap();
        fs::write(outdir.join(HTML_OUT), &content).unwrap();
    }
}

fn main() {
    let outdir = PathBuf::from(&env::var("OUT_DIR").unwrap());

    css::build(&outdir);
    js::build(&outdir);
    html::build(&outdir);
}
