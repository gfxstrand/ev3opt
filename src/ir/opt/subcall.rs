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

use std::collections::VecDeque;
use std::collections::HashMap;

fn should_inline_subcall(instr: &ir::Instruction) -> bool {
    if let ir::Opcode::Call = instr.op {
        for param in instr.params.iter() {
            /* The primary advantage of inlining is to be able to constant-fold
             * across subcall boundaries.  Consider any subcalls with a constant
             * parameter to be an inline candidate.
             */
            if let ir::ParamValue::Constant(_) = param.value {
                return true;
            }
        }
    }
    false
}

fn move_instr(dst: ir::ParamValue, src: ir::ParamValue,
              data_type: ir::DataType) -> ir::Instruction {
    let op = match data_type {
        ir::DataType::Int8 => ir::Opcode::Move8_8,
        ir::DataType::Int16 => ir::Opcode::Move16_16,
        ir::DataType::Int32 => ir::Opcode::Move32_32,
        ir::DataType::Float => ir::Opcode::MoveF_F,
        _ => panic!("Unhandled subcall parameter type"),
    };

    let dst_param = ir::Parameter {
        param_type: ir::ParamType::Output(data_type),
        value: dst,
    };

    let src_param = ir::Parameter {
        param_type: ir::ParamType::Input(data_type),
        value: src,
    };

    ir::Instruction {
        ip: 0,
        op: op,
        params: vec![src_param, dst_param],
    }
}

pub fn inline_subcalls(image: &mut ir::Image) -> bool {
    let mut callees = VecDeque::new();
    for obj in image.objects.iter() {
        /* This pass assumes a flat list of instructions */
        assert!(obj.blocks.is_empty());

        for instr in obj.instrs.iter() {
            if should_inline_subcall(instr) {
                let callee_idx = instr.params[0].to_u32() - 1;
                let callee_obj = &image.objects[callee_idx as usize];
                callees.push_back((callee_idx, callee_obj.clone()));
            }
        }
    }

    if callees.is_empty() {
        return false;
    }

    for caller in image.objects.iter_mut() {
        let mut instrs = vec![];
        for instr in caller.instrs.drain(..) {
            if should_inline_subcall(&instr) {
                /* First, insert a Nop with the same ip as the call instruction
                 * in case it's used as a jump target.
                 */
                instrs.push(ir::Instruction {
                    ip: instr.ip,
                    op: ir::Opcode::Nop,
                    params: vec![],
                });

                let (callee_idx, mut callee) = callees.pop_front().unwrap();
                assert!(callee_idx == instr.params[0].to_u32() - 1);

                /* This pass assumes a flat list of instructions */
                assert!(callee.blocks.is_empty());

                /* Align up to 4 bytes just to be safe */
                let callee_locals_start = (caller.local_bytes + 3) & !3u32;
                caller.local_bytes = callee_locals_start + callee.local_bytes;

                let mut byte = callee_locals_start;
                for param in &instr.params[1..] {
                    if let ir::ParamType::Input(data_type) = param.param_type {
                        let mut mov = move_instr(ir::ParamValue::Local(byte),
                                                 param.value.clone(),
                                                 data_type);
                        mov.ip = caller.last_ip + 1;
                        caller.last_ip = mov.ip;
                        instrs.push(mov);
                    }
                    byte += param.param_type.data_type().size();
                }
                debug_assert!(byte <= caller.local_bytes);

                let mut ip_map = HashMap::new();
                for instr in callee.instrs.iter_mut() {
                    caller.last_ip += 1;
                    ip_map.insert(instr.ip, caller.last_ip);
                    instr.ip = caller.last_ip;
                }
                let obj_end_ip = caller.last_ip;

                for mut instr in callee.instrs.drain(..) {
                    for param in instr.params.iter_mut() {
                        if let ir::ParamValue::Local(byte) = param.value {
                            let byte = callee_locals_start + byte;
                            param.value = ir::ParamValue::Local(byte);
                        }
                        if let ir::ParamType::IP = param.param_type {
                            let new_ip = ip_map.get(&param.to_u32()).unwrap();
                            param.value = ir::ParamValue::Constant(*new_ip as i32);
                        }
                    }
                    match instr.op {
                        ir::Opcode::Return => {
                            debug_assert!(instr.params.is_empty());
                            /* Turn returns into a jump to ObjectEnd */
                            instr.op = ir::Opcode::Jr;
                            instr.params = vec![ir::Parameter {
                                param_type: ir::ParamType::IP,
                                value: ir::ParamValue::Constant(obj_end_ip as i32),
                            }];
                        },
                        ir::Opcode::ObjectEnd => {
                            debug_assert!(instr.params.is_empty());
                            /* Turn ObjectEnd into Nop */
                            instr.op = ir::Opcode::Nop;
                        },
                        _ => {},
                    }
                    instrs.push(instr);
                }

                let mut byte = callee_locals_start;
                for param in &instr.params[1..] {
                    if let ir::ParamType::Output(data_type) = param.param_type {
                        let mut mov = move_instr(param.value.clone(),
                                                 ir::ParamValue::Local(byte),
                                                 data_type);
                        mov.ip = caller.last_ip + 1;
                        caller.last_ip = mov.ip;
                        instrs.push(mov);
                    }
                    byte += param.param_type.data_type().size();
                }
                debug_assert!(byte <= caller.local_bytes);
            } else {
                instrs.push(instr);
            }
        }
        caller.instrs = instrs;
    }

    true
}

pub fn remove_dead_subcalls(image: &mut ir::Image) -> bool {
    let num_objects = image.objects.len();
    let mut live = vec![];
    live.resize(num_objects, false);

    let mut progress = true;
    while progress {
        progress = false;

        for (idx, obj) in image.objects.iter().enumerate() {
            /* Consider all VMTHREAD and BLOCK objects always live */
            if !obj.is_subcall() {
                live[idx] = true;
            }

            if live[idx] {
                for instr in obj.iter_instrs() {
                    if let ir::Opcode::Call = instr.op {
                        let callee_idx = (instr.params[0].to_u32() - 1) as usize;
                        if !live[callee_idx] {
                            live[callee_idx] = true;
                            progress = true;
                        }
                    }
                }
            }
        }
    }

    let mut remap = vec![];
    remap.resize(num_objects, 0i32);

    let mut kept = vec![];
    for (idx, obj) in image.objects.drain(..).enumerate() {
        if live[idx] {
            remap[idx] = kept.len() as i32;
            kept.push(obj);
        }
    }
    image.objects = kept;

    for obj in image.objects.iter_mut() {
        for instr in obj.iter_instrs_mut() {
            if let ir::Opcode::Call = instr.op {
                let callee_idx = (instr.params[0].to_u32() - 1) as usize;
                instr.params[0].value =
                    ir::ParamValue::Constant(remap[callee_idx] + 1);
            }
        }
    }

    image.objects.len() < num_objects
}
