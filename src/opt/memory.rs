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

#[derive(Copy, Clone)]
enum GlobalMemUsage {
    Unused,
    Global,
    Local(u32),
}

fn update_global_usage(usage: &mut Vec<GlobalMemUsage>,
                       obj_idx: u32, start: u32, size: u32) {
    let end = start + size;

    let mut any_other_block = false;
    for byte in start..end {
        match usage[byte as usize] {
            GlobalMemUsage::Unused => {},
            GlobalMemUsage::Global => {
                any_other_block = true;
                break;
            },
            GlobalMemUsage::Local(other_idx) => {
                if other_idx != obj_idx {
                    any_other_block = true;
                    break;
                }
            },
        }
    }

    if any_other_block {
        for byte in start..end {
            usage[byte as usize] = GlobalMemUsage::Global;
        }
    } else {
        for byte in start..end {
            usage[byte as usize] = GlobalMemUsage::Local(obj_idx);
        }
    }
}

fn update_alignments(alignments: &mut Vec<u8>, align: u8,
                     start: u32, size: u32) {
    let end = start + size;
    assert!(start % (align as u32) == 0);

    let mut align = align;
    for byte in start..end {
        align = cmp::max(align, alignments[byte as usize]);
    }

    /* Mark everything aligned as high as we can reasonably go */
    for byte in start..end {
        alignments[byte as usize] = align;
    }
}

pub fn global_to_local(image: &mut ir::Image) -> bool {
    /* Save off an immutable copy of this */
    let global_bytes = image.global_bytes;

    let mut usage = vec![];
    usage.resize(global_bytes as usize, GlobalMemUsage::Unused);

    let mut alignments = vec![];
    alignments.resize(global_bytes as usize, 0u8);

    for obj_idx in 0..image.objects.len() {
        for instr in image.objects[obj_idx].iter_instrs() {
            for param in instr.params.iter() {
                if let ir::ParamValue::Global(start) = param.value {
                    assert!(start < global_bytes);
                    let size = cmp::min(param.param_type.data_type().size(),
                                        global_bytes - start);
                    let align = param.param_type.data_type().align() as u8;
                    update_global_usage(&mut usage, obj_idx as u32, start, size);
                    update_alignments(&mut alignments, align, start, size);
                }
            }
        }
    }

    let mut global_to_global = vec![];
    global_to_global.resize(global_bytes as usize, 0u32);

    /* Build one of these and re-use it for each object */
    let mut global_to_local = vec![];
    global_to_local.resize(global_bytes as usize, 0u32);

    let mut need_remap = false;
    let mut next_byte = 0;
    for byte in 0..global_bytes {
        debug_assert!(next_byte <= byte);
        match usage[byte as usize] {
            GlobalMemUsage::Unused => { },
            GlobalMemUsage::Local(_) => need_remap = true,
            GlobalMemUsage::Global => {
                /* Make sure next_byte has the right alignment */
                let byte_align = alignments[byte as usize] as u32;
                debug_assert!(byte_align > 0);
                next_byte += (byte - next_byte) & (byte_align - 1);
                debug_assert!(next_byte % byte_align == byte % byte_align);

                if byte != next_byte {
                    need_remap = true;
                }

                global_to_global[byte as usize] = next_byte;
                next_byte += 1;
            }
        }
    }
    image.global_bytes = next_byte;

    if !need_remap {
        return false;
    }

    for obj_idx in 0..image.objects.len() {
        next_byte = image.objects[obj_idx].local_bytes;
        for byte in 0..global_bytes {
            if let GlobalMemUsage::Local(idx) = usage[byte as usize] {
                if idx as usize == obj_idx {
                    /* Make sure next_byte has the right alignment */
                    let byte_align = alignments[byte as usize] as u32;
                    debug_assert!(byte_align > 0);
                    next_byte += byte.wrapping_sub(next_byte) & (byte_align - 1);
                    debug_assert!(next_byte % byte_align == byte % byte_align);

                    global_to_local[byte as usize] = next_byte;
                    next_byte += 1;
                }
            }
        }
        image.objects[obj_idx].local_bytes = next_byte;

        for instr in image.objects[obj_idx].iter_instrs_mut() {
            for param in instr.params.iter_mut() {
                if let ir::ParamValue::Global(start) = param.value {
                    match usage[start as usize] {
                        GlobalMemUsage::Unused => {
                            panic!("We should have seen this before");
                        }
                        GlobalMemUsage::Local(idx) => {
                            debug_assert!(idx as usize == obj_idx);
                            param.value = ir::ParamValue::Local(global_to_local[start as usize]);
                        },
                        GlobalMemUsage::Global => {
                            param.value = ir::ParamValue::Global(global_to_global[start as usize]);
                        },
                    }
                }
            }
        }
    }
    true
}

