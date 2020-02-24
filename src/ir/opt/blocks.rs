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

use std::collections::HashMap;

use crate::ir;

fn op_is_jump(op: ir::Opcode) -> bool {
    match op {
        ir::Opcode::Return => true,
        ir::Opcode::ObjectEnd => true,
        _ => {
            for param_type in op.get_proto().iter() {
                if let ir::ParamType::Offset = param_type {
                    return true;
                }
            }
            false
        },
    }
}

pub fn flat_to_blocks_obj(obj: &mut ir::Object) {
    assert!(obj.blocks.is_empty());

    /* First, we walk the instructions to find every ip which is a jump target.
     * These will be the first instructions in each block.
     */
    let mut targets = HashMap::new();
    for instr in obj.instrs.iter() {
        for param in instr.params.iter() {
            if let ir::ParamType::IP = param.param_type {
                targets.insert(param.to_u32(), u32::max_value());
            }
        }

        /* ObjectEnd always goes in its own block */
        if let ir::Opcode::ObjectEnd = instr.op {
            targets.insert(instr.ip, u32::max_value());
        }
    }

    let mut block = ir::Block::new(0);
    let mut next_block_id = 1;
    for instr in obj.instrs.drain(..) {
        /* Every jump target starts a new block if needed */
        if let Some(block_id) = targets.get_mut(&instr.ip) {
            if !block.instrs.is_empty() {
                obj.blocks.push(block);
                block = ir::Block::new(next_block_id);
                next_block_id += 1;
            }
            *block_id = block.id;
        }

        let is_jump = op_is_jump(instr.op);
        block.instrs.push(instr);

        /* Every jump ends a block.  This includes ObjectEnd */
        if is_jump {
            obj.blocks.push(block);
            block = ir::Block::new(next_block_id);
            next_block_id += 1;
        }
    }
    assert!(block.instrs.is_empty());

    for instr in obj.iter_instrs_mut() {
        for param in instr.params.iter_mut() {
            if let ir::ParamType::IP = param.param_type {
                let ip = param.to_u32();
                let block_id = *targets.get(&ip).unwrap();
                param.param_type = ir::ParamType::BlockID;
                param.value = ir::ParamValue::Constant(block_id as i32);
            }
        }
    }
}

pub fn blocks_to_flat_obj(obj: &mut ir::Object) {
    assert!(obj.instrs.is_empty());

    let mut targets = HashMap::new();

    /* Walk the blocks backwards and remember the IP of the first instruction
     * in the previous block.  This way, if we encounter an empty block, any
     * jumps to it will harmlessly jump to the subsequent block.  Since the
     * last block is always the one containing EndObject, it's never empty.
     */
    debug_assert!(!obj.blocks.last().unwrap().instrs.is_empty());
    let mut block_ip = 0;
    for block in obj.blocks.iter().rev() {
        if !block.instrs.is_empty() {
            block_ip = block.instrs[0].ip;
        }
        targets.insert(block.id, block_ip);
    }

    for mut block in obj.blocks.drain(..) {
        for mut instr in block.instrs.drain(..) {
            for param in instr.params.iter_mut() {
                if let ir::ParamType::BlockID = param.param_type {
                    let block_id = param.to_u32();
                    let ip = *targets.get(&block_id).unwrap();
                    param.param_type = ir::ParamType::IP;
                    param.value = ir::ParamValue::Constant(ip as i32);
                }
            }

            obj.instrs.push(instr);
        }
    }
}

fn instr_get_jr_block_id(instr: &ir::Instruction) -> Option<u32> {
    for param in instr.params.iter() {
        if let ir::ParamType::BlockID = param.param_type {
            return Some(param.to_u32());
        }
    }
    None
}

pub fn remove_trivial_jumps_obj(obj: &mut ir::Object) {
    assert!(obj.instrs.is_empty());

    debug_assert!(!obj.blocks.last().unwrap().instrs.is_empty());
    let mut last_block_id = std::u32::MAX;
    for block in obj.blocks.iter_mut().rev() {
        if let Some(last) = block.instrs.last_mut() {
            if let Some(id) = instr_get_jr_block_id(last) {
                if id == last_block_id {
                    last.op = ir::Opcode::Nop;
                    last.params = vec![];
                }
            }
        }

        if !block.instrs.is_empty() {
            last_block_id = block.id;
        }
    }
}

