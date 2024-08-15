///
/// Parser validation result variants.
///
pub enum ParserResult {
    Valid,
    Invalid,
}

///
/// Parser interface for custom validations.
///
pub trait ParamsParser {
    fn parse<I, O, T>(params: (&Option<I>, &Option<O>, &Option<T>)) -> ParserResult;
}

///
/// default params parser
///
pub struct DefaultParser;

///
/// default for `DefaultParser`
///
impl Default for DefaultParser {
    fn default() -> Self {
        Self
    }
}

///
/// default param parser implementation.
///
impl ParamsParser for DefaultParser {
    fn parse<I, O, T>(_params: (&Option<I>, &Option<O>, &Option<T>)) -> ParserResult {
        // match _params {
        //     (Some(_), Some(_), Some(_)) => ParserResult::Valid,
        //     (Some(_), None, None) => ParserResult::Valid,
        //     _ => ParserResult::Invalid
        // }
        ParserResult::Valid
    }
}
