use std::collections::{HashMap, HashSet};
use crate::builtin_words::FINAL;
use crate::file;

pub struct Command
{
    pub mode: HashSet<String>,
    pub value: HashMap<String, u64>,
    pub info: HashMap<String, String>,
    pub answer_file: Vec<String>,
    pub guess_file: Vec<String>,
}

pub fn process_arg() -> Result<Command, String>
{
    let mut args: HashSet<String> = HashSet::new();
    let mut argv: HashMap<String, u64> = HashMap::new();
    let mut argw: HashMap<String, String> = HashMap::new();
    let mut arg: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();
    let mut guess: Vec<String> = Vec::new();
    let mut iter = std::env::args();
    iter.next();
    while let Some(tmp) = iter.next()
    {
        if tmp == "-c" || tmp == "--config"
        {
            if let Some(config) = iter.next()
            {
                if let Some(i) = config.chars().nth(0)
                {
                    if i != '-'
                    {
                        match file::read_config(&config)
                        {
                            Ok(tmp_config) =>
                            {
                                if let Some(set) = tmp_config.random
                                {
                                    if set
                                    {
                                        args.insert("random".to_string());
                                    }
                                }
                                if let Some(set) = tmp_config.difficult
                                {
                                    if set
                                    {
                                        args.insert("difficult".to_string());
                                    }
                                }
                                if let Some(set) = tmp_config.stats
                                {
                                    if set
                                    {
                                        args.insert("stats".to_string());
                                    }
                                }
                                if let Some(set) = tmp_config.day
                                {
                                    args.insert("day".to_string());
                                    argv.insert("day".to_string(), set);
                                }
                                if let Some(set) = tmp_config.seed
                                {
                                    args.insert("seed".to_string());
                                    argv.insert("seed".to_string(), set);
                                }
                                if let Some(set) = tmp_config.final_set
                                {
                                    args.insert("final_set".to_string());
                                    argw.insert("final_set".to_string(), set);
                                }
                                if let Some(set) = tmp_config.acceptable_set
                                {
                                    args.insert("acceptable_set".to_string());
                                    argw.insert("acceptable_set".to_string(), set);
                                }
                                if let Some(set) = tmp_config.state
                                {
                                    args.insert("state".to_string());
                                    argw.insert("state".to_string(), set);
                                }
                                if let Some(set) = tmp_config.word
                                {
                                    args.insert("word".to_string());
                                    argw.insert("word".to_string(), set);
                                }
                            }
                            Err(_) => return Err("INVALID CONFIG".to_string())
                        }
                    }
                    else
                    {
                        return Err("INVALID COMMAND LINE".to_string());
                    }
                }
            }
            else
            {
                return Err("INVALID COMMAND LINE".to_string());
            }
        }
        else
        {
            arg.push(tmp.clone());
        }
    }
    let mut count: usize = 0;
    while count < arg.len()
    {
        match &arg[count]
        {
            a if a == "-w" || a == "--word" =>
            {
                args.insert("word".to_string());
                if count >= arg.len() - 1
                {
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("word".to_string(), arg[count + 1].to_string());
                        count += 1;
                    }
                }
            }
            b if b == "-r" || b == "--random" =>
            {
                args.insert("random".to_string());
            }
            c if c == "-D" || c == "--difficult" =>
            {
                args.insert("difficult".to_string());
            }
            d if d == "-t" || d == "--stats" =>
            {
                args.insert("stats".to_string());
            }
            e if e == "-d" || e == "--day" =>
            {
                args.insert("day".to_string());
                if count >= arg.len() - 1
                {
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argv.insert("day".to_string(), arg[count + 1].parse().unwrap());
                        count += 1;
                    }
                }
            }
            f if f == "-s" || f == "--seed" =>
            {
                args.insert("seed".to_string());
                if count >= arg.len() - 1
                {
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argv.insert("seed".to_string(), arg[count + 1].parse().unwrap());
                        count += 1;
                    }
                }
            }
            g if g == "-f" || g == "--final-set" =>
            {
                args.insert("final_set".to_string());
                if count >= arg.len() - 1
                {
                    return Err("INVALID COMMAND LINE".to_string());
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("final_set".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                    else
                    {
                        return Err("INVALID COMMAND LINE".to_string());
                    }
                }
            }
            h if h == "-a" || h == "--acceptable-set" =>
            {
                args.insert("acceptable_set".to_string());
                if count >= arg.len() - 1
                {
                    return Err("INVALID COMMAND LINE".to_string());
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("acceptable_set".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                    else
                    {
                        return Err("INVALID COMMAND LINE".to_string());
                    }
                }
            }
            i if i == "-S" || i == "--state" =>
            {
                args.insert("state".to_string());
                if count >= arg.len() - 1
                {
                    return Err("INVALID COMMAND LINE".to_string());

                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("state".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                    else
                    {
                        return Err("INVALID COMMAND LINE".to_string());
                    }
                }
            }
            _ =>
            {
                println!("HERE");
                return Err("INVALID COMMAND LINE".to_string());
            }
        }
        count += 1;
    }
    if let Some(file) = argw.get("acceptable_set")
    {
        match file::read_txt(&file)
        {
            Ok(tmp) => guess = tmp,
            Err(_) =>
            {
                return Err("INVALID A SET".to_string());
            }
        }
    }
    if let Some(file) = argw.get("final_set")
    {
        match file::read_txt(&file)
        {
            Ok(tmp) => answer = tmp,
            Err(_) =>
            {
                return Err("INVALID F SET".to_string());
            }
        }
    }
    Ok(Command
        {
            mode: args,
            value: argv,
            info: argw,
            answer_file: answer,
            guess_file: guess,
        })
}

pub fn arg_is_valid(cmd: &Command) -> bool
{
    if let Some(_i) = cmd.mode.get("final_set")
    {
        if let None = cmd.info.get("final_set")
        {
            return false;
        }
    }
    if let Some(_i) = cmd.mode.get("acceptable_set")
    {
        if let None = cmd.info.get("acceptable_set")
        {
            return false;
        }
        if let Some(_j) = cmd.mode.get("final_set")
        {
            if !cmd.answer_file.is_empty()
            {
                for check in cmd.answer_file.iter()
                {
                    if !cmd.guess_file.contains(check)
                    {
                        return false;
                    }
                }
            }
        }
        else
        {
            for check in FINAL
            {
                if !cmd.guess_file.contains(&check.to_string())
                {
                    return false;
                }
            }   
        }
    }
    if let Some(_i) = cmd.mode.get("state")
    {
        if let None = cmd.info.get("state")
        {
            return false;
        }
    }
    if let Some(_i) = cmd.mode.get("random")
    {
        if let Some(_j) = cmd.mode.get("word")
        {
            return false;
        }
    }
    if let Some(_i) = cmd.mode.get("day")
    {
        if let None = cmd.mode.get("random")
        {
            return false;
        }
    }
    if let Some(_i) = cmd.mode.get("seed")
    {
        if let None = cmd.mode.get("random")
        {
            return false;
        }
    }
    true
}