/* Copyright © 2020, Jason Ekstrand
 *
 * Permission is hereby granted, free of charge, to any person obtaining a
 * copy of this software and associated documentation files (the "Software"),
 * to deal in the Software without restriction, including without limitation
 * the rights to use, copy, modify, merge, publish, distribute, sublicense,
 * and/or sell copies of the Software, and to permit persons to whom the
 * Software is furnished to do so, subject to the following conditions:
 *
 * The above copyright notice and this permission notice (including the next
 * paragraph) shall be included in all copies or substantial portions of the
 * Software.
 *
 * THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
 * IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
 * FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT.  IN NO EVENT SHALL
 * THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
 * LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
 * FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS
 * IN THE SOFTWARE.
 */

use std::error::Error;
use std::fs::File;
use std::io;
use std::path::Path;

use crate::util::sign_extend_i32;
use crate::util::truncate_u32;
use crate::ir;

const PRIMPAR_SHORT: u8         = 0x00;
const PRIMPAR_LONG: u8          = 0x80;
const PRIMPAR_CONST: u8         = 0x00;
const PRIMPAR_VARIABLE: u8      = 0x40;
const PRIMPAR_LOCAL: u8         = 0x00;
const PRIMPAR_GLOBAL: u8        = 0x20;
const PRIMPAR_INDEX: u8         = 0x1F;
const PRIMPAR_VALUE: u8         = 0x3F;
const PRIMPAR_BYTES: u8         = 0x07;
const PRIMPAR_STRING_OLD: u8    = 0;
const PRIMPAR_1_BYTE: u8        = 1;
const PRIMPAR_2_BYTES: u8       = 2;
const PRIMPAR_4_BYTES: u8       = 3;
const PRIMPAR_STRING: u8        = 4;
const PRIMPAR_LABEL: u8         = 0x20;

const CALLPAR_IN: u8            = 0x80;
const CALLPAR_OUT: u8           = 0x40;

const CALLPAR_TYPE: u8          = 0x07;
const CALLPAR_DATA8: u8         = 0x00;
const CALLPAR_DATA16: u8        = 0x01;
const CALLPAR_DATA32: u8        = 0x02;
const CALLPAR_DATAF: u8         = 0x03;
const CALLPAR_STRING: u8        = 0x04;

fn read_u8(r: &mut dyn io::Read) -> io::Result<u8> {
    let mut buf = [0u8; 1];
    r.read_exact(&mut buf)?;
    Ok(buf[0])
}

fn read_le_u16(r: &mut dyn io::Read) -> io::Result<u16> {
    let mut buf = [0u8; 2];
    r.read_exact(&mut buf)?;
    Ok(u16::from_le_bytes(buf))
}

fn read_le_u32(r: &mut dyn io::Read) -> io::Result<u32> {
    let mut buf = [0u8; 4];
    r.read_exact(&mut buf)?;
    Ok(u32::from_le_bytes(buf))
}

fn write_u8(w: &mut dyn io::Write, u: u8) -> io::Result<()> {
    w.write_all(&[u])
}

fn write_le_u16(w: &mut dyn io::Write, u: u16) -> io::Result<()> {
    w.write_all(&u.to_le_bytes())
}

fn write_le_u32(w: &mut dyn io::Write, u: u32) -> io::Result<()> {
    w.write_all(&u.to_le_bytes())
}

fn read_param_imm_i32(r: &mut dyn io::Read, enc: u8) -> io::Result<i32> {
    match enc {
        PRIMPAR_1_BYTE => Ok(read_u8(r)? as i8 as i32),
        PRIMPAR_2_BYTES => Ok(read_le_u16(r)? as i16 as i32),
        PRIMPAR_4_BYTES => Ok(read_le_u32(r)? as i32),
        _ => Err(io::Error::new(io::ErrorKind::Other,
                 "Invalid parameter encoding")),
    }
}

