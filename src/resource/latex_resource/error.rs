// src/resources/latex/error.rs

use thiserror::Error;

#[derive(Debug, Error)]
pub enum LatexError {
    #[error("LaTeX executable not found. Please install TeX Live / MacTeX.")]
    LatexNotFound,

    #[error("dvisvgm executable not found. Please install dvisvgm.")]
    DviSvgmNotFound,

    #[error("LaTeX compilation failed:\n{0}")]
    LatexFailed(String),

    #[error("dvisvgm failed:\n{0}")]
    DviSvgmFailed(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
