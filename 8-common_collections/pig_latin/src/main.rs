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
    let mut pigified_text = String::new();
    while let Some(c) = chars.next() {
        let suffix = match c {
            'a' | 'e' | 'i' | 'o' | 'u' => {
                pigified_text.push(c);
                String::from("-hay")
            }
            'a'...'z' | 'A'...'Z' => {
                format!("-{}ay", c)
            }
            _ => {
                pigified_text.push(c);
                continue;
            }
        };
    
        while let Some(&c) = chars.peek() {
            match c {
                'a'...'z' | 'A'...'Z' => {
                    chars.next();
                    pigified_text.push(c);
                }
                _ => break,
            }
        }
    
        pigified_text += &suffix;
    }
    pigified_text
}
