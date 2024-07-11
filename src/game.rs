use crate::builtin_words::{ACCEPTABLE, FINAL};
use std::{cmp::Ordering, collections::HashMap};
use rand::prelude::*;
use rand::rngs::StdRng;
use crossterm::{execute, style::{Color, Print, ResetColor, SetForegroundColor},};
use std::io::stdout;
use crossterm::event::{self, Event, KeyCode};
use std::io;
use tui::backend::Backend;
use tui::layout::{Constraint, Direction, Layout};
use tui::style::Style;
use tui::text::{Span, Spans};
use tui::widgets::{Block, Borders, Paragraph};
use tui::Terminal;

//判断猜测是否合法
pub fn guess_is_valid(guess: &str, guess_file: &Vec<String>) -> Result<String, String>
{
    if !guess_file.is_empty()
    {
        for word in guess_file.iter()
        {
            if guess == word
            {
                return Ok(guess.to_string())
            }
        }
        return Err("INVALID".to_string());

    }
    for word in ACCEPTABLE
    {
        if guess == word.to_uppercase() {return Ok(guess.to_string());}
    }
    Err("INVALID".to_string())
}

//判断答案是否合法
pub fn answer_is_valid(answer: &str, answer_file: &Vec<String>) -> Result<String, String>
{
    if !answer_file.is_empty()
    {
        for word in answer_file.iter()
        {
            if answer == word
            {
                return Ok(answer.to_string())
            }
        }
        return Err("INVALID".to_string());

    }
    for word in FINAL
    {
        if answer == word.to_uppercase() {return Ok(answer.to_string());}
    }
    Err("INVALID".to_string())
}

//代表单词所有信息的单词结构体
pub struct Word
{
    pub origin: String,
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
pub fn gen_answer(word: &str, answer_file: &Vec<String>) -> Result<Word, String>
{
    let answer = answer_is_valid(&word.to_uppercase(), answer_file)?;
    Ok(Word::new(&answer))
}

//生成猜测单词
pub fn gen_guess(word: &str, guess_file: &Vec<String>) -> Result<Word, String>
{
    let guess = guess_is_valid(&word.to_uppercase(), guess_file)?;
    Ok(Word::new(&guess))
}

//从标准输入获取答案
pub fn std_answer(answer_file: &Vec<String>) -> Word
{
    let mut word: String = "".to_string();
    loop
    {
        match std::io::stdin().read_line(&mut word)
        {
            Ok(_) => (),
            Err(error) => println!("ERROR: {}", error),
        }
        match gen_answer(word.trim(), answer_file)
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                println!("{}", warning);
            }
        }
        word = "".to_string();
    }
}

//从TUI获取答案
pub fn tui_answer<B: Backend>(terminal: &mut Terminal<B>, answer_file: &Vec<String>) -> Word
{
    let mut word: String = "".to_string();
    let mut output: String = String::new();
    output += "CHOOSE AN ANSWER\n";
    loop
    {
        match get_input_str(terminal, &output)
        {
            Ok(tmp) => word = tmp,
            Err(_) => 
            {
                word = "".to_string();
                continue;
            }
        }
        match gen_answer(word.trim(), answer_file)
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                output = output + &warning + " ANSWER\n";
                continue;
            }
        }
    }
}

