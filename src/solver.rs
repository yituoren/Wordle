use crate::{builtin_words::{ACCEPTABLE, FINAL}, game::{self, cmp_val, Word}};
use std::{collections::{BinaryHeap, HashMap}, vec};
use std::cmp::{Ordering, min};
use rayon::prelude::*;
use std::sync::Arc;

//单步最优
pub fn help(record: &Vec<String>, full_result: &Vec<[u8; 5]>) -> (Vec<(String, f64)>, Vec<(String, f64)>) {

    //寻找可能答案
    let possible_answers: Vec<String> = ACCEPTABLE
        .par_iter()
        .filter_map(|&possible_answer| {
            let uppercased = possible_answer.to_uppercase();
            let mut count: usize = 0;
            let mut possible = true;
            for guess in record.iter() {
                if Word::new(&uppercased).compare(&guess) != full_result[count] {
                    possible = false;
                    break;
                }
                count += 1;
            }
            if possible {
                Some(uppercased)
            } else {
                None
            }
        })
        .collect();

    let possible_answers_arc = Arc::new(possible_answers);
    let possible_answers_len = possible_answers_arc.len() as f64;

    //对所有候选词进行遍历计算信息熵
    let mut info_sorted: Vec<(String, f64)> = ACCEPTABLE
        .par_iter()
        .filter_map(|&possible_guess| {
            let uppercased_guess = possible_guess.to_uppercase();
            if record.contains(&uppercased_guess) {
                return None;
            }
            let mut possibilities: HashMap<[u8; 5], usize> = HashMap::new();
            for possible_answer in possible_answers_arc.iter() {
                let tmp = possibilities.entry(Word::new(&possible_answer).compare(&uppercased_guess)).or_insert(0);
                *tmp += 1;
            }

            let entropy: f64 = possibilities
                .values()
                .map(|&count| {
                    let tmp = count as f64 / possible_answers_len;
                    tmp * (-tmp.log2())
                })
                .sum();

            Some((uppercased_guess, entropy))
        })
        .collect();

    //排序
    info_sorted.par_sort_by(|a, b| cmp_val(a, b));

    //筛选出可能是答案的
    let help: Vec<(String, f64)> = info_sorted
        .par_iter()
        .filter(|x| possible_answers_arc.contains(&x.0))
        .cloned()
        .collect();

    (info_sorted, help)
}

//计算复杂熵的两个函数

//优化后
fn compute_entropy(words: &Vec<String>, possible_answers: &Vec<String>) -> f64 {
    let possibilities: HashMap<Vec<[u8; 5]>, usize> = possible_answers
        .par_iter()
        .map(|possible_answer| {
            let results: Vec<[u8; 5]> = words
                .iter()
                .map(|word| Word::new(&possible_answer).compare(word))
                .collect();
            results
        })
        .fold(HashMap::new, |mut acc, results| {
            *acc.entry(results).or_insert(0) += 1;
            acc
        })
        .reduce(HashMap::new, |mut acc, map| {
            for (key, value) in map {
                *acc.entry(key).or_insert(0) += value;
            }
            acc
        });

    let entropy: f64 = possibilities
        .iter()
        .map(|(_, count)| {
            let p = *count as f64 / possible_answers.len() as f64;
            p * (-p.log2())
        })
        .sum();

    entropy
} 

//优化前
/*fn compute_entropy(words: &Vec<String>, possible_answers: &Vec<String>) -> f64
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
}*/

#[derive(Debug, Clone)]
struct Path {
    words: Vec<String>,
    entropy: f64,
}

impl PartialEq for Path {
    fn eq(&self, other: &Self) -> bool {
        self.entropy == other.entropy
    }
}

impl Eq for Path {}

impl PartialOrd for Path {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.entropy.partial_cmp(&self.entropy)
    }
}

