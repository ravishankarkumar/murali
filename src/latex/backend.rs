use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};

use crate::latex::error::LatexError;
use crate::latex::template::latex_document;

/// Result of LaTeX compilation
pub struct LatexSvg {
    pub svg_path: PathBuf,
}

pub fn compile_latex_to_svg(
    latex_src: &str,
    work_dir: &Path,
) -> Result<LatexSvg, LatexError> {
    fs::create_dir_all(work_dir)?;

    let tex_path = work_dir.join("expr.tex");
    let dvi_path = work_dir.join("expr.dvi");
    let svg_path = work_dir.join("expr.svg");

    // 1. Write .tex
    fs::write(&tex_path, latex_document(latex_src))?;

    // 2. Run latex → dvi
    let latex_out = Command::new("latex")
        .arg("-interaction=nonstopmode")
        .arg("expr.tex")
        .current_dir(work_dir)
        .output()
        .map_err(|_| LatexError::LatexNotFound)?;

    if !latex_out.status.success() {
        return Err(LatexError::LatexFailed(
            String::from_utf8_lossy(&latex_out.stderr).to_string(),
        ));
    }

    if !dvi_path.exists() {
        return Err(LatexError::LatexFailed(
            "latex did not produce DVI output".into(),
        ));
    }

    // 3. Run dvisvgm (NO FONTS)
    let dvisvgm_out = Command::new("dvisvgm")
        .arg("--no-fonts")
        .arg("expr.dvi")
        .arg("-o")
        .arg("expr.svg")
        .current_dir(work_dir)
        .output()
        .map_err(|_| LatexError::DviSvgmNotFound)?;

    if !dvisvgm_out.status.success() {
        return Err(LatexError::DviSvgmFailed(
            String::from_utf8_lossy(&dvisvgm_out.stderr).to_string(),
        ));
    }

    if !svg_path.exists() {
        return Err(LatexError::DviSvgmFailed(
            "dvisvgm did not produce SVG output".into(),
        ));
    }

    Ok(LatexSvg { svg_path })
}