fn read_param_imm_u32(r: &mut dyn io::Read, enc: u8) -> io::Result<u32> {
    match enc {
        PRIMPAR_1_BYTE => Ok(read_u8(r)? as u32),
        PRIMPAR_2_BYTES => Ok(read_le_u16(r)? as u32),
        PRIMPAR_4_BYTES => read_le_u32(r),
        _ => Err(io::Error::new(io::ErrorKind::Other,
                 "Invalid parameter encoding")),
    }
}

fn read_param_value(r: &mut dyn io::Read) -> io::Result<ir::ParamValue> {
    /* Read just the first byte to start with */
    let header = read_u8(r)?;

    if (header & PRIMPAR_LONG) != 0 {
        let enc = header & PRIMPAR_BYTES;

        if (header & PRIMPAR_VARIABLE) != 0 {
            let index = read_param_imm_u32(r, enc)?;
            if (header & PRIMPAR_GLOBAL) != 0 {
                Ok(ir::ParamValue::Global(index))
            } else {
                Ok(ir::ParamValue::Local(index))
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
                            r.read_exact(&mut buf)?;
                            if buf[0] == 0 {
                                /* We don't store the NULL */
                                return Ok(ir::ParamValue::String(v));
                            }
                            v.push(buf[0])
                        }
                    },
                    _ => Ok(ir::ParamValue::Constant(read_param_imm_i32(r, enc)?)),
                }
            }
        }
    } else {
        /* PRIMPAR_SHORT */
        if (header & PRIMPAR_VARIABLE) != 0 {
            /* Parameters indices aren't sign-extended */
            let index = (header & PRIMPAR_INDEX) as u32;
            if (header & PRIMPAR_GLOBAL) != 0 {
                Ok(ir::ParamValue::Global(index))
            } else {
                Ok(ir::ParamValue::Local(index))
            }
        } else {
            /* Short constant; sign-extend the 6-bit value */
            Ok(ir::ParamValue::Constant(sign_extend_i32(header as i32, 6)))
        }
    }
}

fn write_param_imm_i32(w: &mut dyn io::Write, header: u8, val: i32) -> io::Result<()> {
    if val == sign_extend_i32(val, 6) {
        write_u8(w, header | PRIMPAR_SHORT | ((val as u8) & PRIMPAR_VALUE))
    } else if val == sign_extend_i32(val, 8) {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_1_BYTE)?;
        write_u8(w, val as u8)
    } else if val == sign_extend_i32(val, 16) {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_2_BYTES)?;
        write_le_u16(w, val as u16)
    } else {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_4_BYTES)?;
        write_le_u32(w, val as u32)
    }
}

fn write_param_imm_u32(w: &mut dyn io::Write, header: u8, val: u32) -> io::Result<()> {
    if val == truncate_u32(val, 5) {
        write_u8(w, header | PRIMPAR_SHORT | ((val as u8) & PRIMPAR_INDEX))
    } else if val == truncate_u32(val, 8) {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_1_BYTE)?;
        write_u8(w, val as u8)
    } else if val == truncate_u32(val, 16) {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_2_BYTES)?;
        write_le_u16(w, val as u16)
    } else {
        write_u8(w, header | PRIMPAR_LONG | PRIMPAR_4_BYTES)?;
        write_le_u32(w, val)
    }
}

pub fn write_param_value(w: &mut dyn io::Write, val: &ir::ParamValue) -> io::Result<()> {
    match val {
        ir::ParamValue::Local(i) => {
            write_param_imm_u32(w, PRIMPAR_VARIABLE | PRIMPAR_LOCAL, *i)
        }
        ir::ParamValue::Global(i) => {
            write_param_imm_u32(w, PRIMPAR_VARIABLE | PRIMPAR_GLOBAL, *i)
        }
        ir::ParamValue::Constant(x) => {
            write_param_imm_i32(w, PRIMPAR_CONST, *x)
        }
        ir::ParamValue::String(s) => {
            w.write(&[PRIMPAR_LONG | PRIMPAR_STRING])?;
            w.write(s.as_slice())?;
            /* We don't store the NULL */
            w.write(&[0u8])?;
            Ok(())
        }
    }
}

