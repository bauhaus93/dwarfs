use super::{ FieldType, FieldMaterial };

pub struct Field {
    field_type: FieldType,    
    field_material: FieldMaterial
}

impl Field {
    pub fn get_type(&self) -> FieldType {
        self.field_type
    }
    pub fn get_material(&self) -> FieldMaterial {
        self.field_material
    }
    pub fn set_type(&mut self, new_type: FieldType) {
        self.field_type = new_type;
    }
    pub fn set_material(&mut self, new_material: FieldMaterial) {
        self.field_material = new_material;
    }
}

impl Default for Field {
    fn default() -> Self {
        Self {
            field_type: FieldType::CUBE,
            field_material: FieldMaterial::MUD
        }
    }
}
