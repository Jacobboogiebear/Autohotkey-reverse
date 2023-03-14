pub fn compare_bytes(raw_buffer: &mut Vec<u8>, cmp: Vec<u8>, index: usize) -> bool {
    let mut ret = true;
    let root = &raw_buffer[index..cmp.len() + index];
    let cmp_root = &cmp[0..cmp.len()];
    for i in 0..cmp.len() {
        if root[i] != cmp_root[i] {
            ret = false;
        }
    }
    return ret;
}

pub fn vec_get_u8(raw_buffer: &mut Vec<u8>, index: usize) -> Option<u8> {
    if raw_buffer.len() >= index + 1 {
        let mut ret: u8 = 0;
        ret = raw_buffer.get(index).unwrap().clone();
        return Some(ret);
    } else {
        return None;
    }
}

pub fn vec_get_u16(raw_buffer: &mut Vec<u8>, index: usize) -> Option<u16> {
    if raw_buffer.len() >= index + 2 {
        let mut ret: u16 = 0;
        let root = &raw_buffer[index..index + 2];
        ret = root[0] as u16;
        ret = ret | ((root[1] as u16) << 8);
        return Some(ret);
    } else {
        return None;
    }
}

pub fn vec_get_u32(raw_buffer: &mut Vec<u8>, index: usize) -> Option<u32> {
    if raw_buffer.len() >= index + 2 {
        let mut ret: u32 = 0;
        let root = &raw_buffer[index..index + 4];
        ret = root[0] as u32;
        ret = ret | ((root[1] as u32) << 8);
        ret = ret | ((root[2] as u32) << 16);
        ret = ret | ((root[3] as u32) << 24);
        return Some(ret);
    } else {
        return None;
    }
}

pub fn vec_get_u64(raw_buffer: &mut Vec<u8>, index: usize) -> Option<u64> {
    if raw_buffer.len() >= index + 2 {
        let mut ret: u64 = 0;
        let root = &raw_buffer[index..index + 8];
        ret = root[0] as u64;
        ret = ret | ((root[1] as u64) << 8);
        ret = ret | ((root[2] as u64) << 16);
        ret = ret | ((root[3] as u64) << 24);
        ret = ret | ((root[4] as u64) << 32);
        ret = ret | ((root[5] as u64) << 40);
        ret = ret | ((root[6] as u64) << 48);
        ret = ret | ((root[7] as u64) << 56);
        return Some(ret);
    } else {
        return None;
    }
}

pub fn vec_get_string_nnt(raw_buffer: &mut Vec<u8>, index: usize, len: usize) -> Option<String> {
    if raw_buffer.len() >= index + len {
        let mut ret = String::new();
        let root = &raw_buffer[index..index + len];
        for i in root {
            let ch = i.clone() as char;
            ret.push(ch);
        }
        return Some(ret.trim_matches(char::from(0x00)).to_string());
    } else {
        return None;
    }
}

pub fn vec_get_string_ct(raw_buffer: &mut Vec<u8>, index: usize, ch: char) -> Option<String> {
    if raw_buffer.len() >= index + 1 {
        let mut ret = String::new();
        let lp = &raw_buffer[index..];
        for i in lp {
            if i.clone() as char == ch {
                return Some(ret);
            } else {
                ret.push(i.clone() as char);
            }
        }
        return Some(ret);
    } else {
        return None;
    }
}

pub fn vec_slide_search(raw_buffer: &mut Vec<u8>, search: &Vec<u8>) -> Option<usize> {
    let original_comparison = &search[..];
    for i in 0..raw_buffer.len() - (search.len() - 1) {
        let comparison = &raw_buffer[i..search.len() + i];
        if comparison == original_comparison {
            return Some(i);
        }
    }
    return None;
}