#[derive(Copy, Clone)]
enum MemValue {
    Unwritten,
    OneValue,
    ManyValues,
    CopyMem(u32),
    Constant(u8),
}

impl MemValue {
    pub fn merge(&mut self, other: MemValue) {
        match self {
            MemValue::Unwritten => *self = other,
            MemValue::OneValue => *self = MemValue::ManyValues,
            MemValue::ManyValues => {},
            MemValue::CopyMem(x) => {
                if let MemValue::CopyMem(y) = other {
                    if *x != y {
                        *self = MemValue::ManyValues;
                    }
                } else {
                    *self = MemValue::ManyValues;
                }
            },
            MemValue::Constant(x) => {
                if let MemValue::Constant(y) = other {
                    if *x != y {
                        *self = MemValue::ManyValues;
                    }
                } else {
                    *self = MemValue::ManyValues;
                }
            },
        }
    }
}

fn merge_mem_values_const_i32(values: &mut Vec<MemValue>,
                              start: u32, size: u32, x: i32) {
    for b in 0..size {
        let x_byte = ((x as u32) >> (b * 8)) as u8;
        values[(start + b) as usize].merge(MemValue::Constant(x_byte));
    }
}

fn merge_mem_values_unknown(values: &mut Vec<MemValue>,
                            start: u32, size: u32) {
    let size = cmp::min(size, values.len() as u32 - start);
    for b in 0..size {
        values[(start + b) as usize].merge(MemValue::OneValue);
    }
}

fn set_mem_values_const_i32(values: &mut Vec<MemValue>,
                            start: u32, size: u32, x: i32) {
    for b in 0..size {
        let x_byte = ((x as u32) >> (b * 8)) as u8;
        values[(start + b) as usize] = MemValue::Constant(x_byte);
    }
}

fn set_mem_values_copy_mem(values: &mut Vec<MemValue>,
                           dst: u32, size: u32, src: u32) {
    for b in 0..size {
        values[(dst + b) as usize] = MemValue::CopyMem(src + b);
    }
}

fn destroy_mem_copy_values(values: &mut Vec<MemValue>,
                           start: u32, size: u32) {
    for val in values.iter_mut() {
        if let MemValue::CopyMem(byte) = val {
            if *byte >= start && (*byte - start) < size {
                *val = MemValue::ManyValues;
            }
        }
    }
}

fn get_const_mem_value(values: &Vec<MemValue>,
                       start: u32, size: u32) -> Option<i32> {
    let mut x = 0u32;
    for b in 0..size {
        if let MemValue::Constant(x_byte) = values[(start + b) as usize] {
            x |= (x_byte as u32) << (b * 8);
        } else {
            return None;
        }
    }
    Some(x as i32)
}

fn get_copy_mem_value(values: &Vec<MemValue>,
                      start: u32, size: u32) -> Option<u32> {
    if let MemValue::CopyMem(byte_0) = values[start as usize] {
        for n in 1..size {
            if let MemValue::CopyMem(byte_n) = values[(start + n) as usize] {
                if byte_n != byte_0 + n {
                    return None;
                }
            } else {
                return None;
            }
        }
        Some(byte_0)
    } else {
        None
    }
}

fn update_const_mem_values_for_instr(values: &mut Vec<MemValue>,
                                     instr: &ir::Instruction,
                                     block_local: bool) {
    match instr.op {
        ir::Opcode::Move8_8 |
        ir::Opcode::Move16_16 |
        ir::Opcode::Move32_32 |
        ir::Opcode::MoveF_F => {
            debug_assert!(instr.params.len() == 2);
            if let ir::ParamValue::Local(dst) = instr.params[1].value {
                let size = instr.params[1].param_type.data_type().size();
                if let ir::ParamValue::Constant(x) = instr.params[0].value {
                    if block_local {
                        set_mem_values_const_i32(values, dst, size, x);
                    } else {
                        merge_mem_values_const_i32(values, dst, size, x);
                    }
                } else if block_local {
                    if let ir::ParamValue::Local(src) = instr.params[0].value {
                        set_mem_values_copy_mem(values, dst, size, src);
                    }
                } else {
                    merge_mem_values_unknown(values, dst, size);
                }
                destroy_mem_copy_values(values, dst, size);
            }
        },
        _ => {
            /* Anything else we consider an unknown write.  We could
             * potentially handle more Move instructions here but they
             * will get handled by constant_folding and it saves us
             * complexity if we don't have to think about truncation and
             * sign-extension here.
             */
            for param in instr.params.iter() {
                if let ir::ParamType::Output(_) = param.param_type {
                    if let ir::ParamValue::Local(byte) = param.value {
                        let size = param.param_type.data_type().size();
                        merge_mem_values_unknown(values, byte, size);
                        destroy_mem_copy_values(values, byte, size);
                    }
                }
            }
        },
    }
}

