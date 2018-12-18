use uuid::Uuid;

pub struct Form {
    pub name: String,           // 表单名称
    pub id: String,             // 表单实例id，运行时后才有
    pub key: String,            // 表单键
    pub fields: Vec<FormField>, // 表单字段列表
}

pub struct FormField {
    pub kind: FormFieldKind, // 字段类型
    pub id: String,          // 字段id，可以类比为input name
    pub name: String,        // 字段名，可以类比为label部分
    pub value: String,       // 字段的值
    pub required: bool,      // 是否必填
    pub readonly: bool,      // 是否只读
}

pub enum FormFieldKind {
    Text,
    Radio,
    TextArea,
    Date,
}

impl Form {
    pub fn new(name: String, key: String, fields: Vec<FormField>) -> Self {
        Form {
            name,
            key,
            id: Uuid::new_v4().to_string(),
            fields,
        }
    }
}
