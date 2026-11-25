use crate::form::Participant;
use wasm_bindgen::JsValue;
use worker::{self, D1Database, D1PreparedStatement};

impl Participant {
    pub fn get_insert_query(&self, db: &D1Database) -> worker::Result<D1PreparedStatement> {
        // participant_id and created_at are set automatically
        let query = "INSERT INTO Participants (first_name, last_name, alias, email, phone, country, emergency_contact_first_name, emergency_contact_last_name, emergency_contact_phone) VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)";
        let values: [JsValue; 9] = [
            JsValue::from_str(self.get_first_name()),
            JsValue::from_str(self.get_last_name()),
            self.get_alias()
                .map_or(JsValue::NULL, |a| JsValue::from_str(a)),
            JsValue::from_str(self.get_email()),
            JsValue::from_str(self.get_phone()),
            JsValue::from_str(self.get_country()),
            JsValue::from_str(self.get_emergency_contact_first_name()),
            JsValue::from_str(self.get_emergency_contact_last_name()),
            JsValue::from_str(self.get_emergency_contact_phone()),
        ];
        let q = db.prepare(query);
        q.bind(&values)
    }
}
