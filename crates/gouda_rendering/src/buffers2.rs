#[derive(Debug)]
pub enum ShaderDataType {
    Float = 0,
    Float2,
    Float3,
    Float4,
    Int,
    Int2,
    Int3,
    Int4,
}

impl ShaderDataType {
    pub fn size(&self) -> u32 {
        match self {
            ShaderDataType::Float => 4,
            ShaderDataType::Float2 => 4 * 2,
            ShaderDataType::Float3 => 4 * 3,
            ShaderDataType::Float4 => 4 * 4,
            ShaderDataType::Int => 4,
            ShaderDataType::Int2 => 4 * 2,
            ShaderDataType::Int3 => 4 * 3,
            ShaderDataType::Int4 => 4 * 4,
        }
    }
}

#[derive(Debug)]
pub struct BufferLayout {
    pub elements: Vec<BufferElement>,
    pub stride: u32,
}

impl BufferLayout {
    pub fn new(elements: Vec<BufferElement>) -> BufferLayout {
        let mut res = BufferLayout {
            elements,
            stride: 0,
        };
        res.calculate_offsets_and_stride();
        return res;
    }

    fn calculate_offsets_and_stride(&mut self) {
        let mut offset = 0;
        self.stride = 0;
        for element in self.elements.iter_mut() {
            element.offset = offset;
            offset += element.size;
            self.stride += element.size;
        }
    }
}

#[derive(Debug)]
pub struct BufferElement {
    pub name: &'static str,
    pub data_type: ShaderDataType,
    pub offset: u32,
    pub size: u32,
    pub normalized: bool,
}

impl BufferElement {
    pub fn new_normalized(name: &'static str, data_type: ShaderDataType) -> BufferElement {
        let size = data_type.size();
        return BufferElement {
            name,
            data_type,
            offset: 0,
            size: size,
            normalized: true,
        };
    }

    pub fn new(name: &'static str, data_type: ShaderDataType) -> BufferElement {
        let size = data_type.size();
        return BufferElement {
            name,
            data_type,
            offset: 0,
            size: size,
            normalized: false,
        };
    }
}
