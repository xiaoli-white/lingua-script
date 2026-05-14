use std::collections::HashMap;

fn once() -> HashMap<&'static str, f64> {
    let mut m = HashMap::new();
    m.insert("zero", 0.0); m.insert("one", 1.0); m.insert("two", 2.0);
    m.insert("three", 3.0); m.insert("four", 4.0); m.insert("five", 5.0);
    m.insert("six", 6.0); m.insert("seven", 7.0); m.insert("eight", 8.0);
    m.insert("nine", 9.0); m.insert("ten", 10.0);
    m.insert("eleven", 11.0); m.insert("twelve", 12.0); m.insert("thirteen", 13.0);
    m.insert("fourteen", 14.0); m.insert("fifteen", 15.0); m.insert("sixteen", 16.0);
    m.insert("seventeen", 17.0); m.insert("eighteen", 18.0); m.insert("nineteen", 19.0);
    m
}

fn tens() -> HashMap<&'static str, f64> {
    let mut m = HashMap::new();
    m.insert("twenty", 20.0); m.insert("thirty", 30.0);
    m.insert("forty", 40.0); m.insert("fifty", 50.0);
    m.insert("sixty", 60.0); m.insert("seventy", 70.0);
    m.insert("eighty", 80.0); m.insert("ninety", 90.0);
    m
}

fn scales() -> HashMap<&'static str, f64> {
    let mut m = HashMap::new();
    m.insert("hundred", 100.0);
    m.insert("thousand", 1000.0);
    m.insert("million", 1_000_000.0);
    m.insert("billion", 1_000_000_000.0);
    m
}

pub fn is_number_word(word: &str) -> bool {
    let word = word.to_lowercase();
    once().contains_key(word.as_str())
        || tens().contains_key(word.as_str())
        || scales().contains_key(word.as_str())
        || word == "half"
        || word == "quarter"
}

pub fn parse_english_number(words: &[String]) -> Option<f64> {
    if words.is_empty() {
        return None;
    }

    if words.len() == 1 {
        let w = words[0].to_lowercase();
        if w == "half" { return Some(0.5); }
        if w == "quarter" { return Some(0.25); }
    }

    let once_map = once();
    let tens_map = tens();
    let scales_map = scales();

    let mut total = 0.0;
    let mut current = 0.0;
    let mut fractional = false;
    let mut fraction_divisor = 10.0;

    for word in words {
        let w = word.to_lowercase();

        if w == "point" {
            fractional = true;
            fraction_divisor = 10.0;
            continue;
        }

        if fractional {
            if let Some(digit) = once_map.get(w.as_str()) {
                current = current + digit / fraction_divisor;
                fraction_divisor *= 10.0;
            } else {
                return None;
            }
            continue;
        }

        if let Some(val) = once_map.get(w.as_str()) {
            current += val;
        } else if let Some(val) = tens_map.get(w.as_str()) {
            current += val;
        } else if let Some(&scale) = scales_map.get(w.as_str()) {
            if scale >= 1000.0 {
                if current == 0.0 { current = 1.0; }
                total += current * scale;
                current = 0.0;
            } else {
                current *= scale;
            }
        } else if w == "and" {
            continue;
        } else {
            return None;
        }
    }

    total += current;
    if total == 0.0 && current == 0.0 && fractional {
        total = current;
    }
    Some(total)
}
