use std::fmt::{Display, Formatter};
use worker::FormData;

#[derive(Clone, Copy, Debug)]
pub enum FormDataError {
    MissingField(&'static str),                 // str field
    MalformedField(&'static str, &'static str), // str field, str expl
}

/// form: &FormData
/// target_type: StringField/EmailField... (any type that implements TryFrom<String> where Error = &'static str)
/// field_name: &'static str
///
/// -> Result<target_type, FormDataError>
macro_rules! parse_form_field {
    ($form:expr, $target_type:ty, $field_name:literal) => {
        if let Some(s) = $form.get_field($field_name) {
            if &s == "" {
                Err(FormDataError::MissingField($field_name))
            } else {
                match <$target_type>::try_from(s) {
                    Ok(f) => Ok(f),
                    Err(explanation) => {
                        Err(FormDataError::MalformedField($field_name, explanation))
                    }
                }
            }
        } else {
            Err(FormDataError::MissingField($field_name))
        }
    };
}

#[derive(Debug)]
struct StringField(String);

impl StringField {
    fn to_str(&self) -> &str {
        &self.0
    }
}

/// TODO TryFrom<&str> instead?, or eliminate cloning as we are nonetheless cloning quite frequently?
/// Not that this is super critical...
impl TryFrom<String> for StringField {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // TODO disallow certain characters
        Ok(Self(s.split_whitespace().collect::<Vec<_>>().join(" ")))
    }
}

impl Display for StringField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
struct PhoneField(String);

impl PhoneField {
    fn to_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for PhoneField {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut characters = s
            .split(|c| c == ' ' || c == '-')
            .map(|substring| substring.chars())
            .flatten();

        match characters.next() {
            Some('+') => (),
            None => {
                return Err("Phone number must not be empty");
            }
            _ => {
                return Err("Phone number must start with '+'");
            }
        }

        let mut parsed = String::from("+");

        for c in characters {
            if c.is_digit(10) {
                parsed.push(c);
            } else {
                return Err("Phone number must only contain digits preceded by a '+'");
            }
        }

        // since parsed only contains '+' and '0'..='9', len() in bytes is equal to number of characters
        // TODO look into bounds and additional phone number checks
        if parsed.len() < 4 {
            Err("Phone number too short")
        } else if parsed.len() > 15 {
            Err("Phone number too long")
        } else {
            Ok(PhoneField(parsed))
        }
    }
}

impl Display for PhoneField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[derive(Debug)]
struct EmailField(String);

impl EmailField {
    fn to_str(&self) -> &str {
        &self.0
    }
}

impl TryFrom<String> for EmailField {
    type Error = &'static str;

    fn try_from(s: String) -> Result<Self, Self::Error> {
        // TODO validate email address
        if s.contains('@') {
            Ok(Self(s.split_whitespace().collect()))
        } else {
            Err("Email address must contain '@'")
        }
    }
}

impl Display for EmailField {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

#[allow(dead_code)]
#[derive(Debug)]
struct EmergencyContact {
    first_name: StringField,
    last_name: StringField,
    phone: PhoneField,
}

impl TryFrom<&FormData> for EmergencyContact {
    type Error = FormDataError;

    fn try_from(form: &FormData) -> Result<Self, Self::Error> {
        let first_name = parse_form_field!(form, StringField, "emergency_contact_first_name")?;
        let last_name = parse_form_field!(form, StringField, "emergency_contact_last_name")?;
        let phone = parse_form_field!(form, PhoneField, "emergency_contact_phone")?;
        Ok(Self {
            first_name,
            last_name,
            phone,
        })
    }
}

#[derive(Debug)]
#[allow(dead_code)]
pub struct Participant {
    first_name: StringField,
    last_name: StringField,
    alias: Option<StringField>,
    email: EmailField,
    phone: PhoneField,
    country: StringField,
    emergency_contact: EmergencyContact,
}

impl Participant {
    pub fn get_first_name(&self) -> &str {
        self.first_name.to_str()
    }
    pub fn get_last_name(&self) -> &str {
        self.last_name.to_str()
    }
    pub fn get_alias(&self) -> Option<&str> {
        match &self.alias {
            None => None,
            Some(a) => Some(a.to_str()),
        }
    }
    pub fn get_email(&self) -> &str {
        self.email.to_str()
    }
    pub fn get_phone(&self) -> &str {
        self.phone.to_str()
    }
    pub fn get_country(&self) -> &str {
        self.country.to_str()
    }
    pub fn get_emergency_contact_first_name(&self) -> &str {
        self.emergency_contact.first_name.to_str()
    }
    pub fn get_emergency_contact_last_name(&self) -> &str {
        self.emergency_contact.last_name.to_str()
    }
    pub fn get_emergency_contact_phone(&self) -> &str {
        self.emergency_contact.phone.to_str()
    }
}

impl TryFrom<&FormData> for Participant {
    type Error = FormDataError;