struct IPReloc {
    file_offset: u32,
    source_ip: u32,
    target_ip: u32,
}

struct OffsetReloc {
    file_offset: u32,
    value: i32,
}

struct RelocWriter {
    ip_relocs: Vec<IPReloc>,
    offset_relocs: Vec<OffsetReloc>,
    ip_map: std::collections::HashMap::<u32, (i32, i32)>,
    reloc_bits: u8,
}

impl RelocWriter {
    pub const SIZES: [u8; 4] = [6, 8, 16, 32];

    pub fn new(reloc_bits: u8) -> RelocWriter {
        RelocWriter {
            ip_relocs: vec![],
            offset_relocs: vec![],
            ip_map: std::collections::HashMap::new(),
            reloc_bits: reloc_bits,
        }
    }

    fn write_value(&self, w: &mut dyn io::Write, x: u32) -> io::Result<()> {
        match self.reloc_bits {
            6 => write_u8(w, PRIMPAR_CONST | PRIMPAR_SHORT |
                             ((x as u8) & PRIMPAR_VALUE)),
            8 => write_u8(w, x as u8),
            16 => write_le_u16(w, x as u16),
            32 => write_le_u32(w, x as u32),
            _ => panic!("Invalid reloc size"),
        }
    }

    pub fn write_param_reloc<W: io::Write + io::Seek>(&mut self, w: &mut W,
                                                      source_ip: u32,
                                                      target_ip: u32)
                                                      -> io::Result<()> {
        match self.reloc_bits {
            6 => { /* Do nothing; we'll write it at the end */ },
            8 => write_u8(w, PRIMPAR_CONST | PRIMPAR_LONG | PRIMPAR_1_BYTE)?,
            16 => write_u8(w, PRIMPAR_CONST | PRIMPAR_LONG | PRIMPAR_2_BYTES)?,
            32 => write_u8(w, PRIMPAR_CONST | PRIMPAR_LONG | PRIMPAR_4_BYTES)?,
            _ => panic!("Invalid reloc size"),
        }
        let file_offset = w.seek(io::SeekFrom::Current(0))?;
        assert!(file_offset < u32::max_value() as u64);
        self.ip_relocs.push(IPReloc {
            file_offset: file_offset as u32,
            source_ip: source_ip,
            target_ip: target_ip,
        });
        self.write_value(w, 0xdeadbeef)
    }

    pub fn register_ip(&mut self, ip: u32, start_offset: u64, end_offset:u64) {
        assert!(start_offset < i32::max_value() as u64);
        assert!(end_offset < i32::max_value() as u64);
        assert!(start_offset < end_offset);
        self.ip_map.insert(ip, (start_offset as i32, end_offset as i32));
    }

    pub fn calc_offsets(&mut self) -> bool {
        assert!(self.offset_relocs.is_empty());
        for reloc in self.ip_relocs.iter() {
            let source_offset = match self.ip_map.get(&reloc.source_ip) {
                Some((_, end)) => end,
                None => panic!("Invalid instruction offset"),
            };

            let target_offset = match self.ip_map.get(&reloc.target_ip) {
                Some((start, _)) => start,
                None => panic!("Invalid instruction offset"),
            };

            let rel_offset = *target_offset - *source_offset;
            if rel_offset != sign_extend_i32(rel_offset, self.reloc_bits) {
                return false;
            }
            self.offset_relocs.push(OffsetReloc {
                file_offset: reloc.file_offset,
                value: rel_offset,
            });
        }
        self.ip_relocs.clear();
        true
    }