pub fn clear_dead_blocks_obj(obj: &mut ir::Object) -> bool {
    assert!(obj.instrs.is_empty());

    let num_blocks = obj.blocks.len();
    assert!(num_blocks != 0);

    let mut id_to_idx = HashMap::new();
    for (idx, block) in obj.blocks.iter().enumerate() {
        id_to_idx.insert(block.id, idx);
    }

    let mut live = vec![];
    live.resize(num_blocks, false);

    live[0] = true;
    /* The last block contains the ObjectEnd */
    live[num_blocks - 1] = true;

    let mut progress = true;
    while progress {
        progress = false;

        for (idx, block) in obj.blocks.iter().enumerate() {
            if !live[idx] {
                continue;
            }

            if let Some(instr) = block.instrs.last() {
                for param in instr.params.iter() {
                    if let ir::ParamType::BlockID = param.param_type {
                        let jr_idx = id_to_idx.get(&param.to_u32()).unwrap();
                        if !live[*jr_idx] {
                            progress = true;
                            live[*jr_idx] = true;
                        }
                    }
                }
                match instr.op {
                    ir::Opcode::Jr => {},
                    ir::Opcode::Return => {},
                    ir::Opcode::ObjectEnd => {},
                    _ => {
                        /* Anything else falls through to the next block */
                        if !live[idx + 1] {
                            progress = true;
                            live[idx + 1] = true;
                        }
                    },
                }
            } else {
                /* Empty blocks fall through */
                if !live[idx + 1] {
                    progress = true;
                    live[idx + 1] = true;
                }
            }
        }
    }

    let mut progress = false;
    for (idx, block) in obj.blocks.iter_mut().enumerate() {
        if !live[idx] && !block.instrs.is_empty() {
            block.instrs.clear();
            progress = true;
        }
    }
    progress
}

fn block_get_jr_block_id(block: &ir::Block) -> Option<u32> {
    if let Some(instr) = block.instrs.last() {
        instr_get_jr_block_id(instr)
    } else {
        None
    }
}

fn block_num_successors(block: &ir::Block) -> u8 {
    if let Some(instr) = block.instrs.last() {
        if op_is_jump(instr.op) {
            match instr.op {
                ir::Opcode::Jr |
                ir::Opcode::Return |
                ir::Opcode::ObjectEnd => 1,
                _ => 2,
            }
        } else {
            1
        }
    } else {
        1
    }
}

