pub fn add_jitter(ms: u64) -> u64 {
    let jitter = (rand::random::<i64>() % 100) - 50;
    (ms as i64 + jitter) as u64
}

pub fn get_plausible_typing_time(text: String) -> u64 {
    let words_per_minute = rand::random_range(30..40);
    let seconds_per_word = 60.0 / words_per_minute as f64;
    let words = text.split_whitespace().count();
    (0..words)
        .map(|_| add_jitter((seconds_per_word * 1000.0) as u64))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_plausible_typing_time() {
        let text = "Mijn lievelingsdier is een pissebed, want ze zijn zo schattig en ze rollen zich op als je ze aanraakt. Ik vind ze echt heel leuk. Ze zijn ook heel nuttig voor het milieu, want ze helpen bij het afbreken van organisch materiaal.";
        let typing_time = get_plausible_typing_time(text.to_string());
        println!("Typing time: {}", typing_time);
        assert!(typing_time > 0);
    }
}