    pub fn finish_relocs<W: io::Write + io::Seek>(&mut self, w: &mut W) -> io::Result<()> {
        /* Save off the current file offset as we're about to scratch around
         * to do relocations for the current object.
         */
        let obj_end = w.seek(io::SeekFrom::Current(0))?;

        assert!(self.ip_relocs.is_empty());
        /* Write all the relocations for this object */
        for reloc in self.offset_relocs.iter() {
            w.seek(io::SeekFrom::Start(reloc.file_offset as u64))?;
            self.write_value(w, reloc.value as u32)?;
        }
        self.offset_relocs.clear();

        /* Put the file back where we found it */
        w.seek(io::SeekFrom::Start(obj_end))?;
        Ok(())
    }
}

fn read_instruction(r: &mut dyn io::Read, objects: &Vec<ir::Object>, ip: u32) -> io::Result<ir::Instruction> {
    let opcode_u8 = read_u8(r)?;
    let has_subcode = ir::Opcode::u8_has_subcode(opcode_u8);
    let subcode_u8 = if has_subcode { read_u8(r)? } else { 0 };
    let opcode = match ir::Opcode::from_u8(opcode_u8, subcode_u8) {
        Ok(val) => val,
        Err(val) => return Err(io::Error::new(io::ErrorKind::Other, val)),
    };

    let mut params = Vec::new();
    match opcode {
        ir::Opcode::Call => {
            let obj_id_value = read_param_value(r)?;
            let obj_id = match obj_id_value {
                ir::ParamValue::Constant(x) => x,
                _ => return Err(io::Error::new(io::ErrorKind::Other,
                                "Object for Call instruction must be constant")),
            };
            /* Objects are 1-indexed */
            if obj_id == 0 {
                return Err(io::Error::new(io::ErrorKind::Other,
                           "Call instruction 1-index objects; 0 is invalid"));
            }
            let obj_id = obj_id - 1;
            params.push(ir::Parameter {
                param_type: ir::ParamType::Input(ir::DataType::Int16),
                value: obj_id_value,
            });

            let num_params = read_u8(r)?;
            let proto = &objects[obj_id as usize].params;
            if proto.len() != num_params as usize {
                return Err(io::Error::new(io::ErrorKind::Other,
                           "Call instruction has param count mismatch"));
            }
            for param_type in proto {
                params.push(ir::Parameter {
                    param_type: *param_type,
                    value: read_param_value(r)?,
                });
            }
        },
        _ => {
            for param_type in opcode.get_proto() {
                params.push(ir::Parameter {
                    param_type: *param_type,
                    value: read_param_value(r)?,
                });
            }
        },
    }
    Ok(ir::Instruction {
        ip: ip,
        op: opcode,
        params: params,
    })
}

fn write_instruction<W: io::Write + io::Seek>(w: &mut W,
                                              instr: &ir::Instruction,
                                              relocs: &mut RelocWriter)
                                              -> io::Result<()> {
    w.write(&[instr.op.to_u8()])?;
    if instr.op.has_subcode() {
        write_u8(w, instr.op.get_subcode_as_u8())?;
    }
    let mut first_param = true;
    for param in instr.params.iter() {
        if let ir::ParamType::IP = param.param_type {
            if let ir::ParamValue::Constant(ip) = param.value {
                assert!(ip >= 0);
                relocs.write_param_reloc(w, instr.ip, ip as u32)?;
                continue;
            } else {
                panic!("IPs must be constants");
            }
        }
        write_param_value(w, &param.value)?;
        if first_param {
            if let ir::Opcode::Call = instr.op {
                write_u8(w, (instr.params.len() - 1) as u8)?;
            }
            first_param = false;
        }
    }
    Ok(())
}

