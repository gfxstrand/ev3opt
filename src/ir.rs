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

mod opcodes;
pub use opcodes::Opcode;

pub enum Parameter {
    Local(u32),
    Global(u32),
    Constant(i32),
    String(Vec<u8>),
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Parameter::Local(i) => write!(f, "local+{}", i),
            Parameter::Global(i) => write!(f, "global+{}", i),
            Parameter::Constant(x) => write!(f, "{}", x),
            Parameter::String(v) => {
                match std::string::String::from_utf8(v.to_vec()) {
                    Ok(s) => write!(f, "\"{}\"", s),
                    Err(_) => write!(f, "<invalid UTF8 string>"),
                }
            },
        }
    }
}

pub struct Instruction {
    pub ip: u32,
    pub op: Opcode,
    pub inputs: Vec<Parameter>,
    pub outputs: Vec<Parameter>,
}

impl fmt::Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:5}: {:20}", self.ip, self.op.to_str())?;
        for param in self.inputs.iter() {
            write!(f, " {:10}", param)?;
        }
        for param in self.outputs.iter() {
            write!(f, " {:10}", param)?;
        }
        Ok(())
    }
}

pub struct Object {
    pub owner_id: u16,
    pub trigger_count: u16,
    pub local_bytes: u32,

    pub instrs: Vec<Instruction>,
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "object {{\n")?;
        write!(f, "    owner_id: {}\n", self.owner_id)?;
        write!(f, "    trigger_count: {}\n", self.trigger_count)?;
        write!(f, "    local_bytes: {}\n", self.local_bytes)?;
        write!(f, "\n")?;
        for instr in self.instrs.iter() {
            write!(f, "    {}\n", instr)?;
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
        write!(f, "global_bytes: {}\n", self.global_bytes)?;
        for obj in self.objects.iter() {
            write!(f, "\n{}", obj)?;
        }
        Ok(())
    }
}
