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
    for block in obj.blocks.iter() {
        targets.insert(block.id, block.instrs[0].ip);
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