fn read_call_param_type(r: &mut dyn io::Read) -> io::Result<ir::ParamType> {
    let param = read_u8(r)?;
    let data_type = match param & CALLPAR_TYPE {
        CALLPAR_DATA8 => ir::DataType::Int8,
        CALLPAR_DATA16 => ir::DataType::Int16,
        CALLPAR_DATA32 => ir::DataType::Int32,
        CALLPAR_DATAF => ir::DataType::Float,
        CALLPAR_STRING => ir::DataType::String(read_u8(r)?),
        _ => return Err(io::Error::new(io::ErrorKind::Other,
                        "Invalid subcall parameter type"))
    };
    if (param & CALLPAR_IN) != 0 {
        Ok(ir::ParamType::Input(data_type))
    } else if (param & CALLPAR_OUT) != 0 {
        Ok(ir::ParamType::Output(data_type))
    } else {
        Err(io::Error::new(io::ErrorKind::Other,
            "Invalid subcall parameter type"))
    }
}

fn write_call_param_type(w: &mut dyn io::Write, param: &ir::ParamType) -> io::Result<()> {
    let inout = match param {
        ir::ParamType::Input(_) => CALLPAR_IN,
        ir::ParamType::Output(_) => CALLPAR_OUT,
        _ => panic!("Invalid parameter type for call instruction"),
    };
    match param.data_type() {
        ir::DataType::Int8 => write_u8(w, inout | CALLPAR_DATA8),
        ir::DataType::Int16 => write_u8(w, inout | CALLPAR_DATA16),
        ir::DataType::Int32 => write_u8(w, inout | CALLPAR_DATA32),
        ir::DataType::Float => write_u8(w, inout | CALLPAR_DATAF),
        ir::DataType::String(len) => {
            write_u8(w, inout | CALLPAR_STRING)?;
            write_u8(w, len)
        },
        _ => panic!("Invalid data type for subcall parameter"),
    }
}

pub fn read_rbf_file(path: &Path) -> io::Result<ir::Image> {
    let file = match File::open(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                   why.description()),
        Ok(file) => file,
    };

    use io::Read;
    use io::Seek;
    let mut reader = io::BufReader::new(file);
    let r = &mut reader;

    let mut sign = [0u8; 4];
    r.read_exact(&mut sign)?;
    if sign != ['L' as u8, 'E' as u8, 'G' as u8, 'O' as u8] {
        return Err(io::Error::new(io::ErrorKind::Other,
                   "Not a valid LEGO EV3 bytecode file"));
    }

    /* Read the image header */
    let _ = read_le_u32(r)?;
    let image_version = read_le_u16(r)?;
    let num_objects = read_le_u16(r)? as usize;
    let image_global_bytes = read_le_u32(r)?;

    /* Read object headers */
    let mut objects: Vec<ir::Object> = vec![];
    let mut obj_offsets: Vec<u32> = vec![];
    for _ in 0..num_objects {
        obj_offsets.push(read_le_u32(r)?);
        let owner_id = read_le_u16(r)?;
        let trigger_count = read_le_u16(r)?;
        let local_bytes = read_le_u32(r)?;
        objects.push(ir::Object::new(owner_id, trigger_count, local_bytes));
    }

    /* Read object prototypes for subroutines */
    for obj_idx in 0..num_objects {
        let obj = &mut objects[obj_idx];
        if obj.is_subcall() {
            r.seek(io::SeekFrom::Start(obj_offsets[obj_idx] as u64))?;
            let num_params = read_u8(r)?;
            for _ in 0..num_params {
                obj.params.push(read_call_param_type(r)?);
            }
            obj_offsets[obj_idx] = r.seek(io::SeekFrom::Current(0))? as u32;
        }
    }

    /* Read instructions */
    for obj_idx in 0..num_objects {
        let obj_start = obj_offsets[obj_idx];
        let mut rel_offset = 0i32;
        r.seek(io::SeekFrom::Start(obj_start as u64))?;
        loop {
            /* We give each new instruction rel_offset as its ip */
            let mut instr = read_instruction(r, &objects, rel_offset as u32)?;

            /* Compute the new rel_offset after reading the instruction */
            let abs_offset = r.seek(io::SeekFrom::Current(0))?;
            assert!(abs_offset >= obj_start as u64);
            if abs_offset >= i32::max_value() as u64 {
                return Err(io::Error::new(io::ErrorKind::Other,
                           "File offset too large"));
            }
            rel_offset = abs_offset as i32 - obj_start as i32;

            /* Turn any jump offsets into IPs */
            for param in instr.params.iter_mut() {
                if let ir::ParamType::Offset = param.param_type {
                    if let ir::ParamValue::Constant(offset) = param.value {
                        let target = rel_offset + offset;
                        param.param_type = ir::ParamType::IP;
                        param.value = ir::ParamValue::Constant(target);
                    } else {
                        return Err(io::Error::new(io::ErrorKind::Other,
                                   "Jump offsets must be constants"));
                    }
                }
            }

            let end = match instr.op {
                ir::Opcode::ObjectEnd => true,
                _ => false,
            };

            objects[obj_idx].instrs.push(instr);

            if end {
                objects[obj_idx].last_ip = rel_offset as u32;
                break;
            }
        }
    }

    Ok(ir::Image {
        version: image_version,
        global_bytes: image_global_bytes,
        objects: objects,
    })
}

