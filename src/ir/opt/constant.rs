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

#[inline]
fn f32_as_i32(f: f32) -> i32 {
    i32::from_le_bytes(f.to_le_bytes())
}

fn has_constant_inputs(instr: &ir::Instruction) -> bool {
    for param in instr.params.iter() {
        if let ir::ParamType::Input(_) = param.param_type {
            match param.value {
                ir::ParamValue::Constant(_) => {},
                _ => return false,
            }
        }
    }
    true
}

pub fn constant_fold_obj(obj: &mut ir::Object) -> bool {
    let mut progress = false;

    use ir::Opcode::*;
    use ir::opcodes::MathSubcode::*;
    for instr in obj.iter_instrs_mut() {
        if !has_constant_inputs(instr) {
            continue;
        }

        let p = &instr.params;
        let val = match instr.op {
            /* Binary Arithmetic */
            Add8 =>           (p[0].to_i8()  +  p[1].to_i8()) as i32,
            Add16 =>          (p[0].to_i16() +  p[1].to_i16()) as i32,
            Add32 =>           p[0].to_i32() +  p[1].to_i32(),
            AddF => f32_as_i32(p[0].to_f32() +  p[1].to_f32()),
            Sub8 =>           (p[0].to_i8()  -  p[1].to_i8()) as i32,
            Sub16 =>          (p[0].to_i16() -  p[1].to_i16()) as i32,
            Sub32 =>           p[0].to_i32() -  p[1].to_i32(),
            SubF => f32_as_i32(p[0].to_f32() -  p[1].to_f32()),
            Mul8 =>           (p[0].to_i8()  *  p[1].to_i8()) as i32,
            Mul16 =>          (p[0].to_i16() *  p[1].to_i16()) as i32,
            Mul32 =>           p[0].to_i32() *  p[1].to_i32(),
            MulF => f32_as_i32(p[0].to_f32() *  p[1].to_f32()),
            Div8 =>           (p[0].to_i8()  /  p[1].to_i8()) as i32,
            Div16 =>          (p[0].to_i16() /  p[1].to_i16()) as i32,
            Div32 =>           p[0].to_i32() /  p[1].to_i32(),
            DivF => f32_as_i32(p[0].to_f32() /  p[1].to_f32()),
            Or8 =>            (p[0].to_i8()  |  p[1].to_i8()) as i32,
            Or16 =>           (p[0].to_i16() |  p[1].to_i16()) as i32,
            Or32 =>            p[0].to_i32() |  p[1].to_i32(),
            And8 =>           (p[0].to_i8()  &  p[1].to_i8()) as i32,
            And16 =>          (p[0].to_i16() &  p[1].to_i16()) as i32,
            And32 =>           p[0].to_i32() &  p[1].to_i32(),
            Xor8 =>           (p[0].to_i8()  ^  p[1].to_i8()) as i32,
            Xor16 =>          (p[0].to_i16() ^  p[1].to_i16()) as i32,
            Xor32 =>           p[0].to_i32() ^  p[1].to_i32(),
            Rl8 =>            (p[0].to_i8()  << p[1].to_i8()) as i32,
            Rl16 =>           (p[0].to_i16() << p[1].to_i16()) as i32,
            Rl32 =>            p[0].to_i32() << p[1].to_i32(),

            /* Move (and data convert) instructions
             *
             * Note: We don't handle any non-converting Move instructions here
             * because that can lead to infinite optimization loops.
             */
            Move8_16 =>            p[0].to_i8() as i32,
            Move8_32 =>            p[0].to_i8() as i32,
            Move8_F =>  f32_as_i32(p[0].to_i8() as f32),
            Move16_8 =>            p[0].to_i16() as i8 as i32,
            Move16_32 =>           p[0].to_i16() as i32,
            Move16_F => f32_as_i32(p[0].to_i16() as f32),
            Move32_8 =>            p[0].to_i32() as i8 as i32,
            Move32_16 =>           p[0].to_i32() as i16 as i32,
            Move32_F => f32_as_i32(p[0].to_i32() as f32),
            MoveF_8 =>             p[0].to_f32() as i8 as i32,
            MoveF_16 =>            p[0].to_f32() as i16 as i32,
            MoveF_32 =>            p[0].to_f32() as i32,

            /* Jump instructions.  0 => Nop, else => Jr */
            JrFalse => (!p[0].to_bool()) as i32,
            JrTrue => p[0].to_bool() as i32,
            JrNan => p[0].to_f32().is_nan() as i32,

            /* Comparison and jump comparison instructions */
            CpLt8    | JrLt8 =>    (p[0].to_i8()  <  p[1].to_i8()) as i32,
            CpLt16   | JrLt16 =>   (p[0].to_i16() <  p[1].to_i16()) as i32,
            CpLt32   | JrLt32 =>   (p[0].to_i32() <  p[1].to_i32()) as i32,
            CpLtF    | JrLtF =>    (p[0].to_f32() <  p[1].to_f32()) as i32,
            CpGt8    | JrGt8 =>    (p[0].to_i8()  >  p[1].to_i8()) as i32,
            CpGt16   | JrGt16 =>   (p[0].to_i16() >  p[1].to_i16()) as i32,
            CpGt32   | JrGt32 =>   (p[0].to_i32() >  p[1].to_i32()) as i32,
            CpGtF    | JrGtF =>    (p[0].to_f32() >  p[1].to_f32()) as i32,
            CpEq8    | JrEq8 =>    (p[0].to_i8()  == p[1].to_i8()) as i32,
            CpEq16   | JrEq16 =>   (p[0].to_i16() == p[1].to_i16()) as i32,
            CpEq32   | JrEq32 =>   (p[0].to_i32() == p[1].to_i32()) as i32,
            CpEqF    | JrEqF =>    (p[0].to_f32() == p[1].to_f32()) as i32,
            CpNeq8   | JrNeq8 =>   (p[0].to_i8()  != p[1].to_i8()) as i32,
            CpNeq16  | JrNeq16 =>  (p[0].to_i16() != p[1].to_i16()) as i32,
            CpNeq32  | JrNeq32 =>  (p[0].to_i32() != p[1].to_i32()) as i32,
            CpNeqF   | JrNeqF =>   (p[0].to_f32() != p[1].to_f32()) as i32,
            CpLteq8  | JrLteq8 =>  (p[0].to_i8()  <= p[1].to_i8()) as i32,
            CpLteq16 | JrLteq16 => (p[0].to_i16() <= p[1].to_i16()) as i32,
            CpLteq32 | JrLteq32 => (p[0].to_i32() <= p[1].to_i32()) as i32,
            CpLteqF  | JrLteqF =>  (p[0].to_f32() <= p[1].to_f32()) as i32,
            CpGteq8  | JrGteq8 =>  (p[0].to_i8()  >= p[1].to_i8()) as i32,
            CpGteq16 | JrGteq16 => (p[0].to_i16() >= p[1].to_i16()) as i32,
            CpGteq32 | JrGteq32 => (p[0].to_i32() >= p[1].to_i32()) as i32,
            CpGteqF  | JrGteqF =>  (p[0].to_f32() >= p[1].to_f32()) as i32,

            /* Select */
            Select8 =>  (if p[0].to_bool() { p[1].to_i8()  } else { p[2].to_i8()  }) as i32,
            Select16 => (if p[0].to_bool() { p[1].to_i16() } else { p[2].to_i16() }) as i32,
            Select32 => (if p[0].to_bool() { p[1].to_i32() } else { p[2].to_i32() }) as i32,
            SelectF =>  (if p[0].to_bool() { p[1].to_i32() } else { p[2].to_i32() }) as i32,

            Math(subcode) => match subcode {
                EXP =>    f32_as_i32(p[0].to_f32().exp()),
                MOD =>    f32_as_i32(p[0].to_f32() % p[1].to_f32()),
                FLOOR =>  f32_as_i32(p[0].to_f32().floor()),
                CEIL =>   f32_as_i32(p[0].to_f32().ceil()),
                ROUND =>  f32_as_i32(p[0].to_f32().round()),
                ABS =>    f32_as_i32(p[0].to_f32().abs()),
                NEGATE => f32_as_i32(-p[0].to_f32()),
                SQRT =>   f32_as_i32(p[0].to_f32().sqrt()),
                LOG =>    f32_as_i32(p[0].to_f32().log10()),
                LN =>     f32_as_i32(p[0].to_f32().ln()),
                SIN =>    f32_as_i32(p[0].to_f32().sin()),
                COS =>    f32_as_i32(p[0].to_f32().cos()),
                TAN =>    f32_as_i32(p[0].to_f32().tan()),
                ASIN =>   f32_as_i32(p[0].to_f32().asin()),
                ACOS =>   f32_as_i32(p[0].to_f32().acos()),
                ATAN =>   f32_as_i32(p[0].to_f32().atan()),
                MOD8 =>   (p[0].to_i8() % p[1].to_i8()) as i32,
                MOD16 =>  (p[0].to_i16() % p[1].to_i16()) as i32,
                MOD32 =>  (p[0].to_i32() % p[1].to_i32()),
                POW =>    f32_as_i32(p[0].to_f32().powf(p[1].to_f32())),
                /* We're not going to bother folding this one */
                TRUNC => continue,
            },

            /* Default */
            _ => continue,
        };

        progress = true;

        let last_param = instr.params.pop().unwrap();
        match last_param.param_type {
            ir::ParamType::Offset |
            ir::ParamType::IP |
            ir::ParamType::BlockID => {
                if val == 0 {
                    instr.op = Nop;
                    instr.params = vec![];
                } else {
                    instr.op = Jr;
                    instr.params = vec![last_param];
                }
            },
            ir::ParamType::Output(data_type) => {
                instr.op = match data_type {
                    ir::DataType::Int8 =>  Move8_8,
                    ir::DataType::Int16 => Move16_16,
                    ir::DataType::Int32 => Move32_32,
                    ir::DataType::Float => MoveF_F,
                    _ => panic!("Unexpected output type"),
                };
                let const_val = ir::Parameter {
                    param_type: ir::ParamType::Input(data_type),
                    value: ir::ParamValue::Constant(val),
                };
                instr.params = vec![const_val, last_param];
            },
            _ => panic!("Unexpected parameter"),
        }
    }
    progress
}
