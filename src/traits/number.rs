// 拓展数字功能

pub struct SqlEnum(pub u32);

impl Into<Vec<u8>> for SqlEnum {
    fn into(self) -> Vec<u8> {
        self.0.to_le_bytes().to_vec()
    }
}

impl From<u32> for SqlEnum {
    fn from(value: u32) -> Self {
        Self(value)
    }
}
