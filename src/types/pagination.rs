use crate::types::question::Question;
use handle_errors::Error;
use std::collections::HashMap;

#[derive(Debug)]
pub struct Pagination {
    pub start: usize,
    pub end: usize,
}

pub fn extract_pagination(params: HashMap<String, String>) -> Result<Pagination, Error> {
    // Uses the .contains method on the
    // HashMap to check if both
    // parameters are there
    if params.contains_key("start") && params.contains_key("end") {
        // If both parameters are there, we return Result
        // (via return Ok()). We need the return keyword
        // here because we want to return early.
        return Ok(
            // Creates a new Pagination
            // object and sets the start
            // and end number
            Pagination {
                start: params
                    // The .get method on HashMap returns an
                    // option, because it can’t be sure that the key
                    // exists. We can do the unsafe .unwrap here,
                    // because we already checked if both parameters
                    // are in the HashMap a few lines earlier. We parse
                    // the containing &str value to a usize integer
                    // type. This returns Result, which we unwrap or
                    // return an error if it fails via .map_err and the
                    // question mark at the end of the line.
                    .get("start")
                    .unwrap()
                    .parse::<usize>()
                    .map_err(Error::ParseError)?,
                end: params
                    .get("end")
                    .unwrap()
                    .parse::<usize>()
                    .map_err(Error::ParseError)?,
            },
        );
    }
    // If not, the if clause isn’t being executed and we go
    // right down to Err, where we return our custom
    // MissingParameters error, which we access from
    // the Error enum with the double colons (::).
    Err(Error::MissingParameters)
}

pub fn check_valid_pagination_range(
    pagination: &Pagination,
    res: Vec<Question>,
) -> Result<Vec<Question>, Error> {
    if pagination.start > res.len()
        || pagination.end > res.len()
        || pagination.end > pagination.start
    {
        return Ok(res);
    }
    Err(Error::InvalidRange)
}
