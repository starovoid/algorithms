use rand::{thread_rng, Rng};

/// Реализация функции RandomizeArray
/// Время: O(N), доп. память: O(1)
#[allow(dead_code)]
fn randomize_vec(data: &mut Vec<i32>) {
    let mut rng = thread_rng();
    for i in 0..data.len() {
        let j = rng.gen_range(i..data.len());
        data.swap(i, j);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_randomize_vec() {
        let mut data: Vec<i32> = (0..20).collect();
        randomize_vec(&mut data);
        println!("{:?}", data);
    }
}