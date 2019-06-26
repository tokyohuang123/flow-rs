#[derive(Clone)]
pub struct Form {
    pub name: String,           // 表单名称
    pub key: String,            // 表单键
    pub fields: Vec<FormField>, // 表单字段列表
}

#[derive(Clone)]
pub struct FormField {
    pub kind: FormFieldKind, // 字段类型
    pub id: String,          // 字段id，可以类比为input name
    pub name: String,        // 字段名，可以类比为label部分
    pub value: String,       // 字段的值
}

#[derive(Clone)]
pub enum FormFieldKind {
    Text,
    Radio,
    TextArea,
    Date,
}

impl Form {
    pub fn new(name: String, key: String, fields: Vec<FormField>) -> Self {
        Form { name, key, fields }
    }
}
