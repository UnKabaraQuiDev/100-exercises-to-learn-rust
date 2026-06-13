// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for `Status`.
//  The parsing should be case-insensitive.

use std::future::IntoFuture;
use thiserror::Error;

#[derive(Debug, PartialEq, Clone)]
enum Status {
    ToDo,
    InProgress,
    Done,
}

#[derive(Error, Debug)]
enum TryFromError {
    #[error("Not found.")]
    NotFound,
}

impl TryFrom<String> for Status {
    type Error = TryFromError;
    fn try_from(value: String) -> Result<Self, Self::Error> {
        let lowercased = value.to_lowercase();
        if lowercased == "todo" {
            return Ok(Self::ToDo);
        } else if lowercased == "inprogress" {
            return Ok(Self::InProgress);
        } else if lowercased == "done" {
            return Ok(Self::Done);
        }
        Err(Self::Error::NotFound)
    }
}

impl TryFrom<&str> for Status {
    type Error = TryFromError;
    fn try_from(val: &str) -> Result<Self, Self::Error> {
        let value: String = String::from(val);
        let lowercased = value.to_lowercase();
        if lowercased == "todo" {
            return Ok(Self::ToDo);
        } else if lowercased == "inprogress" {
            return Ok(Self::InProgress);
        } else if lowercased == "done" {
            return Ok(Self::Done);
        }
        Err(Self::Error::NotFound)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let status = Status::try_from("ToDO".to_string()).unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inproGress".to_string()).unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("Done".to_string()).unwrap();
        assert_eq!(status, Status::Done);
    }

    #[test]
    fn test_try_from_str() {
        let status = Status::try_from("todo").unwrap();
        assert_eq!(status, Status::ToDo);

        let status = Status::try_from("inprogress").unwrap();
        assert_eq!(status, Status::InProgress);

        let status = Status::try_from("done").unwrap();
        assert_eq!(status, Status::Done);
    }
}
