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

use std::cmp;

use crate::ir;

fn check_live(live: &Vec<bool>, start: u32, data_type: ir::DataType) -> bool {
    let start = start as usize;
    let size = cmp::min(data_type.size() as usize, live.len() - start);
    for byte in start..(start + size) {
        if live[byte] {
            return true;
        }
    }
    false
}

fn set_live(live: &mut Vec<bool>, start: u32, data_type: ir::DataType) -> bool {
    let mut progress = false;
    let start = start as usize;
    let size = cmp::min(data_type.size() as usize, live.len() - start);
    for byte in start..(start + size) {
        if !live[byte] {
            live[byte] = true;
            progress = true;
        }
    }
    progress
}

fn instr_is_live(instr: &ir::Instruction, live: &Vec<bool>) -> bool {
    use ir::Opcode::*;

    match instr.op {
        Add8 | Add16 |Add32 | AddF => {},
        Sub8 | Sub16 |Sub32 | SubF => {},
        Mul8 | Mul16 |Mul32 | MulF => {},
        Div8 | Div16 |Div32 | DivF => {},
        Or8 | Or16 | Or32 => {},
        And8 | And16 | And32 => {},
        Xor8 | Xor16 | Xor32 => {},
        Rl8 | Rl16 | Rl32 => {},
        Move8_8 | Move8_16 | Move8_32 | Move8_F => {},
        Move16_8 | Move16_16 | Move16_32 | Move16_F => {},
        Move32_8 | Move32_16 | Move32_32 | Move32_F => {},
        MoveF_8 | MoveF_16 | MoveF_32 | MoveF_F => {},
        CpLt8 | CpLt16 | CpLt32 | CpLtF => {},
        CpGt8 | CpGt16 | CpGt32 | CpGtF => {},
        CpEq8 | CpEq16 | CpEq32 | CpEqF => {},
        CpNeq8 | CpNeq16 | CpNeq32 | CpNeqF => {},
        CpLteq8 | CpLteq16 | CpLteq32 | CpLteqF => {},
        CpGteq8 | CpGteq16 | CpGteq32 | CpGteqF => {},
        Select8 | Select16 | Select32 | SelectF => {},
        /* Anything not in the above whitelist may have side-effects so we
         * should consider it live.
         */
        _ => return true,
    }

    for param in instr.params.iter() {
        if let ir::ParamType::Output(data_type) = param.param_type {
            if let ir::ParamValue::Local(start) = param.value {
                if check_live(live, start, data_type) {
                    return true;
                }
            } else {
                /* If it outputs to anything other than local, consider it
                 * to be live.
                 */
                return true;
            }
        }
    }

    false
}

fn instr_set_live(instr: &ir::Instruction, live: &mut Vec<bool>) -> bool {
    let mut progress = false;
    for param in instr.params.iter() {
        if let ir::ParamValue::Local(start) = param.value {
            if set_live(live, start, param.param_type.data_type()) {
                progress = true;
            }
        }
    }
    progress
}

pub fn dead_code_obj(obj: &mut ir::Object) -> bool {
    let mut live = vec![];
    live.resize(obj.local_bytes as usize, false);

    let mut param_start = 0;
    for param_type in obj.params.iter() {
        if let ir::ParamType::Output(data_type) = param_type {
            set_live(&mut live, param_start, *data_type);
        }
        param_start += param_type.data_type().size();
    }
    debug_assert!(param_start <= obj.local_bytes);

    loop {
        let mut progress = false;

        for instr in obj.iter_instrs().rev() {
            if instr_is_live(instr, &live) {
                if instr_set_live(instr, &mut live) {
                    progress = true;
                }
            }
        }

        if !progress {
            break;
        }
    }

    let mut progress = false;
    for instr in obj.iter_instrs_mut() {
        if !instr_is_live(instr, &live) {
            instr.op = ir::Opcode::Nop;
            instr.params.clear();
            progress = true;
        }
    }
    progress
}

pub fn remove_nops_obj(obj: &mut ir::Object) -> bool {
    let mut progress = false;

    for block in obj.blocks.iter_mut() {
        let mut keep_instrs = vec![];
        for instr in block.instrs.drain(..) {
            if let ir::Opcode::Nop = instr.op {
                progress = true;
            } else {
                keep_instrs.push(instr);
            }
        }
        block.instrs = keep_instrs;
    }

    let mut keep_instrs = vec![];
    for instr in obj.instrs.drain(..) {
        if let ir::Opcode::Nop = instr.op {
            progress = true;
        } else {
            keep_instrs.push(instr);
        }
    }
    obj.instrs = keep_instrs;

    progress
}
