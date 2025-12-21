use std::path::{Path, PathBuf};
use std::process::Command;
use std::{fs, io};
use sha2::{Sha256, Digest};

use crate::resource::latex_resource::error::LatexError; // Updated paths
use crate::resource::latex_resource::template::latex_document;

pub struct LatexResource {
    pub svg_content: String,
    pub hash: String,
}

/// Compiles LaTeX source to SVG. 
/// Now designed to be called by the Resource Cache, not the Frontend.
pub fn compile_latex(latex_src: &str, cache_dir: &Path) -> Result<LatexResource, LatexError> {
    // 1. Generate a unique hash for this source to avoid redundant work
    let mut hasher = Sha256::new();
    hasher.update(latex_src);
    let hash = format!("{:x}", hasher.finalize());
    
    let work_dir = cache_dir.join(&hash);
    fs::create_dir_all(&work_dir)?;

    let tex_path = work_dir.join("expr.tex");
    let dvi_path = work_dir.join("expr.dvi");
    let svg_path = work_dir.join("expr.svg");

    // 2. Check if we already compiled this in a previous run
    if svg_path.exists() {
        let svg_content = fs::read_to_string(&svg_path)?;
        return Ok(LatexResource { svg_content, hash });
    }

    // 3. Write .tex using the template
    fs::write(&tex_path, latex_document(latex_src))?;

    // 4. Run latex → dvi
    // We use -output-directory to keep the cache clean
    let latex_out = Command::new("latex")
        .arg("-interaction=nonstopmode")
        .arg("-output-directory")
        .arg(&work_dir)
        .arg(&tex_path)
        .output()
        .map_err(|_| LatexError::LatexNotFound)?;

    if !latex_out.status.success() {
        return Err(LatexError::LatexFailed(
            String::from_utf8_lossy(&latex_out.stdout).to_string(),
        ));
    }

    // 5. Run dvisvgm (using --no-fonts to convert glyphs to SVG paths)
    // This makes the output backend-agnostic as we don't need to load .ttf files
    let dvisvgm_out = Command::new("dvisvgm")
        .arg("--no-fonts")
        .arg("--precision=3")
        .arg("-o")
        .arg(&svg_path)
        .arg(&dvi_path)
        .output()
        .map_err(|_| LatexError::DviSvgmNotFound)?;

    if !dvisvgm_out.status.success() {
        return Err(LatexError::DviSvgmFailed(
            String::from_utf8_lossy(&dvisvgm_out.stderr).to_string(),
        ));
    }

    let svg_content = fs::read_to_string(&svg_path)?;
    
    Ok(LatexResource { svg_content, hash })
}