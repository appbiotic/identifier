use appbiotic_error::{Status, StatusDetails, StatusResult};

pub trait IdentifierGenerator {
    fn generate_identifier(&self) -> StatusResult<String>;
}

#[cfg(feature = "uuid")]
pub struct UuidIdentifierGenerator;

#[cfg(feature = "uuid")]
impl IdentifierGenerator for UuidIdentifierGenerator {
    fn generate_identifier(&self) -> StatusResult<String> {
        Ok(uuid::Uuid::now_v7().hyphenated().to_string())
    }
}

#[cfg(feature = "petname")]
pub struct PetnameIdentifierGenerator {
    words: u8,
    separator: String,
}

#[cfg(feature = "petname")]
impl Default for PetnameIdentifierGenerator {
    fn default() -> Self {
        Self {
            words: 3,
            separator: "-".to_string(),
        }
    }
}

#[cfg(feature = "petname")]
impl IdentifierGenerator for PetnameIdentifierGenerator {
    fn generate_identifier(&self) -> StatusResult<String> {
        petname::petname(self.words, &self.separator).ok_or_else(|| {
            Status::Internal(StatusDetails {
                message: "Ran out of pet names".to_string(),
                error_details: Vec::default(),
            })
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn uuid_generator_works() {
        let generator = UuidIdentifierGenerator;
        let id = generator.generate_identifier().unwrap();
        assert_eq!(id.chars().filter(|x| x.eq(&'-')).count(), 4);
        assert_eq!(id.len(), 36);
    }

    #[test]
    fn petname_generator_works() {
        let generator = PetnameIdentifierGenerator::default();
        let id = generator.generate_identifier().unwrap();
        assert_eq!(
            id.chars().filter(|x| x.eq(&'-')).count(),
            generator.words.saturating_sub(1).into()
        );
    }
}
