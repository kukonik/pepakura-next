use std::io::{self, Cursor, Read};
use byteorder::{LittleEndian, ReadBytesExt};

#[derive(Debug)]
pub struct PdoHeader {
    pub magic: u32,        // 'PDO' signature (0x4F445000)
    pub locked: u32,       // 4=unlocked, 5=locked
    pub unknown1: u32,
    pub version: u32,      // Version number
    pub key: Option<u32>,  // Decryption key if locked
}

impl PdoHeader {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let magic = reader.read_u32::<LittleEndian>()?;
        
        // Check for encryption
        let locked = reader.read_u32::<LittleEndian>()?;
        let unknown1 = reader.read_u32::<LittleEndian>()?;
        let version = reader.read_u32::<LittleEndian>()?;
        
        let key = if locked == 5 {
            // Skip encrypted fields for now
            skip_wstr(reader)?; // creator
            let key = reader.read_u32::<LittleEndian>()?;
            skip_wstr(reader)?; // locale
            skip_wstr(reader)?; // codepage
            reader.read_u32::<LittleEndian>()?; // unknown
            skip_wstr(reader)?; // hexstring
            Some(key)
        } else {
            None
        };

        Ok(PdoHeader {
            magic,
            locked,
            unknown1,
            version,
            key,
        })
    }
}

#[derive(Debug)]
pub struct PdoVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl PdoVertex {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let x = reader.read_f64::<LittleEndian>()?;
        let y = reader.read_f64::<LittleEndian>()?;
        let z = reader.read_f64::<LittleEndian>()?;

        Ok(PdoVertex { x, y, z })
    }
}

#[derive(Debug)]
pub struct PdoShapePoint {
    pub vertex_index: u32,     // Index into vertices
    pub coord_2d: [f64; 2],    // 2D coordinate on paper
    pub unknown_2d: [f64; 2],
}

impl PdoShapePoint {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let vertex_index = reader.read_u32::<LittleEndian>()?;
        let x_2d = reader.read_f64::<LittleEndian>()?;
        let y_2d = reader.read_f64::<LittleEndian>()?;
        let unknown_x = reader.read_f64::<LittleEndian>()?;
        let unknown_y = reader.read_f64::<LittleEndian>()?;

        Ok(PdoShapePoint {
            vertex_index,
            coord_2d: [x_2d, y_2d],
            unknown_2d: [unknown_x, unknown_y],
        })
    }
}

#[derive(Debug)]
pub struct PdoShape {
    pub unknown_shape: u32,
    pub part_number: u32,      // Part number in Pepakura
    pub unknown_double: [f64; 4],
    pub points: Vec<PdoShapePoint>,
    pub unknown_after_points: Vec<u8>, // Complex structure after points
    pub edge_color: [f32; 3],  // RGB color for edges
}

impl PdoShape {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let unknown_shape = reader.read_u32::<LittleEndian>()?;
        let part_number = reader.read_u32::<LittleEndian>()?;
        
        let mut unknown_double = [0.0f64; 4];
        for val in &mut unknown_double {
            *val = reader.read_f64::<LittleEndian>()?;
        }

        // Parse points
        let points_count = reader.read_u32::<LittleEndian>()?;
        let mut points = Vec::with_capacity(points_count as usize);
        for _ in 0..points_count {
            points.push(PdoShapePoint::parse(reader)?);
        }

        // Skip unknown structure after points (for now)
        let _unknown_after_points: Vec<u8> = Vec::new();
        for _ in 0..3 {
            reader.read_u32::<LittleEndian>()?; // uint32[3] unknown
        }
        
        let mut edge_color = [0.0f32; 3];
        for val in &mut edge_color {
            *val = reader.read_f32::<LittleEndian>()?;
        }

        // Skip more unknown data
        skip_array(reader, |r| {
            for _ in 0..4 {
                r.read_u32::<LittleEndian>()?; // uint32[4] unknown
            }
            Ok(())
        })?;

        Ok(PdoShape {
            unknown_shape,
            part_number,
            unknown_double,
            points,
            unknown_after_points: Vec::new(), // Placeholder
            edge_color,
        })
    }
}

#[derive(Debug)]
pub struct PdoTexture {
    pub name: String,
    pub has_image: bool,
    pub image_data: Option<Vec<u8>>, // Decompressed image
    pub width: Option<u32>,
    pub height: Option<u32>,
}

