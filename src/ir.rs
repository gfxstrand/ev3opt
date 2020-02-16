use std::io;

pub enum Parameter {
    Local(u32),
    Global(u32),
    Constant(i32),
    String(Vec<u8>),
}

fn sign_extend_i32(x: i32, bits: u8) -> i32 {
    let shift = 32 - bits;
    (x << shift) >> shift
}

fn truncate_u32(x: u32, bits: u8) -> u32 {
    let shift = 32 - bits;
    (x << shift) >> shift
}

const PRIMPAR_LONG: u8          = 0x80;
const PRIMPAR_VARIABLE: u8      = 0x40;
const PRIMPAR_GLOBAL: u8        = 0x20;
const PRIMPAR_HANDLE: u8        = 0x10;
const PRIMPAR_ADDR: u8          = 0x08;
const PRIMPAR_INDEX: u8         = 0x1F;
const PRIMPAR_VALUE: u8         = 0x3F;
const PRIMPAR_BYTES: u8         = 0x07;
const PRIMPAR_STRING_OLD: u8    = 0;
const PRIMPAR_1_BYTE: u8        = 1;
const PRIMPAR_2_BYTES: u8       = 2;
const PRIMPAR_4_BYTES: u8       = 3;
const PRIMPAR_STRING: u8        = 4;
const PRIMPAR_LABEL: u8         = 0x20;

impl Parameter {
    fn read_i32(reader: &mut dyn io::Read, enc: u8) -> io::Result<i32> {
        match enc {
            PRIMPAR_1_BYTE => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                Ok(buf[0] as i8 as i32)
            },
            PRIMPAR_2_BYTES => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                Ok(i16::from_le_bytes(buf) as i32)
            },
            PRIMPAR_4_BYTES => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(i32::from_le_bytes(buf))
            },
            _ => Err(io::Error::new(io::ErrorKind::Other,
                     "Invalid parameter encoding")),
        }
    }

    fn read_u32(reader: &mut dyn io::Read, enc: u8) -> io::Result<u32> {
        match enc {
            PRIMPAR_1_BYTE => {
                let mut buf = [0u8; 1];
                reader.read_exact(&mut buf)?;
                Ok(buf[0] as u32)
            },
            PRIMPAR_2_BYTES => {
                let mut buf = [0u8; 2];
                reader.read_exact(&mut buf)?;
                Ok(u16::from_le_bytes(buf) as u32)
            },
            PRIMPAR_4_BYTES => {
                let mut buf = [0u8; 4];
                reader.read_exact(&mut buf)?;
                Ok(u32::from_le_bytes(buf))
            },
            _ => Err(io::Error::new(io::ErrorKind::Other,
                     "Invalid parameter encoding")),
        }
    }

    pub fn read(reader: &mut dyn io::Read) -> io::Result<Parameter> {
        /* Read just the first byte to start with */
        let mut header_buf = [0u8, 1];
        reader.read_exact(&mut header_buf)?;
        let header = header_buf[0];

        if (header & PRIMPAR_LONG) != 0 {
            let enc = header & PRIMPAR_BYTES;

            if (header & PRIMPAR_VARIABLE) != 0 {
                let index = Parameter::read_u32(reader, enc)?;
                if (header & PRIMPAR_GLOBAL) != 0 {
                    Ok(Parameter::Global(index))
                } else {
                    Ok(Parameter::Local(index))
                }
            } else {
                /* Long constant */
                if (header & PRIMPAR_LABEL) != 0 {
                    panic!("Labels currently unhandled");
                } else {
                    match enc {
                        PRIMPAR_STRING | PRIMPAR_STRING_OLD => {
                            let mut v: Vec<u8> = vec![];
                            loop {
                                let mut buf = [0u8, 1];
                                reader.read_exact(&mut buf)?;
                                if buf[0] == 0 {
                                    /* We don't store the NULL */
                                    return Ok(Parameter::String(v));
                                }
                                v.push(buf[0])
                            }
                        },
                        _ => Ok(Parameter::Constant(Parameter::read_i32(reader, enc)?))
                    }
                }
            }
        } else {
            /* PRIMPAR_SHORT */
            if (header & PRIMPAR_VARIABLE) != 0 {
                /* Parameters indices aren't sign-extended */
                let index = (header & PRIMPAR_INDEX) as u32;
                if (header & PRIMPAR_GLOBAL) != 0 {
                    Ok(Parameter::Global(index))
                } else {
                    Ok(Parameter::Local(index))
                }
            } else {
                /* Short constant; sign-extend the 6-bit value */
                Ok(Parameter::Constant(sign_extend_i32(header as i32, 6)))
            }
        }
    }

    fn write_i32_param(writer: &mut dyn io::Write, header: u8, val: i32) -> io::Result<()> {
        if val == sign_extend_i32(val, 6) {
            writer.write(&[header | ((val as u8) & PRIMPAR_VALUE)])?;
        } else if val == sign_extend_i32(val, 8) {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_1_BYTE])?;
            writer.write(&[val as u8])?;
        } else if val == sign_extend_i32(val, 16) {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_2_BYTES])?;
            writer.write(&(val as i16).to_le_bytes())?;
        } else {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_4_BYTES])?;
            writer.write(&val.to_le_bytes())?;
        }
        Ok(())
    }

    fn write_u32_param(writer: &mut dyn io::Write, header: u8, val: u32) -> io::Result<()> {
        if val == truncate_u32(val, 5) {
            writer.write(&[header | ((val as u8) & PRIMPAR_INDEX)])?;
        } else if val == truncate_u32(val, 8) {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_1_BYTE])?;
            writer.write(&[val as u8])?;
        } else if val == truncate_u32(val, 16) {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_2_BYTES])?;
            writer.write(&(val as i16).to_le_bytes())?;
        } else {
            writer.write(&[header | PRIMPAR_LONG | PRIMPAR_4_BYTES])?;
            writer.write(&val.to_le_bytes())?;
        }
        Ok(())
    }

    pub fn write(&self, writer: &mut dyn io::Write) -> io::Result<()> {
        match self {
            Parameter::Local(val) => Parameter::write_u32_param(writer, 0x40u8, *val),
            Parameter::Global(val) => Parameter::write_u32_param(writer, 0x60u8, *val),
            Parameter::Constant(val) => Parameter::write_i32_param(writer, 0x00u8, *val),
            Parameter::String(val) => {
                writer.write(&[PRIMPAR_LONG | PRIMPAR_STRING])?;
                writer.write(val.as_slice())?;
                /* We don't store the NULL */
                writer.write(&[0u8])?;
                Ok(())
            }
        }
    }
}
