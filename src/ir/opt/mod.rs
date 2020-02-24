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

mod blocks;
mod constant;
mod dead_code;
mod memory;
mod subcall;

pub fn optimize_obj(obj: &mut ir::Object) -> bool{
    blocks::flat_to_blocks_obj(obj);

    memory::trim_array_lengths_obj(obj);

    let mut progress = false;
    loop {
        let mut p = false;
        p |= memory::copy_propagation_obj(obj);
        p |= constant::constant_fold_obj(obj);
        p |= blocks::clear_dead_blocks_obj(obj);
        p |= dead_code::dead_code_obj(obj);
        p |= dead_code::remove_nops_obj(obj);
        p |= blocks::peephole_select_obj(obj);

        if p {
            progress = true;
        } else {
            break;
        }
    }

    blocks::remove_trivial_jumps_obj(obj);
    blocks::blocks_to_flat_obj(obj);

    progress
}

pub fn optimize(image: &mut ir::Image) -> bool{
    let mut progress = false;
    loop {
        let mut p = false;
        p |= memory::global_to_local(image);
        for obj in image.objects.iter_mut() {
            p |= optimize_obj(obj);
        }
        p |= subcall::inline_subcalls(image);
        p |= subcall::remove_dead_subcalls(image);

        if p {
            progress = true;
        } else {
            break;
        }
    }

    progress
}
