use super::FieldType;

pub struct Field {
    field_type: FieldType    
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_type: FieldType::MUD
        }
    }
}
