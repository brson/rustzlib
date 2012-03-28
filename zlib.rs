use std;

import core::ptr;
import core::vec;

import libc::c_int;

export compress, uncompress;

#[link_name = "z"]
#[abi = "cdecl"]
native mod _native {
    fn compressBound(len: u64) -> u64;
    fn compress2(dest: *u8, destlen: *u64, src: *u8, srclen: u64,
                 level: c_int) -> c_int;
    fn uncompress(dest: *u8, destlen: *u64, src: *u8, srclen: u64) -> c_int;
}

fn compress(src: [u8], level: int) -> [u8] unsafe {
    assert(level >= 0);
    assert(level <= 9);
    let srclen = vec::len(src) as u64;
    let destlen = _native::compressBound(srclen);
    let dest: [mut u8] = vec::to_mut(vec::from_elem::<u8>(destlen as uint, 0u8));
    let pdest = vec::unsafe::to_ptr::<u8>(dest);
    let psrc = vec::unsafe::to_ptr::<u8>(src);
    let pdestlen = ptr::addr_of::<u64>(destlen);
    let r = _native::compress2(pdest, pdestlen, psrc, srclen, level as c_int);
    // XXX: 0 == Z_OK
    if r != 0 as c_int {
        fail #fmt["zlib compress2() returned %d", r as int];
    }
    vec::slice::<u8>(dest, 0u, destlen as uint)
}

fn uncompress(src: [u8], _destlen: u64) -> [u8] unsafe {
    let srclen = vec::len(src) as u64;
    let destlen = _destlen;
    let pdestlen = ptr::addr_of::<u64>(destlen);
    let dest: [mut u8] = vec::to_mut(vec::from_elem::<u8>(destlen as uint, 0u8));
    let pdest = vec::unsafe::to_ptr::<u8>(dest);
    let psrc = vec::unsafe::to_ptr::<u8>(src);
    let r = _native::uncompress(pdest, pdestlen, psrc, srclen);
    if r != 0 as c_int {
        fail #fmt["zlib uncompress() returned %d", r as int];
    }
    vec::slice::<u8>(dest, 0u, destlen as uint)
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let d: [u8] = [0xdeu8, 0xadu8, 0xd0u8, 0x0du8];
        let c = compress(d, 9);
        let r = uncompress(c, vec::len(d) as u64);
        assert(r == d);
    }
}
