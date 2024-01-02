use crate::types::question::Question;
use handle_errors::Error;
use std::collections::HashMap;

/// Pagination struct that is getting extracted
/// from query params
#[derive(Debug)]
pub struct Pagination {
    /// The index of the first item that has to be returned
    pub start: usize,
    /// The index of the last item that has to be returned
    pub end: usize,
}
/// Extract query parameters from the `/questions` route
/// # Example query
/// GET requests to this route can have a pagination attached so we just
/// return the questions we need
/// `/questions?start=1&end=10`
/// /// # Example usage
/// ```rust
/// let mut query = HashMap::new();
/// query.insert("start".to_string(), "1".to_string());
/// query.insert("end".to_string(), "10".to_string());
/// let p = types::pagination::extract_pagination(query).unwrap();
/// assert_eq!(p.start, 1);
/// assert_eq!(p.end, 10);
/// ```
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
                // Takes the "start" parameter in the query
                // and tries to convert it to a number
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
                // Takes the "end" parameter in the query
                // and tries to convert it to a number
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