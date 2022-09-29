use crate::cores::error::service::Error;

pub struct Fields {
    invalid_data: Vec<&'static str>,
}

impl Fields {
    pub fn new() -> Self {
        Self {
            invalid_data: Vec::new(),
        }
    }

    pub fn add(&mut self, invalid: &'static str) {
        self.invalid_data.push(&invalid);
    }

    pub fn check(&self) -> Result<(), Error> {
        if self.invalid_data.len() != 0 {
            let invalid_data_string = self.invalid_data.join(",");
            tracing::error!("Error Validations: {}", invalid_data_string);
            return Err(Error::BadRequest(invalid_data_string));
        }
        Ok(())
    }
}