pub fn write_rbf_file(path: &Path, image: &ir::Image) -> io::Result<()> {
    let file = match File::create(&path) {
        Err(why) => panic!("couldn't open {}: {}", path.display(),
                                                   why.description()),
        Ok(file) => file,
    };

    use io::Write;
    use io::Seek;
    let mut writer = io::BufWriter::new(file);
    let w = &mut writer;

    /* Skip past the image and object headers; we'll write them later */
    w.seek(io::SeekFrom::Start(16 + (image.objects.len() as u64) * 12))?;

    let mut obj_offsets: Vec<u32> = vec![];
    for obj in image.objects.iter() {
        obj_offsets.push(w.seek(io::SeekFrom::Current(0))? as u32);

        if obj.is_subcall() {
            write_u8(w, obj.params.len() as u8)?;
            for param in obj.params.iter() {
                write_call_param_type(w, param)?;
            }
        }

        /* We don't know how bit the offsets need to be until we've written out
         * the instructions and we can't write the instructions without knowing
         * the sizes of our offsets.  To solve this chicken-and-egg problem, we
         * start at the smallest size and, if that fails, try to write out the
         * object again with a bigger size.
         *
         * Stash this in case we need to re-try this object.
         */
        let instrs_start = w.seek(io::SeekFrom::Current(0))?;
        for reloc_size in RelocWriter::SIZES.iter() {
            w.seek(io::SeekFrom::Start(instrs_start))?;

            let mut relocs = RelocWriter::new(*reloc_size);

            let mut start_offset = w.seek(io::SeekFrom::Current(0))?;
            for instr in obj.instrs.iter() {
                write_instruction(w, instr, &mut relocs)?;
                let end_offset = w.seek(io::SeekFrom::Current(0))?;
                relocs.register_ip(instr.ip, start_offset, end_offset);
                start_offset = end_offset;
            }

            if relocs.calc_offsets() {
                relocs.finish_relocs(w)?;
                break;
            }
            assert!(*reloc_size < 32);
        }
    }
    let image_size = w.seek(io::SeekFrom::Current(0))? as u32;

    /* Now we can write the image and object headers */
    w.seek(io::SeekFrom::Start(0))?;

    w.write_all(&['L' as u8, 'E' as u8, 'G' as u8, 'O' as u8])?;
    write_le_u32(w, image_size)?;
    write_le_u16(w, image.version)?;
    write_le_u16(w, image.objects.len() as u16)?;
    write_le_u32(w, image.global_bytes)?;

    for obj_idx in 0..image.objects.len() {
        write_le_u32(w, obj_offsets[obj_idx])?;
        write_le_u16(w, image.objects[obj_idx].owner_id)?;
        write_le_u16(w, image.objects[obj_idx].trigger_count)?;
        write_le_u32(w, image.objects[obj_idx].local_bytes)?;
    }

    Ok(())
}
