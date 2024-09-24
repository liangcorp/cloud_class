use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(feature = "ssr")] {
        const RAPID_SEED: u64 = 0xbdd89aa982704029;
        const RAPID_SECRET: [u64; 3] = [0x2d358dccaa6c78a5, 0x8bb84b93962eacc9, 0x4b33a62ed433d4a3];

        #[inline(always)]
        fn rapid_mum(a: &mut u64, b: &mut u64) {
            let r = *a as u128 * *b as u128;
            *a = r as u64;
            *b = (r >> 64) as u64;
        }

        #[inline(always)]
        fn rapid_mix(mut a: u64, mut b: u64) -> u64 {
            rapid_mum(&mut a, &mut b);
            a ^ b
        }

        #[inline]
        fn rapidhash_internal(data: &[u8], mut seed: u64) -> u64 {
            seed ^= rapid_mix(seed ^ RAPID_SECRET[0], RAPID_SECRET[1]) ^ (data.len() as u64);

            let mut a: u64;
            let mut b: u64;

            if data.len() <= 16 {
                if data.len() >= 4 {
                    let plast = data.len() - 4;
                    a = ((bytes_u32(&data[0..4]) as u64) << 32) | bytes_u32(&data[plast..]) as u64;
                    let delta = (data.len() & 24) >> (data.len() >> 3);
                    b = ((bytes_u32(&data[delta..]) as u64) << 32) | bytes_u32(&data[plast - delta..]) as u64;
                } else if data.len() > 0 {
                    // len is bounded between 1 and 3
                    let len = data.len();
                    a = ((data[0] as u64) << 56) | ((data[len >> 1] as u64) << 32) | data[len - 1] as u64;
                    b = 0;
                } else {
                    a = 0;
                    b = 0;
                }
            } else {
                let mut slice = data;

                // "most CPUs benefit from this unrolled loop"
                let mut see1 = seed;
                let mut see2 = seed;
                while slice.len() >= 96 {
                    seed = rapid_mix(bytes_u64(&slice[0..]) ^ RAPID_SECRET[0], bytes_u64(&slice[8..]) ^ seed);
                    see1 = rapid_mix(bytes_u64(&slice[16..]) ^ RAPID_SECRET[1], bytes_u64(&slice[24..]) ^ see1);
                    see2 = rapid_mix(bytes_u64(&slice[32..]) ^ RAPID_SECRET[2], bytes_u64(&slice[40..]) ^ see2);
                    seed = rapid_mix(bytes_u64(&slice[48..]) ^ RAPID_SECRET[0], bytes_u64(&slice[56..]) ^ seed);
                    see1 = rapid_mix(bytes_u64(&slice[64..]) ^ RAPID_SECRET[1], bytes_u64(&slice[72..]) ^ see1);
                    see2 = rapid_mix(bytes_u64(&slice[80..]) ^ RAPID_SECRET[2], bytes_u64(&slice[88..]) ^ see2);
                    slice = &slice[96..];
                }
                if slice.len() >= 48 {
                    seed = rapid_mix(bytes_u64(&slice[0..]) ^ RAPID_SECRET[0], bytes_u64(&slice[8..]) ^ seed);
                    see1 = rapid_mix(bytes_u64(&slice[16..]) ^ RAPID_SECRET[1], bytes_u64(&slice[24..]) ^ see1);
                    see2 = rapid_mix(bytes_u64(&slice[32..]) ^ RAPID_SECRET[2], bytes_u64(&slice[40..]) ^ see2);
                    slice = &slice[48..];
                }
                seed ^= see1 ^ see2;

                if slice.len() > 16 {
                    seed = rapid_mix(bytes_u64(&slice[0..]) ^ RAPID_SECRET[2], bytes_u64(&slice[8..]) ^ seed ^ RAPID_SECRET[1]);
                    if slice.len() > 32 {
                        seed = rapid_mix(bytes_u64(&slice[16..]) ^ RAPID_SECRET[2], bytes_u64(&slice[24..]) ^ seed);
                    }
                }

                a = bytes_u64(&slice[slice.len()-16..]);
                b = bytes_u64(&slice[slice.len()-8..]);
            }

            a ^= RAPID_SECRET[1];
            b ^= seed;

            rapid_mum(&mut a, &mut b);
            rapid_mix(a ^ RAPID_SECRET[0] ^ data.len() as u64, b ^ RAPID_SECRET[1])
        }

        #[inline]
        pub fn rapidhash(data: &[u8]) -> u64 {
            rapidhash_internal(data, RAPID_SEED)
        }

        #[inline]
        pub fn rapidhash_seeded(data: &[u8], seed: u64) -> u64 {
            rapidhash_internal(data, seed)
        }

        #[inline(always)]
        #[allow(dead_code)]
        fn bytes_u64(slice: &[u8]) -> u64 {
            let mut buf: [u8; 8] = Default::default();
            buf.copy_from_slice(&slice[..8]);
            u64::from_le_bytes(buf)
        }

        #[inline(always)]
        fn bytes_u32(slice: &[u8]) -> u32 {
            let mut buf: [u8; 4] = Default::default();
            buf.copy_from_slice(&slice[..4]);
            u32::from_le_bytes(buf)
        }
    }
}