//从标准输入获取猜测
pub fn std_guess(guess_file: &Vec<String>, tmp_result: &[u8; 5], record: &Vec<String>, difficult: &bool) -> Word
{
    let mut word: String = "".to_string();
    loop
    {
        match std::io::stdin().read_line(&mut word)
        {
            Ok(_) => (),
            Err(error) =>
            {
                println!("ERROR: {}", error);
                word = "".to_string();
                continue;
            }
        }
        word = word.to_uppercase().to_string();
        if *difficult && !record.is_empty()
        {
            let last: String = record.last().unwrap().to_string();
            let mut is_valid: bool = true;
            let mut count: usize = 0;
            for i in last.chars()
            {
                match tmp_result[count]
                {
                    3 =>
                    {
                        if word.chars().nth(count).unwrap() != i
                        {
                            is_valid = false;
                            break;
                        }
                    }
                    2 =>
                    {
                        match word.find(i)
                        {
                            Some(_) => (),
                            None =>
                            {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    1 | 0 => (),
                    _ =>
                    {
                        is_valid = false;
                        break;
                    }
                }
                count += 1;
            }
            if !is_valid
            {
                println!("INVALID");
                word = "".to_string();
                continue;
            }
        }
        match gen_guess(word.trim(), guess_file)
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                println!("{}", warning);
            }
        }
        word = "".to_string();
    }
}

//从TUI获取猜测
pub fn tui_guess<B: Backend>(terminal: &mut Terminal<B>, guess_file: &Vec<String>, tmp_result: &[u8; 5], record: &Vec<String>, difficult: &bool, output: &mut Vec<Spans>) -> Word
{
    let mut word: String = "".to_string();
    let mut str: String = String::new();
    loop
    {
        if word.is_empty()
        {
            match get_input_span(terminal, &output)
            {
                Ok(tmp) => word = tmp,
                Err(error) =>
                {
                    get_input_str(terminal, &error.to_string());
                    word = "".to_string();
                    continue;
                }
            }
        }
        word = word.to_uppercase().to_string();
        if *difficult && !record.is_empty()
        {
            let last: String = record.last().unwrap().to_string();
            let mut is_valid: bool = true;
            let mut count: usize = 0;
            for i in last.chars()
            {
                match tmp_result[count]
                {
                    3 =>
                    {
                        if word.chars().nth(count).unwrap() != i
                        {
                            is_valid = false;
                            break;
                        }
                    }
                    2 =>
                    {
                        match word.find(i)
                        {
                            Some(_) => (),
                            None =>
                            {
                                is_valid = false;
                                break;
                            }
                        }
                    }
                    1 | 0 => (),
                    _ =>
                    {
                        is_valid = false;
                        break;
                    }
                }
                count += 1;
            }
            if !is_valid
            {
                str += "INVALID GUESS\n";
                word = get_input_str(terminal, &str).unwrap();
                continue;
            }
        }
        match gen_guess(word.trim(), guess_file)
        {
            Ok(tmp) => break tmp,
            Err(warning) => 
            {
                str = str + &warning + " GUESS\n";
                word = get_input_str(terminal, &str).unwrap();
            }
        }
    }
}

//测试模式更新结果并输出
pub fn test_update_and_show(guess: &str, tmp_result: &[u8; 5], mut result: HashMap<char, u8>) -> (HashMap<char, u8>, bool)
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

//用户模式更新结果并输出
pub fn user_update_and_show(guesses: &Vec<String>, full_result: &Vec<[u8; 5]>, mut best_result: HashMap<char, u8>) -> (HashMap<char, u8>, bool)
{
    let mut is_correct: bool = true;
    let mut big_count: usize = 0;
    let mut color = Color::Black;
    let mut stdout = stdout();
    for guess in guesses.iter()
    {
    print!("     ");
    let mut small_count: usize = 0;
    for letter in guess.chars()
    {
        let tmp = best_result.entry(letter).or_insert(0);
        if full_result[big_count][small_count] > *tmp
        {
            *tmp = full_result[big_count][small_count];
        }
        match full_result[big_count][small_count]
        {
            3 => {color = Color::Green;},
            2 =>
            {
                color = Color::Yellow;
                if big_count == guesses.len() - 1 { is_correct = false; }
            },
            1 =>
            {
                color = Color::Red;
                if big_count == guesses.len() - 1 { is_correct = false; }
            },
            _ =>
            {
                color = Color::Black;
                if big_count == guesses.len() - 1 { is_correct = false; }
            }
        }
        execute!(stdout, SetForegroundColor(color), Print(letter), ResetColor, Print(" "));
        small_count += 1;
    }
    big_count += 1;
    println!("");
    }
    
    let keyboard = [
        "Q W E R T Y U I O P",
        " A S D F G H J K L",
        "  Z X C V B N M"
    ];

    for row in keyboard.iter()
    {
        for letter in row.chars()
        {
            match best_result.get(&letter)
            {
                Some(3) => color = Color::Green,
                Some(2) => color = Color::Yellow,
                Some(1) => color = Color::Red,
                Some(_) | None => color = Color::Black,
            }
            execute!(stdout, SetForegroundColor(color), Print(letter), ResetColor);
        }
        println!("");
    }
    (best_result, is_correct)
}

//TUI模式更新结果并输出
pub fn tui_update_and_show(guesses: &Vec<String>, full_result: &Vec<[u8; 5]>, mut best_result: HashMap<char, u8>, output: &mut Vec<Spans>) -> (HashMap<char, u8>, bool)
{
    let mut is_correct: bool = true;
    let mut big_count: usize = 0;
    let mut color = tui::style::Color::Black;
    for guess in guesses.iter()
    {
        let mut spans: Vec<Span> = Vec::new();
        spans.push(Span::raw("     "));
    let mut small_count: usize = 0;
    for letter in guess.chars()
    {
        let tmp = best_result.entry(letter).or_insert(0);
        if full_result[big_count][small_count] > *tmp
        {
            *tmp = full_result[big_count][small_count];
        }
        match full_result[big_count][small_count]
        {
            3 => {color = tui::style::Color::Green;},
            2 =>
            {
                color = tui::style::Color::Yellow;
                if big_count == guesses.len() - 1 { is_correct = false; }
            },
            1 =>
            {
                color = tui::style::Color::Red;
                if big_count == guesses.len() - 1 { is_correct = false; }
            },
            _ =>
            {
                color = tui::style::Color::Black;
                if big_count == guesses.len() - 1 { is_correct = false; }
            }
        }
        spans.push(Span::styled(format!("{} ", letter), Style::default().fg(color)));
        small_count += 1;
    }
    output.push(Spans::from(spans));
    big_count += 1;
    }
    
    let keyboard = [
        "Q W E R T Y U I O P",
        " A S D F G H J K L",
        "  Z X C V B N M"
    ];

    for row in keyboard.iter()
    {
        let mut spans: Vec<Span> = Vec::new();
        for letter in row.chars()
        {
            match best_result.get(&letter)
            {
                Some(3) => color = tui::style::Color::Green,
                Some(2) => color = tui::style::Color::Yellow,
                Some(1) => color = tui::style::Color::Red,
                Some(_) | None => color = tui::style::Color::Black,
            }
            spans.push(Span::styled(letter.to_string(), Style::default().fg(color)));
        }
        output.push(Spans::from(spans));
    }
    (best_result, is_correct)
}

//随机答案
pub fn random_answer(answer_file: &mut Vec<String>, day: &mut u64, seed: &u64) -> Result<Word, String>
{
    let mut rng = StdRng::seed_from_u64(*seed);
    if answer_file.is_empty()
    {
        let mut copy= FINAL.to_vec();
        copy.shuffle(&mut rng);
        if *day > copy.len() as u64
        {
            return Err("INVALID COMMAND LINE: DAY".to_string());
        }
        else
        {
            let word = copy[(*day - 1) as usize];
            *day = (*day + 1) % copy.len() as u64;
            return gen_answer(word, answer_file);
        }
    }
    else
    {
        let copy = answer_file.clone();
        answer_file.shuffle(&mut rng);
        if *day > answer_file.len() as u64
        {
            return Err("INVALID COMMAND LINE: DAY".to_string());
        }
        else
        {
            let word = answer_file[(*day - 1) as usize].clone();
            *answer_file = copy;
            *day = (*day + 1) % answer_file.len() as u64;
            return gen_answer(&word, answer_file);
        }
    }
}

//两个需要的比较函数
pub fn cmp_ref<T: Ord>(a: &(&String, &T), b: &(&String, &T)) -> Ordering
{
    if a.1 == b.1
    {
        return b.0.cmp(a.0);
    }
    else
    {
        return a.1.cmp(b.1);
    }
}

pub fn cmp_val<X, T: PartialOrd>(a: &(X, T), b: &(X, T)) -> Ordering
{
    if let Some(order) = b.1.partial_cmp(&a.1)
    {
        order
    }
    else
    {
        Ordering::Greater
    }
}

//TUI输出STRING的两个函数
fn draw_ui_str<B: Backend>(terminal: &mut Terminal<B>, input: &str, output: &str) -> Result<(), io::Error> 
{
    terminal.draw(|f| 
        {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        //输入框
        let input_widget = Paragraph::new(input)
            .style(Style::default().fg(tui::style::Color::Blue))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(input_widget, chunks[0]);

        //输出框
        let output_widget = Paragraph::new(output)
            .style(Style::default().fg(tui::style::Color::White))
            .block(Block::default().borders(Borders::ALL));
        f.render_widget(output_widget, chunks[1]);
    })?;

    Ok(())
}

pub fn get_input_str<B: Backend>(terminal: &mut Terminal<B>, output: &str) -> Result<String, io::Error>
{
    let mut input = String::new();
    loop
    {
        draw_ui_str(terminal, &input, output)?;

        if let Event::Key(event) = event::read()?//处理输入
        {
            match event.code
            {
                KeyCode::Char(c) => 
                {
                    if input.len() < 5 
                    {
                        input.push(c);
                    }
                }
                KeyCode::Backspace => 
                {
                    input.pop();
                }
                KeyCode::Enter => 
                {
                    break;
                }
                KeyCode::Esc => 
                {
                    panic!(); // Exit the game loop on Esc key
                }
                _ => {}
            }
        }    
    }
    Ok(input)
}

//TUI输出SPANS的两个函数
pub fn get_input_span<B: Backend>(terminal: &mut Terminal<B>, output: &Vec<Spans>) -> Result<String, io::Error>
{
    let mut input = String::new();
    loop
    {
        draw_ui_span(terminal, &input, output)?;

        if let Event::Key(event) = event::read()?//处理输入
        {
            match event.code
            {
                KeyCode::Char(c) => 
                {
                    if input.len() < 5 
                    {
                        input.push(c);
                    }
                }
                KeyCode::Backspace => 
                {
                    input.pop();
                }
                KeyCode::Enter => 
                {
                    break;
                }
                KeyCode::Esc => 
                {
                    panic!();
                }
                _ => {}
            }
        }    
    }
    Ok(input)
}

fn draw_ui_span<B: Backend>(terminal: &mut Terminal<B>, input: &str, output: &Vec<Spans>) -> Result<(), io::Error> 
{
    terminal.draw(|f| 
        {
        let size = f.size();
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints(
                [
                    Constraint::Length(3),
                    Constraint::Min(1),
                    Constraint::Length(3),
                ]
                .as_ref(),
            )
            .split(size);

        //输入框
        let input_widget = Paragraph::new(input)
            .style(Style::default().fg(tui::style::Color::Blue))
            .block(Block::default().borders(Borders::ALL).title("INPUT"));
        f.render_widget(input_widget, chunks[0]);

        //输出框
        let output_widget = Paragraph::new(output.clone())
            .style(Style::default().fg(tui::style::Color::White))
            .block(Block::default().borders(Borders::ALL).title("OUTPUT"));
        f.render_widget(output_widget, chunks[1]);
    })?;

    Ok(())
}