pub mod status;

#[cfg(test)]
mod tests {
    use crate::status::StatusCode;

    #[test]
    fn it_works() {
        assert!(StatusCode::OK.is_success());
    }
}
