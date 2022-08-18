use std::convert::TryInto;

use crate::{interpreter::Interpreter, Return};
use primitive_types::U256;
use ruint::Uint;

pub fn mload(interp: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    pop!(interp, index);
    let index = as_usize_or_fail_ruint!(index, Return::OutOfGas);
    memory_resize!(interp, index, 32);
    let mem_at_index: [u8; 32] = interp
        .memory
        .get_slice(index, 32)
        .try_into()
        .expect("a 32 byte slice of memory should fit into a 32 byte array exactly");

    push!(interp, Uint::from_be_bytes(mem_at_index));
    Return::Continue
}

pub fn mstore(interp: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    pop!(interp, index, value);
    let index = as_usize_or_fail_ruint!(index, Return::OutOfGas);
    memory_resize!(interp, index, 32);
    interp.memory.set_u256(index, value.into());
    Return::Continue
}

pub fn mstore8(interp: &mut Interpreter) -> Return {
    // gas!(interp, gas::VERYLOW);
    pop!(interp, index, value);
    let index = as_usize_or_fail_ruint!(index, Return::OutOfGas);
    memory_resize!(interp, index, 1);
    let value = (U256::from(value).low_u32() & 0xff) as u8;
    // Safety: we resized our memory two lines above.
    unsafe { interp.memory.set_byte(index, value) }
    Return::Continue
}

pub fn msize(interp: &mut Interpreter) -> Return {
    // gas!(interp, gas::BASE);
    push!(interp, Uint::from(interp.memory.effective_len()));
    Return::Continue
}
