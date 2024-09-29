use once_cell::sync::Lazy;
use tera::Tera;

pub static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    match Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/src/templates/html/**/*")) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {e}");
            std::process::exit(1);
        }
    }
});
