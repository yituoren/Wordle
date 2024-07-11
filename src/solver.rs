use crate::{builtin_words::ACCEPTABLE, game::{cmp_val, Word}};
use std::{collections::{BinaryHeap, HashMap}, vec};
use std::cmp::{Ordering, min};

//单步最优
pub fn help(record: &Vec<String>, full_result: &Vec<[u8; 5]>) -> (Vec<(String, f64)>, Vec<(String, f64)>)
{
    let mut info: Vec<(String, f64)> = Vec::new();
    let mut possible_answers: Vec<String> = Vec::new();

    //筛选可能答案
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

    //计算可选词信息熵
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
        info.push((possible_guess.to_uppercase(), entropy));
    }

    //排序
    info.sort_by(|a, b|cmp_val(a, b));

    //筛选其中是可能答案的
    let help: Vec<(String, f64)> = info.iter().filter(|x| possible_answers.contains(&x.0)).cloned().collect();
    (info, help)
}

//计算复杂情况的信息熵
fn compute_entropy(words: &Vec<String>, possible_answers: &Vec<String>) -> f64
{
    let mut possibilities: HashMap<Vec<[u8; 5]>, usize> = HashMap::new();
    for possible_answer in possible_answers.iter()
    {
        let mut results: Vec<[u8; 5]> = Vec::new();
        for word in words.iter()
        {
            results.push(Word::new(&possible_answer).compare(word));
        }
        let tmp = possibilities.entry(results).or_insert(0);
        *tmp += 1;
    }
    let mut entropy: f64 = 0.0;
    for p in possibilities.iter()
    {
        let tmp: f64 = *p.1 as f64 / possible_answers.len() as f64;
        entropy += tmp * (-tmp.log2());
    }
    entropy
}

#[derive(Debug)]
struct Path
{
    words: Vec<String>,
    entropy: f64,
}

impl PartialEq for Path
{
    fn eq(&self, other: &Self) -> bool 
    {
        self.entropy == other.entropy
    }
}

impl Eq for Path {}

impl PartialOrd for Path 
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering
    {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

//全局最优
pub fn solve(record: &Vec<String>, full_result: &Vec<[u8; 5]>, time: i32) -> Vec<(String, f64)>
{
    let mut possible_answers: Vec<String> = Vec::new();

    //实现优先队列
    let mut solve: BinaryHeap<Path> = BinaryHeap::new();

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

    //最后几步，采用单步最优即可
    if possible_answers.len() <= 5 || time <= 1
    {
        let (_, help) = help(record, full_result);
        return help;
    }

    for possible_guess in ACCEPTABLE
    {
        if record.contains(&possible_guess.to_uppercase())
        {
            continue;
        }
        let new_words = vec![possible_guess.to_uppercase()];
        solve.push(Path{ words: new_words.clone(), entropy: compute_entropy(&new_words, &possible_answers) });
        if solve.len() > 20
        {
            solve.pop();
        }
    }
    let t = min(time, 3);
    for _i in 1..t
    {
        let mut new: BinaryHeap<Path> = BinaryHeap::new();
        for tmp_path in solve.iter()
        {
            println!("HERE");
            for possible_guess in ACCEPTABLE
            {
                if record.contains(&possible_guess.to_uppercase()) || tmp_path.words.contains(&possible_guess.to_uppercase())
                {
                    continue;
                }
                let mut new_words = tmp_path.words.clone();
                new_words.push(possible_guess.to_uppercase());
                new.push(Path{ words: new_words.clone(), entropy: compute_entropy(&new_words, &possible_answers) });
                if new.len() > 20
                {
                    new.pop();
                }
            }
        }
        solve = new;
    }

    let mut info: Vec<(String, f64)> = Vec::new();
    while let Some(path) = solve.pop()
    {
        info.push((path.words.first().unwrap().clone(), path.entropy));
    }
    info.reverse();
    info
}