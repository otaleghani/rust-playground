use minijinja::Environment;
use rust_embed::RustEmbed;
use serde::Serialize;
use std::sync::{Arc, RwLock};

#[derive(RustEmbed)]
#[folder = "./templates"]
struct TemplateFiles;

#[derive(Clone)]
pub struct Templates {
    pub env: Arc<RwLock<Environment<'static>>>,
}

impl Default for Templates {
    fn default() -> Self {
        let mut env = Environment::new();
        env.set_loader(embedded_loader);
        env.add_filter("scream", scream);
        env.add_test("is_sandro", is_sandro);
        let env = Arc::new(RwLock::new(env));
        Self { env }
    }
}

impl Templates {
    pub fn render_templates<S: Serialize>(
        &self,
        name: &str,
        ctx: S,
    ) -> Result<String, minijinja::Error> {
        let some = self.env.read().unwrap();
        some.get_template(name)
            .map_err(|e| {
                println!("Encountered some error: {}", e.to_string());
                e
            })?
            .render(ctx)
            .map_err(|e| {
                println!("Encountered some rendering error: {}", e.to_string());
                e
            })
    }
}

fn embedded_loader(name: &str) -> Result<Option<String>, minijinja::Error> {
    let Some(file) = TemplateFiles::get(name) else {
        return Ok(None);
    };

    let val = String::from_utf8(file.data.to_vec())
        .map_err(|_| minijinja::Error::from(minijinja::ErrorKind::CannotDeserialize))?;

    Ok(Some(val))
}

// Filters / Tests
pub fn scream(value: String) -> String {
    value.to_uppercase()
}
pub fn is_sandro(value: String) -> bool {
    value == "Sandro".to_string()
}
