use std::io::Write;
use crate::result::Result;
use crate::model::{Application, Graph};

pub fn dot<W: Write>(w: &mut W, g: Graph) -> Result<()> {
    writeln!(w, "digraph {{")?;

    for a in g.applications {
        dot_app(w, a)?;
    }

    writeln!(w, "}}")?;

    Ok(())
}

fn dot_app<W: Write>(w: &mut W, a: Application) -> Result<()> {
    writeln!(w, "\"{}/{}\";", a.namespace, a.app_id)?;

    if let Some(input) = a.consumes {
        writeln!(w, "\"{}\" -> \"{}/{}\";", input, a.namespace, a.app_id)?;
    }

    if let Some(output) = a.produces {
        writeln!(w, "\"{}/{}\" -> \"{}\";", a.namespace, a.app_id, output)?;
    }

    Ok(())
}