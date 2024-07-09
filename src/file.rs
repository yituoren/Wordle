use serde::{Serialize, Deserialize};
use std::fs::File;
use std::io::{self, BufRead, BufReader, BufWriter};
use std::collections::HashSet;
use std::io::ErrorKind;
use std::io::Read;

//创建两个具有Serialize和Deserialize的结构体记录数据，方便读写
#[derive(Debug, Serialize, Deserialize)]
pub struct Round
{
    pub answer: String,
    pub guesses: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData
{
    pub total_rounds: i32,
    pub games: Vec<Round>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Config
{
    pub random: Option<bool>,
    pub difficult: Option<bool>,
    pub stats: Option<bool>,
    pub day: Option<u64>,
    pub seed: Option<u64>,
    pub final_set: Option<String>,
    pub acceptable_set: Option<String>,
    pub state: Option<String>,
    pub word: Option<String>,
}

pub fn read_txt(file_name: &str) -> std::io::Result<Vec<String>>
{
    let mut set: HashSet<String> = HashSet::new();
    let mut words: Vec<String> = Vec::new();

    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    for line in reader.lines()
    {
        let word = line?.to_uppercase();
        if set.insert(word.clone())
        {
            words.push(word);
        }
    }
    words.sort();
    Ok(words)
}

pub fn read_state(file_name: &str) -> std::io::Result<GameData>
{
    match File::open(file_name)
    {
        Ok(file) =>
        {
            let mut reader = BufReader::new(file);
            let mut content = String::new();
            reader.read_to_string(&mut content)?;
            if content.trim() == "{}"//json为"{}"
            {
                Ok(GameData{ total_rounds: 0, games: Vec::new(), })
            }
            else
            {
                match serde_json::from_str(&content)
                {
                    Ok(game_data) => Ok(game_data),
                    Err(error) if error.is_eof() => Ok(GameData{ total_rounds: 0, games: Vec::new(), }),//json为空
                    Err(error) => Err(error.into()),//其他错误
                }
            }
        }
        Err(error) if error.kind() == ErrorKind::NotFound => Ok(GameData{ total_rounds: 0, games: Vec::new(), }),//json不存在
        Err(error) => Err(error),//其他错误
    }
}

pub fn write_state(game_data: &GameData, file_name: &str) -> std::io::Result<()>
{
    let file = File::create(file_name)?;
    let writer = BufWriter::new(file);
    serde_json::to_writer_pretty(writer, game_data)?;
    Ok(())
}

pub fn read_config(file_name: &str) -> std::io::Result<Config>
{
    let file = File::open(file_name)?;
    let reader = BufReader::new(file);
    let config = serde_json::from_reader(reader)?;
    Ok(config)
}