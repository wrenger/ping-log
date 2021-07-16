use std::env;
use std::path::PathBuf;

mod css {
    use regex::{Captures, Regex};
    use std::fs;
    use std::path::{Path, PathBuf};

    fn get_path(file: &str, dir: &str) -> PathBuf {
        let file = Path::new(file);
        if file.is_absolute() {
            file.strip_prefix("/").unwrap().to_path_buf()
        } else {
            PathBuf::from(dir).join(file)
        }
    }

    fn inline_imports(input: &str, dir: &str) -> String {
        let import = Regex::new("@import\\s*\"(.+)\"\\s*;").unwrap();
        import
            .replace_all(input, |caps: &Captures| {
                let file = get_path(&caps[1], dir);
                println!("cargo:rerun-if-changed={:?}", file);
                let content = fs::read_to_string(file).unwrap();
                inline_imports(&content, dir)
            })
            .into_owned()
    }

    fn inline_images(input: &str, dir: &str) -> String {
        // TODO: add support for different formats
        let image_svg = Regex::new("url\\((.+\\.svg)\\)").unwrap();
        image_svg
            .replace_all(input, |caps: &Captures| {
                let file = get_path(&caps[1], dir);
                let content = fs::read_to_string(file).unwrap();
                let content = base64::encode(content.trim());
                format!("url(\"data:image/svg+xml;base64,{}\")", content)
            })
            .into_owned()
    }

    fn inline(input: &str, dir: &str) -> String {
        let result = inline_imports(input, dir);
        let result = inline_images(&result, dir);
        result
    }

    pub fn build(mainfile: &str, dir: &str, outdir: &Path) {
        let file: PathBuf = [dir, mainfile].iter().collect();
        println!("cargo:rerun-if-changed={:?}", &file);

        let input = fs::read_to_string(file).unwrap();
        let result = inline(&input, dir);
        fs::write(outdir.join(mainfile), &result).unwrap();
    }
}

mod js {
    use std::fs;
    use std::io::Read;
    use std::path::Path;

    pub struct Target {
        pub input: Vec<String>,
        pub output: String,
    }

    pub fn build(outdir: &Path, targets: &[Target]) {
        for target in targets {
            let mut bundle = String::new();
            for path in &target.input {
                println!("cargo:rerun-if-changed={}", path);

                bundle += &format!("\n// {}:\n", path);
                let mut file = fs::File::open(path).unwrap();
                file.read_to_string(&mut bundle).unwrap();
            }
            fs::write(outdir.join(&target.output), &bundle).unwrap();
        }
    }
}

fn main() {
    let outdir = PathBuf::from(&env::var("OUT_DIR").unwrap());

    css::build("style.css", "static/css", &outdir);

    let targets = vec![
        js::Target {
            input: vec![
                "static/js/moment.min.js".into(),
                "static/js/Chart.min.js".into(),
                "static/js/ping.js".into(),
            ],
            output: "ping.js".into(),
        }
    ];
    js::build(&outdir, &targets);
}
