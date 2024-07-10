use console;
use game::Word;
use std::{collections::HashMap, fmt, io::{self, Write}};

mod arg;
mod game;
mod builtin_words;
mod file;
mod solver;

#[derive(Debug)]
struct MyError
{
    source: String,
}
impl fmt::Display for MyError
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result
    {
        write!(f, "{}", self.source)
    }
}
impl std::error::Error for MyError{}


/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let is_tty = atty::is(atty::Stream::Stdout);

    /*if is_tty {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
    } else {
        println!("I am not in a tty. Please print according to test requirements!");
    }

    if is_tty {
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
    }
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    println!("Welcome to wordle, {}!", line.trim());*/

    //处理命令行
    let mut cmd = match arg::process_arg()
    {
        Ok(tmp) => tmp,
        Err(error) => return Err(Box::new(MyError{ source: error, }))
    };
    if !arg::arg_is_valid(&cmd)
    {
        return Err(Box::new(MyError{ source: "INVALID COMMAND LINE LOGIC".to_string(), }));
    }

    //全局参数
    let mut answer: game::Word = Word::new("");
    let mut guess: game::Word = Word::new("");
    let mut again: bool = true;
    let mut total_round: i32 = 0;
    let mut success_round: i32 = 0;
    let mut success_try: i32 = 0;
    let mut total_word: HashMap<String, i32> = HashMap::new();
    let mut difficult: bool = false;
    let mut game_data: file::GameData = file::GameData
    {
        total_rounds: 0,
        games: Vec::new(),
    };
    if let Some(_i) = cmd.mode.get("difficult")
    {
        difficult = true;
    }
    let mut day: u64 = 1;
    let mut seed: u64 = 1;
    if let Some(j) = cmd.value.get("day")
    {
        day = *j;
    }
    if let Some(k) = cmd.value.get("seed")
    {
        seed = *k;
    }

    //读取游戏存档
    if let Some(state) = cmd.info.get("state")
    {
        if let Ok(tmp) = file::read_state(state)
        {
            game_data = tmp;
            total_round = game_data.total_rounds;
            if game_data.total_rounds as usize != game_data.games.len()
            {
                return Err(Box::new(MyError{source: "INVALID STATE".to_string()}));
            }
            for i in game_data.games.iter()
            {
                if i.guesses.len() > 6
                {
                    return Err(Box::new(MyError{source: "INVALID STATE".to_string()}));
                }
                match i.guesses.last()
                {
                    Some(j) =>
                    {
                        if i.answer == *j
                        {
                            success_round += 1;
                            success_try += i.guesses.len() as i32;
                        }
                    }
                    None => return Err(Box::new(MyError{source: "INVALID STATE".to_string()})),
                }
                for j in i.guesses.iter()
                {
                    let count = total_word.entry(j.to_string()).or_insert(0);
                    *count += 1;
                }
            }
        }
        else
        {
            return Err(Box::new(MyError{source: "INVALID STATE".to_string()}));
        }
    }