    fn try_from(form: &FormData) -> Result<Self, Self::Error> {
        let first_name = parse_form_field!(form, StringField, "first_name")?;
        let last_name = parse_form_field!(form, StringField, "last_name")?;
        let email = parse_form_field!(form, EmailField, "email")?;
        let phone = parse_form_field!(form, PhoneField, "phone")?;
        let country = parse_form_field!(form, StringField, "country")?;

        let alias = match parse_form_field!(form, StringField, "alias") {
            Ok(alias) => Some(alias),
            Err(FormDataError::MissingField("alias")) => None,
            Err(e) => return Err(e),
        };

        let emergency_contact = EmergencyContact::try_from(form)?;

        Ok(Self {
            first_name,
            last_name,
            alias,
            email,
            phone,
            country,
            emergency_contact,
        })
    }
}

#[cfg(test)]
/// Some of these tests must be run using a wasm test runner
/// Run all tests:
/// cargo test -p shared && wasm-pack test --headless --chrome
mod tests {
    use super::*;
    use std::assert_matches;
    use wasm_bindgen_test::wasm_bindgen_test;

    // AI suggested using run_in_browser, though other options exist, e.g. npm?
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);

    #[test]
    fn test_field_parsing() -> Result<(), &'static str> {
        let passing_string_field = StringField::try_from("ECMC  2028".to_string())?;
        let passing_phone_field = PhoneField::try_from("+46 70-420 67 69".to_string())?;
        let passing_email_field = EmailField::try_from("info@ecmc2028.se".to_string())?;

        assert_eq!(&passing_string_field.to_string(), "ECMC 2028"); // note removed whitespace
        assert_eq!(&passing_phone_field.to_string(), "+46704206769"); // note removed whitespace and dash
        assert_eq!(&passing_email_field.to_string(), "info@ecmc2028.se");

        assert!(
            EmailField::try_from("infoatecmc2028.se".to_string()).is_err(),
            "Email without '@' parsed"
        );
        assert!(
            PhoneField::try_from("070-420 67 69".to_string()).is_err(),
            "Phone number without country code parsed"
        );

        Ok(())
    }

    #[wasm_bindgen_test]
    fn test_emergency_contact_parsing() {
        let good_form = FormData::new();
        good_form
            .set("emergency_contact_first_name", "John")
            .unwrap();
        good_form.set("emergency_contact_last_name", "Doe").unwrap();
        good_form
            .set("emergency_contact_phone", "+46 70 123 45 67")
            .unwrap();
        EmergencyContact::try_from(&good_form).expect("Well formed form not parsed");

        let bad_form = FormData::new();
        bad_form
            .set("emergency_contact_first_name", "John")
            .unwrap();
        bad_form.set("emergency_contact_last_name", "Doe").unwrap();
        bad_form
            .set("emergency_contact_phone", "070 123 45 67")
            .unwrap();
        assert!(
            EmergencyContact::try_from(&bad_form).is_err(),
            "Malformed emergency contact parsed"
        );
    }

    #[wasm_bindgen_test]
    fn test_participant_parsing() {
        let good_form = FormData::new();
        good_form.set("first_name", "Jade").unwrap();
        good_form.set("last_name", "Doe").unwrap();
        good_form.set("alias", "").unwrap();
        good_form.set("email", "jade@doe.com").unwrap();
        good_form.set("phone", "+46 00 000 00 00").unwrap();
        good_form.set("country", "Sweden").unwrap();
        good_form
            .set("emergency_contact_first_name", "John")
            .unwrap();
        good_form.set("emergency_contact_last_name", "Doe").unwrap();
        good_form
            .set("emergency_contact_phone", "+46 70 123 45 67")
            .unwrap();

        let good_form_parsed =
            Participant::try_from(&good_form).expect("Well formed form not parsed");
        // assert that empty alias entry -> None instead of empty alias
        assert_matches!(good_form_parsed.alias, None);

        let bad_form = FormData::new();
        bad_form.set("first_name", "Jade").unwrap();
        bad_form.set("last_name", "Doe").unwrap();
        bad_form.set("alias", "").unwrap();
        bad_form.set("email", "jade@doe.com").unwrap();
        bad_form.set("phone", "+46 00 000 00 00").unwrap();
        bad_form
            .set("emergency_contact_first_name", "John")
            .unwrap();
        bad_form.set("emergency_contact_last_name", "Doe").unwrap();
        bad_form
            .set("emergency_contact_phone", "+46 70 123 45 67")
            .unwrap();

        // missing country -> error
        assert!(Participant::try_from(&bad_form).is_err());
    }
}
