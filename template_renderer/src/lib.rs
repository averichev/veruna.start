use tera::{Context, Tera};

pub struct TemplateRenderer {
    tera: Tera,
}

impl TemplateRenderer {
    pub fn new() -> TemplateRenderer {
        let mut tera = match Tera::new("./templates/**/*.html.twig") {
            Ok(t) => t,
            Err(e) => {
                println!("Parsing error(s): {}", e);
                std::process::exit(1);
            }
        };
        tera.full_reload().unwrap();
        TemplateRenderer { tera }
    }
    pub fn render(&self) -> tera::Result<String> {
        self.tera.render("index.html.twig", &Context::new())
    }
}