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

use crate::ir;

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

fn sign_extend_i32(x: i32, bits: u8) -> i32 {
    let shift = 32 - bits;
    (x << shift) >> shift
}

fn truncate_u32(x: u32, bits: u8) -> u32 {
    let shift = 32 - bits;
    (x << shift) >> shift
}

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

fn read_param(r: &mut dyn io::Read) -> io::Result<ir::Parameter> {
    /* Read just the first byte to start with */
    let header = read_u8(r)?;

    if (header & PRIMPAR_LONG) != 0 {
        let enc = header & PRIMPAR_BYTES;

        if (header & PRIMPAR_VARIABLE) != 0 {
            let index = read_param_imm_u32(r, enc)?;
            if (header & PRIMPAR_GLOBAL) != 0 {
                Ok(ir::Parameter::Global(index))
            } else {
                Ok(ir::Parameter::Local(index))
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
                                return Ok(ir::Parameter::String(v));
                            }
                            v.push(buf[0])
                        }
                    },
                    _ => Ok(ir::Parameter::Constant(read_param_imm_i32(r, enc)?)),
                }
            }
        }
    } else {
        /* PRIMPAR_SHORT */
        if (header & PRIMPAR_VARIABLE) != 0 {
            /* Parameters indices aren't sign-extended */
            let index = (header & PRIMPAR_INDEX) as u32;
            if (header & PRIMPAR_GLOBAL) != 0 {
                Ok(ir::Parameter::Global(index))
            } else {
                Ok(ir::Parameter::Local(index))
            }
        } else {
            /* Short constant; sign-extend the 6-bit value */
            Ok(ir::Parameter::Constant(sign_extend_i32(header as i32, 6)))
        }
    }
}

fn write_param_imm_i32(w: &mut dyn io::Write, header: u8, val: i32) -> io::Result<()> {
    if val == sign_extend_i32(val, 6) {
        w.write(&[header | ((val as u8) & PRIMPAR_VALUE)])?;
    } else if val == sign_extend_i32(val, 8) {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_1_BYTE])?;
        w.write(&[val as u8])?;
    } else if val == sign_extend_i32(val, 16) {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_2_BYTES])?;
        w.write(&(val as i16).to_le_bytes())?;
    } else {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_4_BYTES])?;
        w.write(&val.to_le_bytes())?;
    }
    Ok(())
}

fn write_param_imm_u32(w: &mut dyn io::Write, header: u8, val: u32) -> io::Result<()> {
    if val == truncate_u32(val, 5) {
        w.write(&[header | ((val as u8) & PRIMPAR_INDEX)])?;
    } else if val == truncate_u32(val, 8) {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_1_BYTE])?;
        w.write(&[val as u8])?;
    } else if val == truncate_u32(val, 16) {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_2_BYTES])?;
        w.write(&(val as i16).to_le_bytes())?;
    } else {
        w.write(&[header | PRIMPAR_LONG | PRIMPAR_4_BYTES])?;
        w.write(&val.to_le_bytes())?;
    }
    Ok(())
}

pub fn write_param(w: &mut dyn io::Write, param: &ir::Parameter) -> io::Result<()> {
    match param {
        ir::Parameter::Local(val) => write_param_imm_u32(w, 0x40u8, *val),
        ir::Parameter::Global(val) => write_param_imm_u32(w, 0x60u8, *val),
        ir::Parameter::Constant(val) => write_param_imm_i32(w, 0x00u8, *val),
        ir::Parameter::String(val) => {
            w.write(&[PRIMPAR_LONG | PRIMPAR_STRING])?;
            w.write(val.as_slice())?;
            /* We don't store the NULL */
            w.write(&[0u8])?;
            Ok(())
        }
    }
}

fn read_instruction(r: &mut dyn io::Read, ip: u32) -> io::Result<ir::Instruction> {
    let opcode_u8 = read_u8(r)?;
    let has_subcode = ir::Opcode::u8_has_subcode(opcode_u8);
    let subcode_u8 = if has_subcode { read_u8(r)? } else { 0 };
    let opcode = match ir::Opcode::from_u8(opcode_u8, subcode_u8) {
        Ok(val) => val,
        Err(val) => return Err(io::Error::new(io::ErrorKind::Other, val)),
    };

    let mut inputs = Vec::new();
    let mut outputs = Vec::new();
    let mut is_input = true;
    for c in opcode.get_proto().chars() {
        match c {
            'b' | 'w' | 'd' | 's' | 'h' | 'o' => {
                if is_input {
                    inputs.push(read_param(r)?);
                } else {
                    outputs.push(read_param(r)?);
                }
            },
            '.' => is_input = false,
            '+' => { },
            _ => panic!("Invalid parameter prototype"),
        }
    }
    Ok(ir::Instruction {
        ip: ip,
        op: opcode,
        inputs: inputs,
        outputs: outputs,
    })
}

fn write_instruction(w: &mut dyn io::Write, instr: &ir::Instruction) -> io::Result<()> {
    w.write(&[instr.op.to_u8()]);
    if instr.op.has_subcode() {
        w.write(&[instr.op.get_subcode_as_u8()]);
    }
    for param in &instr.inputs {
        write_param(w, &param)?;
    }
    for param in &instr.outputs {
        write_param(w, &param)?;
    }
    Ok(())
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
    r.read_exact(&mut sign);
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
        objects.push(ir::Object {
            owner_id: read_le_u16(r)?,
            trigger_count: read_le_u16(r)?,
            local_bytes: read_le_u32(r)?,
            instrs: vec![],
        });
    }

    /* Read instructions */
    for obj_idx in 0..num_objects {
        let obj_start = obj_offsets[obj_idx];
        let obj = &mut objects[obj_idx];

        r.seek(io::SeekFrom::Start(obj_offsets[obj_idx] as u64))?;
        loop {
            let cur_offset = r.seek(io::SeekFrom::Current(0))?;
            if cur_offset >= u32::max_value() as u64 {
                return Err(io::Error::new(io::ErrorKind::Other,
                           "File offset too large"));
            }
            let cur_offset = cur_offset as u32;
            assert!(cur_offset >= obj_start);
            let ip = cur_offset - obj_start;

            let instr = read_instruction(r, ip)?;
            match instr.op {
                ir::Opcode::ObjectEnd => break,
                _ => {},
            }
            println!("{}", instr);
            obj.instrs.push(instr);
        }
    }

    Ok(ir::Image {
        version: image_version,
        global_bytes: image_global_bytes,
        objects: objects,
    })
}
