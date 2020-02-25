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

use crate::ir;

fn move_op(data_type: ir::DataType) -> ir::Opcode {
    match data_type {
        ir::DataType::Int8 =>  ir::Opcode::Move8_8,
        ir::DataType::Int16 => ir::Opcode::Move16_16,
        ir::DataType::Int32 => ir::Opcode::Move32_32,
        ir::DataType::Float => ir::Opcode::MoveF_F,
        _ => panic!("Unexpected output type"),
    }
}

fn param_is_i32(param: &ir::Parameter, y: i32) -> bool {
    if let ir::ParamType::Input(data_type) = param.param_type {
        if let ir::ParamValue::Constant(_) = param.value {
            match data_type {
                ir::DataType::Int8 => param.to_i8() == y as i8,
                ir::DataType::Int16 => param.to_i16() == y as i16,
                ir::DataType::Int32 => param.to_i32() == y,
                ir::DataType::Float => param.to_f32() == y as f32,
                _ => panic!("Data type is not a scalar"),
            }
        } else {
            false
        }
    } else {
        false
    }
}

fn param_is_zero(param: &ir::Parameter) -> bool {
    param_is_i32(param, 0)
}

fn param_is_one(param: &ir::Parameter) -> bool {
    param_is_i32(param, 1)
}

fn param_is_neg_one(param: &ir::Parameter) -> bool {
    param_is_i32(param, -1)
}

pub fn algebraic_reduce_obj(obj: &mut ir::Object) -> bool {
    let mut progress = false;

    use ir::Opcode::*;
    for instr in obj.iter_instrs_mut() {
        let old_op = instr.op;
        match instr.op {
            Add8 | Add16 | Add32 | AddF => {
                if param_is_zero(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                }
            },
            Sub8 | Sub16 | Sub32 | SubF => {
                if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                }
            },
            /* Note: Float is missing here because NaN * 0 != 0 */
            Mul8 | Mul16 | Mul32 => {
                if param_is_zero(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                } else if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_one(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_one(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                }
            },
            And8 | And16 | And32 => {
                if param_is_zero(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                } else if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_neg_one(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_neg_one(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                }
            },
            Or8 | Or16 | Or32 => {
                if param_is_zero(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                } else if param_is_neg_one(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                } else if param_is_neg_one(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                }
            },
            Xor8 | Xor16 | Xor32 => {
                if param_is_zero(&instr.params[0]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(0);
                } else if param_is_zero(&instr.params[1]) {
                    instr.op = move_op(instr.params[2].param_type.data_type());
                    instr.params.remove(1);
                }
            }
            _ => {},
        }

        if instr.op != old_op {
            progress = true;
        }
    }
    progress
}
