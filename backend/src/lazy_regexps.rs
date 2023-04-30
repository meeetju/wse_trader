use lazy_regex::{regex, Lazy, Regex};

pub static RE_TICKER: &Lazy<Regex> = regex!(r">([A-Z0-9]{3})");
pub static RE_NAME: &Lazy<Regex> = regex!(r"\(([A-Z0-9]*)\)");
pub static RE_ALTMAN: &Lazy<Regex> = regex!(r">([A-D]{1,3}[\+\-]*)</span>");
pub static RE_F_SCORE: &Lazy<Regex> = regex!(r">([0-9])</span>");
pub static RE_FLOAT: &Lazy<Regex> = regex!(r">([0-9]*.[0-9]*)%?</div>");