//主体
if !is_tty//测试模式
{
while again
{
    total_round += 1;

    //生成答案
    if let Some(_i) = cmd.mode.get("random")
    {
        match game::random_answer(&mut cmd.answer_file, &mut day, &seed)
        {
            Ok(tmp) => answer = tmp,
            Err(tmp) => return Err(Box::new(MyError{source: tmp}))
        }
    }
    else
    {
        if let Some(i) = cmd.info.get("word")
        {
            match game::gen_answer(i, &cmd.answer_file)
            {
                Ok(tmp) => answer = tmp,
                Err(_tmp) => return Err(Box::new(MyError{source: "INVALID WORD".to_string()})),
            }
            again = false;
        }
        else
        {
            answer = game::std_answer(&cmd.answer_file);
        }
    }

    //记录每局信息
    let mut record: Vec<String> = Vec::new();
    let mut this_round: file::Round = file::Round { answer: answer.origin.clone(), guesses: Vec::new() };

    //开始猜测
    let mut result: HashMap<char, u8> = HashMap::new();
    let mut is_correct: bool = false;
    let mut tmp_result: [u8; 5] = [0; 5];
    for i in 1..=6
    {
        guess = game::std_guess(&cmd.guess_file, &tmp_result, &record, &difficult);

        let tmp = guess.origin.clone();
        record.push(tmp.clone());
        this_round.guesses.push(tmp.clone());
        let count = total_word.entry(tmp.clone()).or_insert(0);
        *count += 1;

        tmp_result = answer.compare(&guess.origin);
        (result, is_correct) = game::test_update_and_show(&guess.origin, &tmp_result, result);
        if is_correct
        {
            println!("CORRECT {}", i);
            success_try += i;
            success_round += 1;
            break;
        }
    }
    if !is_correct
    {
        println!("FAILED {}", answer.origin);
    }
    //更新游戏存档
    game_data.games.push(this_round);
    game_data.total_rounds += 1;

    //打印数据
    if let Some(_i) = cmd.mode.get("stats")
    {
        let mut average: f32 = 0.0;
        if success_round != 0
        {
            average = (success_try as f32) / (success_round as f32);
        }
        println!("{} {} {:.2}", success_round, total_round - success_round, average);
        let mut vec: Vec<(&String, &i32)> = total_word.iter().collect();
        vec.sort_by(|a, b| game::cmp_ref(b, a));
        let end = if vec.len() < 5 { vec.len() } else { 5 };
        for i in 0..(end - 1)
        {
            print!("{} {} ", vec[i].0, vec[i].1);
        }
        println!("{} {}", vec[end - 1].0, vec[end - 1].1);
    }

    //是否再来
    if again
    {
        //println!("WANT ANOTHER ROUND? [Y / N]");
        let mut another = "".to_string();
        match std::io::stdin().read_line(&mut another)
        {
            Ok(0) => again = false,
            Err(error) => println!("{}", error),
            Ok(_) =>
            {
                match another.trim()
                {
                    a if a == "Y" || a == "y" => (),
                    b if b == "N" || b == "n" => again = false,
                    _ => return Err(Box::new(MyError{source: "INVALID INPUT".to_string()}))
                }
            }
        }
    }
}//while结束
}
else//交互模式
{
    //是否使用UI
    println!("WANT UI? [Y / N]");
    let mut ui = "".to_string();
    let mut is_ui = false;
    match std::io::stdin().read_line(&mut ui)
    {
        Ok(0) => is_ui = false,
        Err(error) => println!("{}", error),
        Ok(_) =>
        {
            match ui.trim()
            {
                a if a == "Y" || a == "y" => is_ui = true,
                b if b == "N" || b == "n" => is_ui = false,
                _ => return Err(Box::new(MyError{source: "INVALID INPUT".to_string()}))
            }
        }
    }

if is_ui
{
//do sth
}

else
{
    while again
    {
        total_round += 1;
    
        //生成答案
        if let Some(_i) = cmd.mode.get("random")
        {
            match game::random_answer(&mut cmd.answer_file, &mut day, &seed)
            {
                Ok(tmp) => answer = tmp,
                Err(tmp) => return Err(Box::new(MyError{source: tmp}))
            }
        }
        else
        {
            if let Some(i) = cmd.info.get("word")
            {
                match game::gen_answer(i, &cmd.answer_file)
                {
                    Ok(tmp) => answer = tmp,
                    Err(_tmp) => return Err(Box::new(MyError{source: "INVALID WORD".to_string()})),
                }
                again = false;
            }
            else
            {
                println!("CHOOSE AN ANSWER: ");
                answer = game::std_answer(&cmd.answer_file);
            }
        }
    
        //记录每局信息
        let mut record: Vec<String> = Vec::new();
        let mut this_round: file::Round = file::Round { answer: answer.origin.clone(), guesses: Vec::new() };
    
        //开始猜测
        let mut best_result: HashMap<char, u8> = HashMap::new();
        let mut is_correct: bool = false;
        let mut tmp_result: [u8; 5] = [0; 5];
        let mut full_result: Vec<[u8; 5]> = Vec::new();
        for i in 1..=6
        {
            let mut is_help = false;
            let mut help = "".to_string();
            println!("WANT SOME HELP? [Y / N]");
            match std::io::stdin().read_line(&mut help)
            {
                Ok(0) => is_help = false,
                Err(error) => println!("{}", error),
                Ok(_) =>
                {
                    match help.trim()
                    {
                        a if a == "Y" || a == "y" => is_help = true,
                        b if b == "N" || b == "n" => is_help = false,
                        _ => return Err(Box::new(MyError{source: "INVALID INPUT".to_string()}))
                    }
                }
            }
            if is_help
            {
                let (info, help) = solver::help(&record, &full_result);
                print!("THE MOST INFORMATIVE GUESSES ARE:");
                for i in 0..std::cmp::min(5 as usize, info.len())
                {
                    print!(" {}: {:.2}", info[i].0, info[i].1);
                }
                println!("");
                print!("THE BEST GUESSES ARE:");
                for i in 0..std::cmp::min(5 as usize, help.len())
                {
                    print!(" {}: {:.2}", help[i].0, help[i].1);
                }
                println!("");
            }
            println!("CHOOSE A GUESS:");
            guess = game::std_guess(&cmd.guess_file, &tmp_result, &record, &difficult);
    
            let tmp = guess.origin.clone();
            record.push(tmp.clone());
            this_round.guesses.push(tmp.clone());
            let count = total_word.entry(tmp.clone()).or_insert(0);
            *count += 1;
    
            tmp_result = answer.compare(&guess.origin);
            full_result.push(tmp_result);
            (best_result, is_correct) = game::test_update_and_show(&guess.origin, &tmp_result, best_result);
            if is_correct
            {
                println!("CORRECT {}", i);
                success_try += i;
                success_round += 1;
                break;
            }
        }
        if !is_correct
        {
            println!("FAILED {}", answer.origin);
        }
        //更新游戏存档
        game_data.games.push(this_round);
        game_data.total_rounds += 1;
    
        //打印数据
        if let Some(_i) = cmd.mode.get("stats")
        {
            let mut average: f32 = 0.0;
            if success_round != 0
            {
                average = (success_try as f32) / (success_round as f32);
            }
            println!("{} {} {:.2}", success_round, total_round - success_round, average);
            let mut vec: Vec<(&String, &i32)> = total_word.iter().collect();
            vec.sort_by(|a, b| game::cmp_ref(b, a));
            let end = if vec.len() < 5 { vec.len() } else { 5 };
            for i in 0..(end - 1)
            {
                print!("{} {} ", vec[i].0, vec[i].1);
            }
            println!("{} {}", vec[end - 1].0, vec[end - 1].1);
        }
    
        //是否再来
        if again
        {
            println!("WANT ANOTHER ROUND? [Y / N]");
            let mut another = "".to_string();
            match std::io::stdin().read_line(&mut another)
            {
                Ok(0) => again = false,
                Err(error) => println!("{}", error),
                Ok(_) =>
                {
                    match another.trim()
                    {
                        a if a == "Y" || a == "y" => (),
                        b if b == "N" || b == "n" => again = false,
                        _ => return Err(Box::new(MyError{source: "INVALID INPUT".to_string()}))
                    }
                }
            }
        }
    }//while结束
}
}

    //存档
    if let Some(state) = cmd.info.get("state")
    {
        if let Err(_) = file::write_state(&game_data, state)
        {
            return Err(Box::new(MyError{source: "FAILED TO WRITE STATE".to_string()}));
        }
    }

    Ok(())
}
