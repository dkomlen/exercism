pub fn find() -> Option<u32> {
    for i in 1..500 {
        let b = i as f32;
        let a = (500000. - 1000. * b) / (1000. - b);
        if a.fract() == 0. {
            let (x,y) = (a as u32, b as u32);
            return Some(x * y * (1000 - x - y))
        }
    }
    None
}