impl Ord for Path {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

//全局最优
pub fn solve(record: &Vec<String>, full_result: &Vec<[u8; 5]>, time: i32) -> Vec<(String, f64)> {
    let mut possible_answers: Vec<String> = Vec::new();

    //使用并行计算筛选可能的答案
    possible_answers = ACCEPTABLE.par_iter()
        .filter_map(|possible_answer| {
            let mut count: usize = 0;
            let mut possible: bool = true;
            for guess in record.iter() {
                if Word::new(&possible_answer.to_uppercase()).compare(&guess) != full_result[count] {
                    possible = false;
                    break;
                }
                count += 1;
            }
            if possible {
                Some(possible_answer.to_uppercase())
            } else {
                None
            }
        })
        .collect();

    //最后一步用单步最优即可    
    if possible_answers.len() <= 5 || time <= 1 {
        let (_, help) = help(record, full_result);
        return help;
    }

    //使用并行计算来初始化优先队列
    let mut solve: BinaryHeap<Path> = ACCEPTABLE.par_iter()
        .filter(|&&possible_guess| !record.contains(&possible_guess.to_uppercase()))
        .map(|&possible_guess| {
            let new_words = vec![possible_guess.to_uppercase()];
            Path {
                words: new_words.clone(),
                entropy: compute_entropy(&new_words, &possible_answers),
            }
        })
        .collect();

    //只保留前10个最优路径
    while solve.len() > 10 {
        solve.pop();
    }

    //迭代后续情况并进行减枝
    let t = min(time, 3);
    for _i in 1..t {
        let mut new_heap: BinaryHeap<Path> = solve.par_iter()
            .flat_map(|tmp_path| {
                ACCEPTABLE.par_iter()
                    .filter(|&&possible_guess| {
                        !record.contains(&possible_guess.to_uppercase()) &&
                        !tmp_path.words.contains(&possible_guess.to_uppercase())
                    })
                    .map(|&possible_guess| {
                        let mut new_words = tmp_path.words.clone();
                        new_words.push(possible_guess.to_uppercase());
                        Path {
                            words: new_words.clone(),
                            entropy: compute_entropy(&new_words, &possible_answers),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        // 只保留前10个最优路径
        while new_heap.len() > 10 {
            new_heap.pop();
        }
        solve = new_heap;
    }

    let mut info: Vec<(String, f64)> = Vec::new();
    while let Some(path) = solve.pop() {
        info.push((path.words.first().unwrap().clone(), path.entropy));
    }
    info.reverse();
    info
}

/*pub fn solve(record: &Vec<String>, full_result: &Vec<[u8; 5]>, time: i32) -> Vec<(String, f64)>
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
        if solve.len() > 10
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
                if new.len() > 10
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
}*/

//测试函数
pub fn test()
{
    let mut count = 0;
    let mut steps = 0;
    for answers in FINAL
    {
        let answer = Word::new(&answers.to_uppercase());
        println!("{}", answer.origin);
        let mut record: Vec<String> = Vec::new();
        let mut full_result: Vec<[u8; 5]> = Vec::new();
        let mut best_result: HashMap<char, u8> = HashMap::new();
        let mut is_correct: bool = false;
        record.push("TARES".to_string());
        full_result.push(answer.compare("TARES"));
        (best_result, is_correct) = game::user_update_and_show(&record, &full_result, best_result);
        if is_correct
        {
            count += 1;
            steps += 1;
            println!("{} {}", count, steps);
            continue;
        }
        for i in 2..=6
        {
            let help = solve(&record, &full_result, 6 - i);
            record.push(help[0].0.clone());
            full_result.push(answer.compare(&help[0].0));
            (best_result, is_correct) = game::user_update_and_show(&record, &full_result, best_result);
            if is_correct
            {
                count += 1;
                steps += i;
                println!("{} {} {}", count, i, steps);
                break;
            }
        }
        if !is_correct
        {
            count += 1;
            steps += 7;
            println!("{} {} {}", count, 7, steps);//失败时认为步数为7
        }
    }
}