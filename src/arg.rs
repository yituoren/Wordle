use std::collections::{HashMap, HashSet};

pub struct Command
{
    pub mode: HashSet<String>,
    pub value: HashMap<String, u64>,
    pub info: HashMap<String, String>,
    pub answer_file: Vec<String>,
    pub guess_file: Vec<String>,
}

pub fn process_arg() -> (Command, bool)
{
    let mut args: HashSet<String> = HashSet::new();
    let mut argv: HashMap<String, u64> = HashMap::new();
    let mut argw: HashMap<String, String> = HashMap::new();
    let mut arg: Vec<String> = Vec::new();
    let mut answer: Vec<String> = Vec::new();
    let mut guess: Vec<String> = Vec::new();
    for tmp in std::env::args()
    {
        if tmp == "-c" || tmp == "--config"
        {
            //do sth
        }
        else
        {
            arg.push(tmp.clone());
        }
    }
    let mut count: usize = 1;
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
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("final_set".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                }
            }
            h if h == "-a" || h == "--acceptable-set" =>
            {
                args.insert("acceptable_set".to_string());
                if count >= arg.len() - 1
                {
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("acceptable_set".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                }
            }
            i if i == "-S" || i == "--state" =>
            {
                args.insert("state".to_string());
                if count >= arg.len() - 1
                {
                    break;
                }
                if let Some(i) = arg[count + 1].chars().nth(0)
                {
                    if i != '-'
                    {
                        argw.insert("state".to_string(), arg[count + 1].clone());
                        count += 1;
                    }
                }
            }
            _ =>
            {
                return (Command
                    {
                        mode: args,
                        value: argv,
                        info: argw,
                        answer_file: answer,
                        guess_file: guess,
                    }, false);
            }
        }
        count += 1;
    }
    (Command
        {
            mode: args,
            value: argv,
            info: argw,
            answer_file: answer,
            guess_file: guess,
        }, true)
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
    }
    if let Some(_i) = cmd.mode.get("state")
    {
        if let None = cmd.info.get("state")
        {
            return false;
        }
    }
    if let Some(_i) = cmd.mode.get("config")
    {
        if let None = cmd.info.get("config")
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