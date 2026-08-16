use std::collections::HashMap;

fn count_words(text: &str) -> HashMap<&str, u32> {
    
    let mut freq: HashMap<&str, u32> = HashMap::new();

    for word in text.split_whitespace(){
        *freq.entry(word).or_insert(0) += 1;
    }   

    return freq;
}

fn main(){

    let text: &str = "rust is fast and rust is safe";
    println!("{:?}",count_words(text));

}