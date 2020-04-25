use std::{error, fmt};
use std::io::Error as IOError;
use std::fs::File;
use std::io::prelude::*;
use tera::Error as TerraError;
use tera::{Context, Tera};
// use std::process::Command;

use session::Session;


// Use given template name to generate a tex file for given session and return it as a string
// template_name: name of the template (e.g. default.tex)
// session: session to render
// return: rendered tex string
fn create_tex_session(template_name: &str, session: &Session) -> Result<String, TerraError> {
    let tera = Tera::new("templates/print/*")?;
    let mut context = Context::new();
    context.insert("session", session);
    return tera.render(&template_name, &context);
}


// Print given session
// 1. generate tex string
// 2. save to file
// 3. generate and save svg image for each series
// 4. call pdflatex on this file
// 5. send generated pdf to printer
pub fn print(session: &Session) -> Result<(), Error> {
    let tex_string = create_tex_session("default.tex", &session)?;
    let mut file = File::create("templates/tmp/foo.tex")?;
    file.write_all(tex_string.as_bytes())?;


    // let _ = Command::new("sudo").arg("/sbin/shutdown");

    return Ok(());
}



#[derive(Debug)]
pub enum Error {
    TemplateError(TerraError),
    SaveError(IOError),
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::TemplateError(_) => "Error parsing template",
            Error::SaveError(_) => "Error saving rendered template",
        }
    }

    fn cause(&self) -> Option<&dyn error::Error> {
        match *self {
            Error::TemplateError(ref e) => Some(e),
            Error::SaveError(ref e) => Some(e),
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::TemplateError(ref err) =>
                write!(f, "Error parsing template: {}", err),
            Error::SaveError(ref err) =>
                write!(f, "Error saving rendered template: {}", err),
        }

    }
}

impl From<TerraError> for Error {
    fn from(err: TerraError) -> Error {
        Error::TemplateError(err)
    }
}

impl From<IOError> for Error {
    fn from(err: IOError) -> Error {
        Error::SaveError(err)
    }
}
