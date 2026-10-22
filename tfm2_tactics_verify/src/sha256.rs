//! 설치 EXE의 전체 SHA256을 고정 분석 기준과 대조한다. 외부 crate/게임 API 호출은 없다.
const K: [u32; 64] = [
    0x428a2f98,0x71374491,0xb5c0fbcf,0xe9b5dba5,0x3956c25b,0x59f111f1,0x923f82a4,0xab1c5ed5,
    0xd807aa98,0x12835b01,0x243185be,0x550c7dc3,0x72be5d74,0x80deb1fe,0x9bdc06a7,0xc19bf174,
    0xe49b69c1,0xefbe4786,0x0fc19dc6,0x240ca1cc,0x2de92c6f,0x4a7484aa,0x5cb0a9dc,0x76f988da,
    0x983e5152,0xa831c66d,0xb00327c8,0xbf597fc7,0xc6e00bf3,0xd5a79147,0x06ca6351,0x14292967,
    0x27b70a85,0x2e1b2138,0x4d2c6dfc,0x53380d13,0x650a7354,0x766a0abb,0x81c2c92e,0x92722c85,
    0xa2bfe8a1,0xa81a664b,0xc24b8b70,0xc76c51a3,0xd192e819,0xd6990624,0xf40e3585,0x106aa070,
    0x19a4c116,0x1e376c08,0x2748774c,0x34b0bcb5,0x391c0cb3,0x4ed8aa4a,0x5b9cca4f,0x682e6ff3,
    0x748f82ee,0x78a5636f,0x84c87814,0x8cc70208,0x90befffa,0xa4506ceb,0xbef9a3f7,0xc67178f2,
];
fn block(h: &mut [u32;8], b: &[u8]) {
    let mut w = [0u32;64];
    for (i, v) in w[..16].iter_mut().enumerate() { *v = u32::from_be_bytes(b[i*4..i*4+4].try_into().unwrap()); }
    for i in 16..64 {
        let a = w[i-15]; let b = w[i-2];
        let s0 = a.rotate_right(7)^a.rotate_right(18)^(a>>3);
        let s1 = b.rotate_right(17)^b.rotate_right(19)^(b>>10);
        w[i] = w[i-16].wrapping_add(s0).wrapping_add(w[i-7]).wrapping_add(s1);
    }
    let [mut a,mut b,mut c,mut d,mut e,mut f,mut g,mut z] = *h;
    for i in 0..64 {
        let t = z.wrapping_add(e.rotate_right(6)^e.rotate_right(11)^e.rotate_right(25))
            .wrapping_add((e&f)^(!e&g)).wrapping_add(K[i]).wrapping_add(w[i]);
        let u = (a.rotate_right(2)^a.rotate_right(13)^a.rotate_right(22)).wrapping_add((a&b)^(a&c)^(b&c));
        z=g;g=f;f=e;e=d.wrapping_add(t);d=c;c=b;b=a;a=t.wrapping_add(u);
    }
    for (x,y) in h.iter_mut().zip([a,b,c,d,e,f,g,z]) { *x=x.wrapping_add(y); }
}
pub fn digest(input: &[u8]) -> String {
    let mut h = [0x6a09e667,0xbb67ae85,0x3c6ef372,0xa54ff53a,0x510e527f,0x9b05688c,0x1f83d9ab,0x5be0cd19];
    let mut chunks = input.chunks_exact(64);
    for b in &mut chunks { block(&mut h,b); }
    let r = chunks.remainder(); let mut tail = [0u8;128];
    tail[..r.len()].copy_from_slice(r); tail[r.len()] = 0x80;
    let size = if r.len() < 56 {64} else {128};
    tail[size-8..size].copy_from_slice(&(input.len() as u64).wrapping_mul(8).to_be_bytes());
    for b in tail[..size].chunks_exact(64) { block(&mut h,b); }
    h.iter().map(|x|format!("{x:08x}")).collect()
}
#[cfg(test)] mod tests {
    use super::*;
    #[test] fn published_vectors_and_multi_block() {
        assert_eq!(digest(b""), "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855");
        assert_eq!(digest(b"abc"), "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad");
        assert_eq!(digest(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"), "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1");
        assert_eq!(digest(&vec![b'a';1_000_000]), "cdc76e5c9914fb9281a1c7e284d73e67f1809a48a497200e046d39ccc7112cd0");
    }
}
