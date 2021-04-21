use std::error::Error;
use std::io;
use std::collections::HashMap;

/// Each program must read from standard input and print the frequencies of unique, space-separated
/// words, in order from most frequent to least frequent. To keep our solutions simple and
/// consistent, here are the (self-imposed) constraints:
/// 1. normalize words to lower case
/// 2. assume words will not have punctuation
/// 3. assume ASCII only characters
/// 4. if frequency of two words is the same, then output in any order
/// 5. single threaded
/// 6. buffer line by line

//type Entry<'a> = (&'a str, &'a usize);

fn main() {

    if let Err(e) = word_count() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}

fn read_from_stdio() -> Result<String, Box<dyn Error>> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line)
}

fn word_count() -> Result<(), Box<dyn Error>> {
    let mut hash_map: HashMap<String, u64> = HashMap::new();

    // keep reading lines until empty line entered
    loop {
        let line = read_from_stdio()?;
        if line == "\n" {
            break;
        } else {
            // tokenize words and add them to hash map
            for word in line.split_ascii_whitespace() {
                let norm_word = word.to_ascii_lowercase();
                let word_count = hash_map.entry(norm_word).or_insert(0);
                *word_count += 1;
            }
        }
    }
    // iterate over hashMap K,V pairs and store in vector
    let mut entries: Vec<(&String, &usize)> = hash_map
        .iter()
        .collect();

    // sort the vector by word count in descending order
    entries.sort_by(|&(_, count1), &(_, count2)|
        count2.partial_cmp(count1).unwrap());

    // print out the details
    for (word, count) in entries {
        println!("{} {}", word, count);
    }

    Ok(())
}
