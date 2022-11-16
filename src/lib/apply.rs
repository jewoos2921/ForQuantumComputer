static STATE_T: Vec<u128> = vec![];

fn hash_64(key: state_t, width: i64) -> u64 {
    let mut k32: u64 = (key & 0xFFFFFFFF) ^ (key >> 32);
    k32 *= 0x9e370001usize;
    k32 = k32 >> (32 - width);
    k32
}

