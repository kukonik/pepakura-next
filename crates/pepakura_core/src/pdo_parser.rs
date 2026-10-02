use std::io::{Cursor, Read};

/// Буфер для чтения бинарных данных формата .pdo
pub struct PdoReader {
    cursor: Cursor<Vec<u8>>,
    pub key: Option<u32>,
    pub codepage: Option<String>,
}

impl PdoReader {
    pub fn new(data: Vec<u8>) -> Self {
        Self {
            cursor: Cursor::new(data),
            key: None,
            codepage: None,
        }
    }

    pub fn read_bytes(&mut self, size: usize) -> Result<Vec<u8>, String> {
        let mut buf = vec![0u8; size];
        self.cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(buf)
    }

    pub fn read_u32(&mut self) -> Result<u32, String> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(u32::from_le_bytes(buf))
    }

    pub fn read_i32(&mut self) -> Result<i32, String> {
        let mut buf = [0u8; 4];
        self.cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(i32::from_le_bytes(buf))
    }

    pub fn read_f64(&mut self) -> Result<f64, String> {
        let mut buf = [0u8; 8];
        self.cursor.read_exact(&mut buf).map_err(|e| e.to_string())?;
        Ok(f64::from_le_bytes(buf))
    }

    pub fn read_bool(&mut self) -> Result<bool, String> {
        Ok(self.read_u32()? != 0)
    }

    /// Чтение строки, возможно зашифрованной ключом (XOR)
    pub fn read_wstr(&mut self) -> Result<String, String> {
        let len = self.read_u32()? as usize;
        let bytes = self.read_bytes(len)?;
        
        let decoded_bytes = if let Some(key) = self.key {
            bytes.iter().map(|&b| ((b as i32 - key as i32 + 256) % 256) as u8).collect()
        } else {
            bytes
        };

        // Обрезаем по null-терминатору
        let actual_str_bytes: Vec<u8> = decoded_bytes.iter().take_while(|&&b| b != 0).cloned().collect();
        Ok(String::from_utf8_lossy(&actual_str_bytes).to_string())
    }

    pub fn read_str(&mut self, size: usize) -> Result<String, String> {
        let bytes = self.read_bytes(size)?;
        let actual_str_bytes: Vec<u8> = bytes.iter().take_while(|&&b| b != 0).cloned().collect();
        Ok(String::from_utf8_lossy(&actual_str_bytes).to_string())
    }
}

/// Главная функция парсинга .pdo
pub fn parse_pdo(data: Vec<u8>) -> Result<(), String> {
    let mut buf = PdoReader::new(data);
    
    let version = buf.read_str(10)?;
    if version != "version 3\n" {
        return Err("Неподдерживаемая версия .pdo (ожидается version 3)".to_string());
    }

    let locked = buf.read_u32()?;
    let _unk1 = buf.read_u32()?;
    let _version_num = buf.read_u32()?;

    if locked == 5 {
        let _software = buf.read_wstr()?;
        buf.key = Some(buf.read_u32()?);
    }

    let _locale = buf.read_wstr()?;
    buf.codepage = Some(buf.read_wstr()?);
    let _unk3 = buf.read_u32()?;
    let _hexstring = buf.read_wstr()?;

    if locked == 5 {
        let _unk4 = buf.read_bool()?;
        let _unk5 = buf.read_bool()?;
    }

    // unk6 (4 doubles)
    for _ in 0..4 { let _ = buf.read_f64()?; }

    // Geometry block
    let geom_count = buf.read_u32()?;
    for _ in 0..geom_count {
        let _name = buf.read_wstr()?;
        let _unk8 = buf.read_bool()?;
        
        // 3D Vertices
        let vert_count = buf.read_u32()?;
        for _ in 0..vert_count {
            let _x = buf.read_f64()?;
            let _y = buf.read_f64()?;
            let _z = buf.read_f64()?;
        }

        // 2D Shapes (Pieces)
        let shape_count = buf.read_u32()?;
        for _ in 0..shape_count {
            let _unk11 = buf.read_i32()?;
            let _part = buf.read_u32()?;
            // unk12 (4 doubles)
            for _ in 0..4 { let _ = buf.read_f64()?; }

            let point_count = buf.read_u32()?;
            for _ in 0..point_count {
                let _index = buf.read_u32()?;
                let _coord_x = buf.read_f64()?;
                let _coord_y = buf.read_f64()?;
                // unk13 (2 doubles)
                for _ in 0..2 { let _ = buf.read_f64()?; }
                let _unk14 = buf.read_bool()?;
                // unk15 (3 doubles)
                for _ in 0..3 { let _ = buf.read_f64()?; }
                // unk16 (3 uint32 + 3 floats)
                for _ in 0..3 { let _ = buf.read_u32()?; }
                // В Python это float, пропускаем
                for _ in 0..3 { let _ = buf.read_f64()?; } 
            }
        }

        // Edges (unk17)
        let edge_count = buf.read_u32()?;
        for _ in 0..edge_count {
            // 4 uint32 + 2 bool + 1 uint32
            for _ in 0..4 { let _ = buf.read_u32()?; }
            for _ in 0..2 { let _ = buf.read_bool()?; }
            let _ = buf.read_u32()?;
        }
    }

    Ok(())
}
