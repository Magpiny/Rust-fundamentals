pub fn simple_interest(principal: f32, rate: f32, time: f32) -> f32 {
    let si = (principal * rate * time) / 100.00;
    si
}
