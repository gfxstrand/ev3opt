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