pub fn copy_propagation_obj(obj: &mut ir::Object) -> bool {
    /* This pass assumes we have basic blocks */
    assert!(obj.instrs.is_empty());

    let mut obj_values = vec![];
    obj_values.resize(obj.local_bytes as usize, MemValue::Unwritten);

    for instr in obj.iter_instrs() {
        update_const_mem_values_for_instr(&mut obj_values, instr, false);
    }

    let mut progress = false;

    for block in obj.blocks.iter_mut() {
        /* Within a single basic block, we can do more accurate tracking
         * because we can track only the most recently written values rather
         * than having to turn any multi-write into Unknown
         */
        let mut block_values = vec![];
        block_values.resize(obj.local_bytes as usize, MemValue::Unwritten);

        for instr in block.instrs.iter_mut() {
            for param in instr.params.iter_mut() {
                if let ir::ParamType::Input(_) = param.param_type {
                    if let ir::ParamValue::Local(byte) = param.value {
                        let size = param.param_type.data_type().size();
                        if let Some(x) = get_const_mem_value(&block_values,
                                                             byte, size) {
                            param.value = ir::ParamValue::Constant(x);
                            progress = true;
                        } else if let Some(x) = get_copy_mem_value(&block_values,
                                                                   byte, size) {
                            param.value = ir::ParamValue::Local(x);
                            progress = true;
                        } else if let Some(x) = get_const_mem_value(&obj_values,
                                                                    byte, size) {
                            param.value = ir::ParamValue::Constant(x);
                            progress = true;
                        }
                    }
                }
            }

            /* After we've processed this instruction and filled out any
             * constant sources, try to add it to the block-local values list.
             * We don't need to merge in this case because we're not worried
             * about control-flow issues within a single basic block.
             */
            update_const_mem_values_for_instr(&mut block_values, instr, true);
        }
    }

    progress
}

pub fn trim_param_size(param: &mut ir::Parameter, size: u32) -> bool {
    match param.param_type {
        ir::ParamType::Input(ref mut data_type) |
        ir::ParamType::Output(ref mut data_type) => {
            assert!(size >= data_type.without_array().size());
            if data_type.is_array() {
                let max_len = size / data_type.without_array().size();
                if max_len <= std::u8::MAX as u32 {
                    let max_len = max_len as u8;
                    let old_len = data_type.array_len();
                    if old_len == 0 || max_len < old_len {
                        data_type.set_array_len(max_len);
                        return true;
                    }
                }
            }
        },
        ir::ParamType::Offset |
        ir::ParamType::IP |
        ir::ParamType::BlockID => {
            assert!(size >= 4);
        },
    }
    false
}

pub fn trim_array_lengths_obj(obj: &mut ir::Object) -> bool {
    let mut progress = false;

    let local_bytes = obj.local_bytes;
    for instr in obj.iter_instrs_mut() {
        for param in instr.params.iter_mut() {
            if let ir::ParamValue::Local(byte) = param.value {
                debug_assert!(byte < local_bytes);
                progress |= trim_param_size(param, local_bytes - byte);
            }
        }

        match instr.op {
            ir::Opcode::InputDevice(subcode) => {
                match subcode {
                    ir::opcodes::InputDeviceSubcode::READY_PCT |
                    ir::opcodes::InputDeviceSubcode::READY_RAW |
                    ir::opcodes::InputDeviceSubcode::READY_SI => {
                        let num_values = instr.params[4].to_u32();
                        /* We don't know the size of a given value without
                         * querying it with GET_FORMAT which we clearly can't
                         * do in a compiler.  Assume 4 bytes per value.
                         */
                        progress |= trim_param_size(&mut instr.params[5],
                                                    num_values * 4);
                    }
                    _ => {},
                }
            },
            ir::Opcode::InputReadExt => {
                let num_values = instr.params[5].to_u32();
                /* We don't know the size of a given value without querying it
                 * with GET_FORMAT which we clearly can't do in a compiler.
                 * Assume 4 bytes per value.
                 */
                progress |= trim_param_size(&mut instr.params[6],
                                            num_values * 4);
            },
            _ => {},
        }
    }

    progress
}
