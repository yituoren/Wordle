use crate::builtin_words::{ACCEPTABLE, FINAL};
use std::collections::HashMap;
use text_io::read;



//判断猜测是否合法
fn guess_is_valid(guess: &str) -> Result<&str, &str>
{
    for word in ACCEPTABLE
    {
        if guess == *word {return Ok(guess);}
    }
    Err("INVALID")
}

//判断答案是否合法
fn answer_is_valid(answer: &str) -> Result<&str, &str>
{
    for word in FINAL
    {
        if answer == *word {return Ok(answer);}
    }
    Err("INVALID")
}

/*#[test]
fn test()
{
    assert!(guess_is_valid("aahed"));
    assert_eq!(answer_is_valid("aahed"), false);
}*/

//代表单词所有信息的单词结构体
struct Word
{
    origin: String,
    letter: HashMap<char, Vec<u8>>,
}

impl Word
{
    pub fn new(word: &str) -> Word
    {
        let mut tmp_word = Word
        {
            origin: word.to_string(),
            letter: HashMap::new(),
        };
        let mut count: u8 = 0;
        for tmp_letter in word.chars()
        {
            let tmp = tmp_word.letter.entry(tmp_letter).or_insert(Vec::new());
            tmp.push(count);
            count += 1;
        }
        tmp_word
    }

    //比较两个单词间的差异
    pub fn compare(&self, guess: &str)-> [u8; 5]
    {
        let mut tmp_result: [u8; 5] = [0; 5];
        let mut answer_map = self.letter.clone();
        let mut count: usize = 0;
        for i in guess.chars()
        {
            let tmp_answer_vec = answer_map.get_mut(&i);
            match tmp_answer_vec
            {
                Some(answer_vec) => 
                {
                    tmp_result[count] = 2;
                    let t = answer_vec.clone();
                    for (j, k ) in t.iter().enumerate()
                    {
                        if *k as usize == count
                        {
                            answer_vec.remove(j);
                            tmp_result[count] = 3;
                        }
                    }
                },
                None =>
                {
                    tmp_result[count] = 1;
                },
            }
            count += 1;
        }
        count = 0;
        for j in guess.chars()
        {
            if tmp_result[count] == 1 || tmp_result[count] == 3
            {
                count += 1;
                continue;
            }
            let answer_vec = answer_map.get_mut(&j).unwrap();
            if answer_vec.is_empty()
            {
                tmp_result[count] = 1;
                count += 1;
                continue;
            }
            answer_vec.pop();
            tmp_result[count] = 2;
            count += 1;
        }
        tmp_result
    }
}

//生成答案单词
fn gen_answer(word: &str) -> Result<Word, &str>
{
    let answer = answer_is_valid(&word)?;
    Ok(Word::new(&answer.to_uppercase()))
}

//生成猜测单词
fn gen_guess(word: &str) -> Result<Word, &str>
{
    let guess = guess_is_valid(&word)?;
    Ok(Word::new(&guess.to_uppercase()))
}

//从标准输入获取答案
fn std_answer() -> Word
{
    //println!("PLEASE CHOOSE AN ANSWER: ");
    let mut word: String = "".to_string();
    loop
    {
        match std::io::stdin().read_line(&mut word)
        {
            Ok(_) => (),
            Err(error) => println!("ERROR: {}", error),
        }
        match gen_answer(word.trim())
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                println!("{}", warning);
                //println!("PLEASE CHOOSE AN ANSWER: ");
            }
        }
        word = "".to_string();
    }
}

//从标准输入获取猜测
fn std_guess(time: u8) -> Word
{
    //println!("GUESS {} : ", time);
    let mut word: String = "".to_string();
    loop
    {
        match std::io::stdin().read_line(&mut word)
        {
            Ok(_) => (),
            Err(error) => println!("ERROR: {}", error),
        }
        match gen_guess(word.trim())
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                println!("{}", warning);
                //println!("GUESS {} : ", time);
            }
        }
        word = "".to_string();
    }
}

fn test_update_and_show(guess: &str, tmp_result: &[u8; 5], mut result: HashMap<char, u8>) -> (HashMap<char, u8>, bool)
{
    let mut is_correct: bool = true;
    let mut count: usize = 0;
    let mut show: String = String::new();
    for i in guess.chars()
    {
        let tmp = result.entry(i).or_insert(0);
        if tmp_result[count] > *tmp
        {
            *tmp = tmp_result[count];
        }
        match tmp_result[count]
        {
            3 => {show += "G";},
            2 =>
            {
                show += "Y";
                is_correct = false;
            },
            1 =>
            {
                show += "R";
                is_correct = false;
            },
            _ =>
            {
                show += "X";
                is_correct = false;
            }
        }
        count += 1;
    }
    show += " ";
    for i in "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars()
    {
        let tmp = result.entry(i).or_insert(0);
        match tmp
        {
            3 => {show += "G"},
            2 => {show += "Y"},
            1 => {show += "R"},
            _ => {show += "X"}
        }
    }
    println!("{}", show);
    (result, is_correct)
}

pub fn std_play()
{
    let answer: Word = std_answer();
    let mut result: HashMap<char, u8> = HashMap::new();
    let mut is_correct: bool = false;
    for i in 1..=6
    {
        let guess: Word = std_guess(i);
        (result, is_correct) = test_update_and_show(&guess.origin, &answer.compare(&guess.origin), result);
        if is_correct
        {
            println!("CORRECT {}", i);
            return;
        }
    }
    println!("FAILED {}", answer.origin);
}