pub mod organization;
pub mod project;
pub mod user;

use std::{fs::File, io::BufReader, path::Path};

use crate::error::ZitadelCLIError;

pub fn load_from_file<T>(path: &Path) -> Result<T, ZitadelCLIError>
where
    T: serde::de::DeserializeOwned,
{
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let payload: T = serde_json::from_reader(reader)?;
    Ok(payload)
}

#[cfg(test)]
mod tests {
    use crate::payloads::user::NewHumanUser;

    use super::*;

    #[test]
    /// TODO: Compare the content of the user loaded from the file with the expected user
    fn test_load_from_file() {
        let path = Path::new("add_user.json");
        load_from_file::<NewHumanUser>(path).unwrap();
    }
}
