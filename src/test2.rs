//将给定字符串转换为Pig Latin格式。在这个格式中，每个单词的
//第一个辅音字母会被移动到单词的结尾并增加“ay”后缀，例如“first”就会变为“irst-fay”。
//元音字母开头的单词则需要在结尾拼接上“hay”
//（例如，“apple”就会变为“apple-hay”）。
//要牢记我们讨论的关于UTF-8编码的内容！

enum WordKind {
    Vowel,
    Consonant,
}

fn main() {
    let apple = String::from("apple");
    let first = String::from("first");

    println!("Apple: {}", pig_latin(apple));
    println!("First : {}", pig_latin(first));
}

fn pig_latin(mut letter: String) -> String {
    let first_letter = letter.split_off(1);
    let vowel = [
        String::from("a"),
        String::from("e"),
        String::from("i"),
        String::from("o"),
        String::from("u"),
    ];

    let mut kind = WordKind::Consonant;
    if vowel.contains(&letter) {
        kind = WordKind::Vowel;
    };

    match kind {
        WordKind::Consonant => {
            return format!("{}-{}ay", first_letter, letter);
        }
        WordKind::Vowel => {
            return format!("{}{}-hay", letter, first_letter);
        }
    }
}