pub fn peephole_select_obj(obj: &mut ir::Object) -> bool {
    let mut progress = false;

    let num_blocks = obj.blocks.len();
    for idx in 0..num_blocks {
        /* We're going to look at four blocks */
        if (idx + 4) as usize > num_blocks {
            break;
        }

        let block0 = &obj.blocks[idx + 0];
        let block1 = &obj.blocks[idx + 1];
        let block2 = &obj.blocks[idx + 2];
        let block3 = &obj.blocks[idx + 3];

        /* The first block needs to jump to the second and third blocks */
        if block_num_successors(&obj.blocks[idx + 0]) != 2 {
            continue;
        }
        if block_get_jr_block_id(block0).unwrap() != block2.id {
            continue;
        }

        /* The second block needs to unconditionally jump to the fourth and
         * only contain two instructions: A Move and the jump
         */
        if block_num_successors(block1) != 1 {
            continue;
        }
        if let Some(id) = block_get_jr_block_id(block1) {
            if id != block3.id {
                continue;
            }
        } else {
            continue;
        }
        if block1.instrs.len() != 2 {
            continue;
        }
        let else_move = block1.instrs.first().unwrap().clone();

        /* The third block must unconditionally jump to the fourth and only
         * contain one instruction: A Move
         */
        if block_num_successors(block2) != 1 {
            continue;
        }
        if block2.instrs.len() != 1 {
            continue;
        }
        let then_move = block2.instrs.first().unwrap().clone();

        /* They have to have the same opcode */
        if then_move.op != else_move.op {
            continue;
        }

        /* They have to be a non-converting move */
        let sel_op = match then_move.op {
            ir::Opcode::Move8_8 => ir::Opcode::Select8,
            ir::Opcode::Move16_16 => ir::Opcode::Select16,
            ir::Opcode::Move32_32 => ir::Opcode::Select32,
            ir::Opcode::MoveF_F => ir::Opcode::SelectF,
            _ => continue,
        };

        /* They have to have the same destination */
        if then_move.params[1] != else_move.params[1] {
            continue;
        }

        let block0 = &mut obj.blocks[idx + 0];
        let cmp = block0.instrs.last_mut().unwrap();
        let jr_op = cmp.op;

        cmp.op = match jr_op {
            ir::Opcode::JrFalse =>  ir::Opcode::CpEq8,
            ir::Opcode::JrTrue =>   ir::Opcode::CpNeq8,
            ir::Opcode::JrNan =>    ir::Opcode::CpNeqF,

            ir::Opcode::JrLt8 =>    ir::Opcode::CpLt8,
            ir::Opcode::JrLt16 =>   ir::Opcode::CpLt16,
            ir::Opcode::JrLt32 =>   ir::Opcode::CpLt32,
            ir::Opcode::JrLtF =>    ir::Opcode::CpLtF,
            ir::Opcode::JrGt8 =>    ir::Opcode::CpGt8,
            ir::Opcode::JrGt16 =>   ir::Opcode::CpGt16,
            ir::Opcode::JrGt32 =>   ir::Opcode::CpGt32,
            ir::Opcode::JrGtF =>    ir::Opcode::CpGtF,
            ir::Opcode::JrEq8 =>    ir::Opcode::CpEq8,
            ir::Opcode::JrEq16 =>   ir::Opcode::CpEq16,
            ir::Opcode::JrEq32 =>   ir::Opcode::CpEq32,
            ir::Opcode::JrEqF =>    ir::Opcode::CpEqF,
            ir::Opcode::JrNeq8 =>   ir::Opcode::CpNeq8,
            ir::Opcode::JrNeq16 =>  ir::Opcode::CpNeq16,
            ir::Opcode::JrNeq32 =>  ir::Opcode::CpNeq32,
            ir::Opcode::JrNeqF =>   ir::Opcode::CpNeqF,
            ir::Opcode::JrLteq8 =>  ir::Opcode::CpLteq8,
            ir::Opcode::JrLteq16 => ir::Opcode::CpLteq16,
            ir::Opcode::JrLteq32 => ir::Opcode::CpLteq32,
            ir::Opcode::JrLteqF =>  ir::Opcode::CpLteqF,
            ir::Opcode::JrGteq8 =>  ir::Opcode::CpGteq8,
            ir::Opcode::JrGteq16 => ir::Opcode::CpGteq16,
            ir::Opcode::JrGteq32 => ir::Opcode::CpGteq32,
            ir::Opcode::JrGteqF =>  ir::Opcode::CpGteqF,
            _ => panic!("Unknown jump instruction"),
        };

        /* Pop off the Block ID parameter; we don't need it anymore */
        let block_id = cmp.params.pop().unwrap();
        assert!(block_id.param_type == ir::ParamType::BlockID);

        /* Three of the jump opcodes require an extra parameter */
        match jr_op {
            ir::Opcode::JrFalse |
            ir::Opcode::JrTrue => {
                cmp.params.push(ir::Parameter {
                    param_type: ir::ParamType::Output(ir::DataType::Int8),
                    value: ir::ParamValue::Constant(0),
                });
            },
            ir::Opcode::JrNan => cmp.params.push(cmp.params[0].clone()),
            _ => {},
        }

        /* Grab a temporary local to store the comparison */
        let tmp_value_byte = obj.local_bytes;
        obj.local_bytes += 1;

        cmp.params.push(ir::Parameter {
            param_type: ir::ParamType::Output(ir::DataType::Int8),
            value: ir::ParamValue::Local(tmp_value_byte),
        });

        block0.instrs.push(ir::Instruction {
            ip: obj.last_ip,
            op: sel_op,
            params: vec![
                ir::Parameter {
                    param_type: ir::ParamType::Input(ir::DataType::Int8),
                    value: ir::ParamValue::Constant(0),
                },
                then_move.params[0].clone(),
                else_move.params[0].clone(),
                then_move.params[1].clone(),
            ],
        });

        obj.blocks[idx + 1].instrs.clear();
        obj.blocks[idx + 2].instrs.clear();

        progress = true;
    }

    progress
}
