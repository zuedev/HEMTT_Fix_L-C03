use std::sync::Arc;

use hemtt_common::{
    reporting::{Code, Diagnostic},
    similar_values,
    workspace::WorkspacePath,
};

use crate::Error;

pub struct ScriptNotFound {
    script: String,
    similar: Vec<String>,
}

impl Code for ScriptNotFound {
    fn ident(&self) -> &'static str {
        "BHE1"
    }

    fn message(&self) -> String {
        format!("Script not found: {}", self.script)
    }

    fn help(&self) -> Option<String> {
        if self.similar.is_empty() {
            None
        } else {
            Some(format!("did you mean `{}`?", self.similar.join("`, `")))
        }
    }

    fn diagnostic(&self) -> Option<Diagnostic> {
        Some(Diagnostic::simple(self))
    }
}

impl ScriptNotFound {
    pub fn code(script: String, scripts: &WorkspacePath) -> Result<Arc<dyn Code>, Error> {
        let scripts = scripts
            .read_dir()?
            .iter()
            .filter_map(|x| {
                if x.is_file().map_or(false, |x| x) {
                    Some(x.filename())
                } else {
                    None
                }
            })
            .collect::<Vec<String>>();
        Ok(Arc::new(Self {
            similar: similar_values(
                &script,
                &scripts
                    .iter()
                    .map(std::string::String::as_str)
                    .collect::<Vec<&str>>(),
            )
            .iter()
            .map(std::string::ToString::to_string)
            .collect(),
            script,
        }))
    }
}
