use ascii::AsciiType;

#[derive(Debug)]
pub struct Command {
    ascii_type: AsciiType,
    source: String,
    output: String,
    scale: f32,
    cols: u32,
}

impl Command {
    pub fn ascii_type(&self) -> &AsciiType {
        &self.ascii_type
    }

    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn output(&self) -> &str {
        &self.output
    }

    pub fn scale(&self) -> &f32 {
        &self.scale
    }

    pub fn cols(&self) -> &u32 {
        &self.cols
    }
}

pub fn read_command() -> Command {
    let matches = clap_app!(img_to_ascii =>
        (version: "0.1.0")
        (author: "Jérôme Mahuet <jerome.mahuet@gmail.com>")
        (about: "Convert an image to ascii art.")
        (@arg type: -t --type +takes_value "Sets the type - simple|complex")
        (@arg src: -s --src +required +takes_value "Path of the source image")
        (@arg output: -o --output +required +takes_value "Path of the output file")
        (@arg scale: --scale +takes_value "Font scaling")
        (@arg cols: --cols +takes_value "Number of columns")
    )
        .get_matches();

    Command {
        ascii_type: read_type(matches.value_of("type").unwrap_or("simple")),
        source: matches.value_of("src").unwrap().to_string(),
        output: matches.value_of("output").unwrap().to_string(),
        scale: value_t!(matches.value_of("scale"), f32).unwrap_or(0.43),
        cols: value_t!(matches.value_of("cols"), u32).unwrap_or(80),
    }
}

fn read_type(t: &str) -> AsciiType {
    match t {
        "simple" => AsciiType::Simple,
        "complex" => AsciiType::Complex,
        _ => AsciiType::Simple,
    }
}
