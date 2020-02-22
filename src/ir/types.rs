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

use std::fmt;
use crate::ir;

#[derive(Copy, Clone)]
pub enum DataType {
    Int8,
    Int16,
    Int32,
    Float,
    Int8Array,
    Int16Array,
    Int32Array,
    FloatArray,
    String(u8), /* Strings may have a size; 0 for unknown */
    Handle,
}

impl DataType {
    pub fn is_array(&self) -> bool {
        match self {
            DataType::Int8Array => true,
            DataType::Int16Array => true,
            DataType::Int32Array => true,
            DataType::FloatArray => true,
            DataType::String(_) => true,
            _ => false,
        }
    }

    pub fn without_array(&self) -> DataType {
        match self {
            DataType::Int8Array => DataType::Int8,
            DataType::Int16Array => DataType::Int16,
            DataType::Int32Array => DataType::Int32,
            DataType::FloatArray => DataType::Float,
            DataType::String(_) => DataType::Int8,
            _ => *self,
        }
    }
}

impl fmt::Display for DataType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DataType::Int8 => write!(f, "i8"),
            DataType::Int16 => write!(f, "i16"),
            DataType::Int32 => write!(f, "i32"),
            DataType::Float => write!(f, "f"),
            DataType::Int8Array => write!(f, "i8[]"),
            DataType::Int16Array => write!(f, "i16[]"),
            DataType::Int32Array => write!(f, "i32[]"),
            DataType::FloatArray => write!(f, "f[]"),
            DataType::String(len) =>
                if *len == 0 {
                    write!(f, "str")
                } else {
                    write!(f, "str({})", len)
                },
            DataType::Handle => write!(f, "handle"),
        }
    }
}

#[derive(Copy, Clone)]
pub enum ParamType {
    Input(DataType),
    Output(DataType),
    Offset,
    IP,
    BlockID,
}

impl ParamType {
    pub fn data_type(&self) -> DataType {
        match self {
            ParamType::Input(t) => *t,
            ParamType::Output(t) => *t,
            _ => panic!("Cannot get a data type from this param type"),
        }
    }
}

impl fmt::Display for ParamType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ParamType::Input(t) => write!(f, "<{}", t),
            ParamType::Output(t) => write!(f, ">{}", t),
            ParamType::Offset => write!(f, ":offset"),
            ParamType::IP => write!(f, ":IP"),
            ParamType::BlockID => write!(f, ":block"),
        }
    }
}

pub enum ParamValue {
    Local(u32),
    Global(u32),
    Constant(i32),
    String(Vec<u8>),
}

pub struct Parameter {
    pub param_type: ParamType,
    pub value: ParamValue,
}

impl Parameter {
    pub fn to_i32(&self) -> i32 {
        match self.value {
            ParamValue::Constant(x) => x,
            _ => panic!("Not a constant"),
        }
    }

    pub fn to_u32(&self) -> u32 {
        self.to_i32() as u32
    }
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self.value {
            ParamValue::Local(i) => write!(f, "local+{}", i)?,
            ParamValue::Global(i) => write!(f, "global+{}", i)?,
            ParamValue::Constant(x) => write!(f, "{}", x)?,
            ParamValue::String(v) => {
                match std::string::String::from_utf8(v.to_vec()) {
                    Ok(s) => write!(f, "\"{}\"", s)?,
                    Err(_) => write!(f, "<invalid UTF8 string>")?,
                }
            },
        };
        write!(f, "{}", self.param_type)
    }
}

pub struct Instruction {
    pub ip: u32,
    pub op: ir::Opcode,
    pub params: Vec<Parameter>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}: {:20}", self.ip, self.op.to_str())?;
        for param in self.params.iter() {
            write!(f, " {:10}", param)?;
        }
        Ok(())
    }
}

pub struct Block {
    pub id: u32,
    pub instrs: Vec<Instruction>,
}

impl Block {
    pub fn new(id: u32) -> Block {
        Block {
            id: id,
            instrs: vec![],
        }
    }
}

impl fmt::Display for Block {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "    block {} {{\n", self.id)?;
        for instr in self.instrs.iter() {
            write!(f, "        {}\n", instr)?;
        }
        write!(f, "    }}")
    }
}

pub struct Object {
    pub owner_id: u16,
    pub trigger_count: u16,
    pub local_bytes: u32,
    pub last_ip: u32,

    pub params: Vec<ParamType>,
    pub instrs: Vec<Instruction>,
    pub blocks: Vec<Block>,
}

impl Object {
    pub fn new(owner_id: u16, trigger_count: u16, local_bytes: u32) -> Object {
        Object {
            owner_id: owner_id,
            trigger_count: trigger_count,
            local_bytes: local_bytes,

            /* These have defaults */
            last_ip: 0,
            params: vec![],
            instrs: vec![],
            blocks: vec![],
        }
    }
}

impl Object {
    pub fn is_subcall(&self) -> bool {
        self.owner_id == 0 && self.trigger_count == 1
    }
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object {{\n")?;
        write!(f, "    owner_id: {}\n", self.owner_id)?;
        write!(f, "    trigger_count: {}\n", self.trigger_count)?;
        write!(f, "    local_bytes: {}\n", self.local_bytes)?;
        if self.is_subcall() {
            write!(f, "\n    proto:")?;
            for param in self.params.iter() {
                write!(f, " {}", param)?;
            }
            write!(f, "\n")?;
        }
        write!(f, "\n")?;
        for instr in self.instrs.iter() {
            write!(f, "    {}\n", instr)?;
        }
        for block in self.blocks.iter() {
            write!(f, "{}\n", block)?;
        }
        write!(f, "}}")
    }
}

pub struct Image {
    pub version: u16,
    pub global_bytes: u32,

    pub objects: Vec<Object>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "version: {}\n", self.version)?;
        write!(f, "global_bytes: {}", self.global_bytes)?;
        for obj in self.objects.iter() {
            write!(f, "\n\n{}", obj)?;
        }
        Ok(())
    }
}