impl PdoTexture {
    pub fn parse<R: Read>(reader: &mut R) -> io::Result<Self> {
        let name = read_wstr(reader)?;
        
        // Skip 4 sets of float[4] unknown
        for _ in 0..4 {
            for _ in 0..4 {
                reader.read_f32::<LittleEndian>()?;
            }
        }

        let has_image = reader.read_u8()? != 0;
        
        let (image_data, width, height) = if has_image {
            let width = reader.read_u32::<LittleEndian>()?;
            let height = reader.read_u32::<LittleEndian>()?;
            let compressed_size = reader.read_u32::<LittleEndian>()?;
            
            let mut compressed_data = vec![0u8; compressed_size as usize];
            reader.read_exact(&mut compressed_data)?;
            
            // Decompress zlib data (basic placeholder)
            let decompressed = decompress_zlib(&compressed_data).unwrap_or_default();
            
            (Some(decompressed), Some(width), Some(height))
        } else {
            (None, None, None)
        };

        // Skip remaining fields for now
        reader.read_u8()?; // some_flag
        if reader.read_u8()? != 0 {
            reader.read_f64::<LittleEndian>()?; // unknown
        }
        reader.read_u8()?; // unknown
        for _ in 0..4 {
            reader.read_f64::<LittleEndian>()?; // double[4] unknown
        }

        Ok(PdoTexture {
            name,
            has_image,
            image_data,
            width,
            height,
        })
    }
}

#[derive(Debug)]
pub struct PdoModel {
    pub header: PdoHeader,
    pub name: String,
    pub vertices: Vec<PdoVertex>,
    pub shapes: Vec<PdoShape>,
    pub textures: Vec<PdoTexture>,
    pub geometry_unknown: u8, // Unknown bool after geometry
}

impl PdoModel {
    pub fn parse_from_bytes(data: &[u8]) -> io::Result<Self> {
        let mut cursor = Cursor::new(data);
        
        let header = PdoHeader::parse(&mut cursor)?;
        
        // Skip unknown after header if locked
        if header.locked == 5 {
            cursor.read_u8()?; // unknown bool
            cursor.read_u8()?; // unknown bool
            for _ in 0..4 {
                cursor.read_f64::<LittleEndian>()?; // double[4] unknown
            }
        }
        
        // Parse geometry section
        let name = read_wstr(&mut cursor)?;
        let geometry_unknown = cursor.read_u8()?;
        
        // Parse vertices
        let vertices_count = cursor.read_u32::<LittleEndian>()?;
        let mut vertices = Vec::with_capacity(vertices_count as usize);
        for _ in 0..vertices_count {
            vertices.push(PdoVertex::parse(&mut cursor)?);
        }
        
        // Parse shapes (unfolded parts)
        let shapes_count = cursor.read_u32::<LittleEndian>()?;
        let mut shapes = Vec::with_capacity(shapes_count as usize);
        for _ in 0..shapes_count {
            shapes.push(PdoShape::parse(&mut cursor)?);
        }
        
        // Skip unknown section
        skip_array(&mut cursor, |_| Ok(()))?; // Unknown array
        
        // Parse textures
        let textures_count = cursor.read_u32::<LittleEndian>()?;
        let mut textures = Vec::with_capacity(textures_count as usize);
        for _ in 0..textures_count {
            textures.push(PdoTexture::parse(&mut cursor)?);
        }
        
        // Skip remaining sections for now
        // (Text, Lines, more unknown data...)
        
        Ok(PdoModel {
            header,
            name,
            vertices,
            shapes,
            textures,
            geometry_unknown,
        })
    }
}

// Helper functions
fn read_wstr<R: Read>(reader: &mut R) -> io::Result<String> {
    let len = reader.read_u32::<LittleEndian>()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    String::from_utf8(buf).map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid UTF-8"))
}

fn skip_wstr<R: Read>(reader: &mut R) -> io::Result<()> {
    let len = reader.read_u32::<LittleEndian>()?;
    let mut buf = vec![0u8; len as usize];
    reader.read_exact(&mut buf)?;
    Ok(())
}

fn skip_array<R: Read, F>(reader: &mut R, skip_item: F) -> io::Result<()>
where
    F: Fn(&mut R) -> io::Result<()>,
{
    let count = reader.read_u32::<LittleEndian>()?;
    for _ in 0..count {
        skip_item(reader)?;
    }
    Ok(())
}

fn decompress_zlib(compressed_data: &[u8]) -> Result<Vec<u8>, io::Error> {
    // Placeholder implementation
    // In real code, use flate2 crate
    Ok(compressed_data.to_vec()) // Just return as-is for now
}








