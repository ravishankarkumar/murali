// src/resources/latex/template.rs

pub fn latex_document(expr: &str) -> String {
    format!(
        r#"\documentclass[preview]{{standalone}}
\usepackage{{amsmath}}
\usepackage{{amssymb}}
\usepackage{{bm}}
\begin{{document}}
${}$
\end{{document}}
"#,
        expr
    )
}
