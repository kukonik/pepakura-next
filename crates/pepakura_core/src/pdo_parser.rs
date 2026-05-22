use std::fmt;
use std::io::{self, Read, Cursor};

use byteorder::{LittleEndian, ReadBytesExt};
use flate2::read::ZlibDecoder;

/// Ошибки парсинга PDO
#[derive(Debug)]
pub enum PdoParseError {
    Io(io::Error),
    Format(String),
    Compression(String),
}

impl fmt::Display for PdoParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PdoParseError::Io(e) => write!(f, "IO error: {}", e),
            PdoParseError::Format(msg) => write!(f, "Format error: {}", msg),
            PdoParseError::Compression(msg) => write!(f, "Compression error: {}", msg),
        }
    }
}

impl std::error::Error for PdoParseError {}

impl From<io::Error> for PdoParseError {
    fn from(err: io::Error) -> Self {
        PdoParseError::Io(err)
    }
}

/// Описание вершины
#[derive(Debug, Clone)]
pub struct PdoVertex {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

/// Описание полигона
#[derive(Debug, Clone)]
pub struct PdoFace {
    pub indices: Vec<u32>,
    pub material_id: u16,
}

/// Описание текстуры (минимальный скелет)
#[derive(Debug, Clone)]
pub struct PdoTexture {
    pub id: u32,
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
    pub name: String,
}

/// Модель PDO целиком
#[derive(Debug, Clone)]
pub struct PdoModel {
    pub header: String,
    pub vertices: Vec<PdoVertex>,
    pub faces: Vec<PdoFace>,
    pub textures: Vec<PdoTexture>,
}

impl PdoModel {
    /// Основной вход: парсинг из массива байт
    pub fn parse_from_bytes(data: &[u8]) -> Result<PdoModel, PdoParseError> {
        let mut cursor = Cursor::new(data);

        // Header — C-строка до нуля, максимум 256 байт (допущение)
        let header = read_c_string(&mut cursor, 256)?;

        // Количество вершин
        let vertex_count = cursor.read_u32::<LittleEndian>()?;

        let mut vertices = Vec::with_capacity(vertex_count as usize);
        for _ in 0..vertex_count {
            let x = cursor.read_f32::<LittleEndian>()?;
            let y = cursor.read_f32::<LittleEndian>()?;
            let z = cursor.read_f32::<LittleEndian>()?;
            vertices.push(PdoVertex { x, y, z });
        }

        // Количество полигонов
        let face_count = cursor.read_u32::<LittleEndian>()?;
        let mut faces = Vec::with_capacity(face_count as usize);
        for _ in 0..face_count {
            let indices_count = cursor.read_u16::<LittleEndian>()?;
            let mut indices = Vec::with_capacity(indices_count as usize);
            for _ in 0..indices_count {
                let idx = cursor.read_u32::<LittleEndian>()?;
                indices.push(idx);
            }
            let material_id = cursor.read_u16::<LittleEndian>()?;
            faces.push(PdoFace { indices, material_id });
        }

        // Количество текстур
        let texture_count = cursor.read_u32::<LittleEndian>()?;
        let mut textures = Vec::with_capacity(texture_count as usize);
        for _ in 0..texture_count {
            let id = cursor.read_u32::<LittleEndian>()?;
            let width = cursor.read_u32::<LittleEndian>()?;
            let height = cursor.read_u32::<LittleEndian>()?;

            // Допущение: несжатый RGBA, width * height * 4 байта
            let byte_len = (width * height * 4) as usize;
            let mut buf = vec![0u8; byte_len];
            cursor.read_exact(&mut buf)?;

            // Читаем имя текстуры (C-строка)
            let name = read_c_string(&mut cursor, 256)?;

            textures.push(PdoTexture {
                id,
                width,
                height,
                data: buf,
                name,
            });
        }

        Ok(PdoModel {
            header,
            vertices,
            faces,
            textures,
        })
    }
}

/// Опциональная распаковка zlib-блока (пока не используется)
pub fn maybe_decompress<R: Read>(reader: R, compressed: bool) -> Result<Vec<u8>, PdoParseError> {
    if !compressed {
        let mut buf = Vec::new();
        let mut r = reader;
        r.read_to_end(&mut buf)?;
        return Ok(buf);
    }

    let mut decoder = ZlibDecoder::new(reader);
    let mut buf = Vec::new();
    decoder
        .read_to_end(&mut buf)
        .map_err(|e| PdoParseError::Compression(e.to_string()))?;
    Ok(buf)
}

/// Чтение C-строки (до нулевого байта или max_len)
fn read_c_string<R: Read>(reader: &mut R, max_len: usize) -> Result<String, PdoParseError> {
    let mut buf = Vec::with_capacity(max_len);
    for _ in 0..max_len {
        let mut byte = [0u8; 1];
        if let Err(e) = reader.read_exact(&mut byte) {
            if e.kind() == io::ErrorKind::UnexpectedEof {
                break;
            } else {
                return Err(PdoParseError::Io(e));
            }
        }
        if byte[0] == 0 {
            break;
        }
        buf.push(byte[0]);
    }
    String::from_utf8(buf)
        .map_err(|e| PdoParseError::Format(format!("invalid UTF-8 in header: {}", e)))
}
