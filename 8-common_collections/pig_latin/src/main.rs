fn main() {
    println!("Hello, world!");
    let text = "The owls are not what they seem.".to_string();
    println!("Human says: {}", text);

    let pigified_text = convert_to_pig_latin(&text);
    println!("Pig says: {}", pigified_text);
}

fn convert_to_pig_latin(text: &str) -> String
{
    let mut chars = text.chars().peekable();
    let mut new_s = String::new();
    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                new_s.push(c);
                String::from("-hay")
            }
            'a'...'z' | 'A'...'Z' => {
                format!("-{}ay", c)
            }
            _ => {
                new_s.push(c);
                continue;
            }
        };
    
        while let Some(&c) = chars.peek() {
            match c {
                'a'...'z' | 'A'...'Z' => {
                    chars.next();
                    new_s.push(c);
                }
                _ => break,
            }
        }
    
        new_s += &suffix;
    }
    new_s
}
