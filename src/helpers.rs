
pub fn atoi(bytes : &[u8], base : u8) -> Option<usize>{
    if base > 16{
        None
    }
    else{
        let chars = "0123456789abcdef".as_bytes();
        
        Some(bytes.iter().rev().enumerate().skip(1).map(|(i,x)| {
                let mut ch = *x;
                ch.make_ascii_lowercase();
                let idx = chars.binary_search(&ch).unwrap();
                (base as usize).checked_pow(i.try_into().unwrap()).unwrap() * idx
            }
            ).sum()
        )
    }
    
}
