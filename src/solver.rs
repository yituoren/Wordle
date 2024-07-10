use crate::{builtin_words::ACCEPTABLE, game::{self, cmp_val, Word}};
use std::collections::HashMap;

//单步最优
pub fn help(record: &Vec<String>, full_result: &Vec<[u8; 5]>) -> (Vec<(String, f64)>, Vec<(String, f64)>)
{
    let mut statics: HashMap<String, f64> = HashMap::new();
    let mut possible_answers: Vec<String> = Vec::new();

    for possible_answer in ACCEPTABLE
    {
        let mut count: usize = 0;
        let mut possible: bool = true;
        for guess in record.iter()
        {
            if Word::new(&possible_answer.to_uppercase()).compare(&guess) != full_result[count]
            {
                possible = false;
                break;
            }
            count += 1;
        }
        if possible
        {
            possible_answers.push(possible_answer.to_uppercase());
        }
    }

    for possible_guess in ACCEPTABLE
    {
        if record.contains(&possible_guess.to_uppercase())
        {
            continue;
        }
        let mut possibilities: HashMap<[u8; 5], usize> = HashMap::new();
        for possible_answer in possible_answers.iter()
        {
            let tmp = possibilities.entry(Word::new(&possible_answer).compare(&possible_guess.to_uppercase())).or_insert(0);
            *tmp += 1;
        }

        let mut entropy: f64 = 0.0;
        for p in possibilities.iter()
        {
            let tmp: f64 = *p.1 as f64 / possible_answers.len() as f64;
            entropy += tmp * (-tmp.log2());
        }
        statics.insert(possible_guess.to_uppercase(), entropy);
    }

    let mut info: Vec<(String, f64)> = statics.into_iter().collect();
    info.sort_by(|a, b|cmp_val(a, b));

    let help: Vec<(String, f64)> = info.iter().filter(|x| possible_answers.contains(&x.0)).cloned().collect();
    (info, help)
}

//全局最优
pub fn solve(record: &Vec<String>, full_result: &Vec<[u8; 5]>, time: i32) -> Vec<(String, f64)>
{
    let mut possible_answers: Vec<String> = Vec::new();
    let mut solve: Vec<(Vec<String>, f64)> = Vec::new();

    for possible_answer in ACCEPTABLE
    {
        let mut count: usize = 0;
        let mut possible: bool = true;
        for guess in record.iter()
        {
            if Word::new(&possible_answer.to_uppercase()).compare(&guess) != full_result[count]
            {
                possible = false;
                break;
            }
            count += 1;
        }
        if possible
        {
            possible_answers.push(possible_answer.to_uppercase());
        }
    }

    
    Vec::new()
}