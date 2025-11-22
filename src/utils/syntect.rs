use crate::api_error::ApiError;
use actix_web::web::Data;
use syntect::highlighting::Theme;
use syntect::html::highlighted_html_for_string;
use syntect::parsing::SyntaxSet;

pub fn highlight_string(
    content: &str,
    language: &str,
    syntect_theme: Data<Theme>,
    syntax_set: Data<SyntaxSet>,
) -> Result<String, ApiError> {
    let syntax = syntax_set
        .find_syntax_by_token(language)
        .unwrap_or_else(|| syntax_set.find_syntax_plain_text());

    Ok(
        highlighted_html_for_string(content, &syntax_set, syntax, &syntect_theme)?
            .replace(r#" style="background-color:#ffffff;""#, ""),
    )
}
