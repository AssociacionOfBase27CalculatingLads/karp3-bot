use async_recursion::async_recursion;
use std::fs::OpenOptions;
use std::io::Write;

use chrono;
use rand::Rng;
use serenity::async_trait;
use serenity::framework::standard::macros::{command, group};
use serenity::framework::standard::{CommandResult, StandardFramework};
// use serenity::futures::TryFutureExt;
// use serenity::http::CacheHttp;
use serenity::model::channel::Message;
use serenity::prelude::*;

#[group]
#[commands(
    ping,
    dekarpdelaspecial,
    meme,
    calculate,
    calculate27,
    calculate_verbose,
    bongal,
    decimal
)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // We are verifying if the bot id is the same as the message author id.
        let mut nick: String = "karp3".to_string();
        if msg.author.id != ctx.cache.current_user_id() {
            nick = match msg.author_nick(&ctx).await {
                Some(v) => v,
                None => {
                    // println!("error reading nickname");
                    "None".to_string()
                }
            };
            // Some lang actions
        }
        // let channel_id = match msg.channel(&ctx).await {
        //     Some(v) => v,
        //     None => {
        //         println!("error reading channel");
        //         "error".to_string()
        //     }
        // };
        println!(
            "{} {}, {} - {}: {:?}",
            chrono::offset::Local::now(),
            msg.channel(&ctx).await.unwrap(),
            msg.author,
            nick,
            msg.content
        );
        let mut file = OpenOptions::new()
            .write(true)
            .append(true)
            .open("logs.txt")
            .unwrap();

        if let Err(e) = writeln!(
            file,
            "{} {}, {} - {}: {:?}",
            chrono::offset::Local::now(),
            msg.channel(&ctx).await.unwrap(),
            msg.author,
            nick,
            msg.content
        ) {
            eprintln!("Couldn't write to file: {}", e);
        }
    }
    // Since data is located in Context, this means you are also able to use it within events!
}

#[tokio::main]
async fn main() {
    let framework = StandardFramework::default()
        .configure(|c| c.prefix("$")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);

    // Login with a bot token from the environment
    // let token = std::env::var("KARPTOKEN").expect("token");
    let token = std::fs::read_to_string("token.txt").expect("The file could not be read");
    // let token = "MTAyMDcyMzU0Nzg3NTg0MDAzMA.GsS0y-.4ARNaQCzYzpPDEY_yciDSrm2chqrdxoVSyYnK0";
    // let intents = GatewayIntents::privileged();
    let intents = GatewayIntents::all();
    let mut client = Client::builder(token, intents)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");

    // start listening for events by starting a single shard
    match client.start().await {
        Err(e) => println!("bad news :( error: {:?}", e),
        Ok(_v) => println!("we're in!"),
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(ctx, "Pong!").await?;

    Ok(())
}

#[command]
async fn dekarpdelaspecial(ctx: &Context, msg: &Message) -> CommandResult {
    msg.channel_id
        .send_message(&ctx, |m| m.content("baka"))
        .await
        .unwrap();
    for _i in 0..25 {
        let num: u8 = rand::thread_rng().gen_range(0..=1);
        match num {
            0 => {
                msg.reply(&ctx, "🗿").await?;
            }
            1 => {
                msg.reply(&ctx, "<@519553926958284800> fart").await?;
            }
            _ => (),
        }
    }
    Ok(())
}
#[command]
async fn meme(ctx: &Context, msg: &Message) -> CommandResult {
    let resp = reqwest::get("https://reddit-meme-api.herokuapp.com")
        .await
        .unwrap()
        .text()
        .await
        .unwrap();
    // .text()?;
    let urls: Vec<&str> = resp.split("https://").collect();
    let url_2: Vec<&str> = urls[urls.len() - 2].split("%").collect();
    let url_3: Vec<&str> = url_2[0].split("%").collect();

    let mut url = "https://".to_string() + url_3[0];
    url = url.replace("\",\"", "");
    // url = url.replace("%..", "");
    msg.reply(&ctx, &url).await?;
    // println!("reply: {}", &url);

    Ok(())
}
#[command]
async fn calculate(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, false, false).await {
        Some(v) => {
            result = v;
        }
        None => {
            msg.channel_id
                .send_message(&ctx, |m| m.content("error running calculation"))
                .await?;
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(result.to_string()))
    //     .await?;
    msg.reply(&ctx, result.to_string()).await?;
    Ok(())
}

#[command]
async fn calculate_verbose(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, true, false).await {
        Some(v) => {
            result = v;
        }
        None => {
            msg.channel_id
                .send_message(&ctx, |m| m.content("error running calculation"))
                .await?;
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(result.to_string()))
    //     .await?;
    msg.reply(&ctx, result.to_string()).await?;
    Ok(())
}
#[command]
async fn calculate27(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<String> = msg
        .content
        .split(" ")
        .skip(1)
        .map(|f| f.to_string())
        .collect();

    let mut result: f64 = 0.0;
    match calculate_section(&msg, &ctx, &args, false, true).await {
        Some(v) => {
            result = v;
        }
        None => {
            msg.channel_id
                .send_message(&ctx, |m| m.content("error running calculation"))
                .await?;
        }
    }

    // msg.channel_id
    //     .send_message(&ctx, |m| m.content(result.to_string()))
    //     .await?;
    msg.reply(
        &ctx,
        match to_bongal(result.to_string()) {
            Some(v) => v,
            None => "error".to_string(),
        },
    )
    .await?;
    Ok(())
}

#[async_recursion]
async fn calculate_section(
    msg: &Message,
    ctx: &Context,
    suply_args: &Vec<String>,
    verbose: bool,
    is_bongal: bool,
) -> Option<f64> {
    let mut args: Vec<String> = suply_args.into_iter().map(|f| f.to_string()).collect();
    let mut result_f: f64 = 0.0;
    // println!("{:?}", args);
    while args.contains(&"(".to_string()) {
        let mut index_open: usize = 0;
        let mut open_counter: usize = 0;
        let mut close_counter: usize = 0;
        let mut index_close: usize = 0;
        for i in 0..args.len() {
            if args[i] == "(" {
                open_counter += 1;
                if open_counter == 1 {
                    index_open = i;
                }
            }
            if args[i] == ")" {
                close_counter += 1;
                if close_counter == open_counter {
                    index_close = i;
                    break;
                }
            }
        }
        // println!("open index at position: {}", index_open);
        // println!("close index at position: {}", index_close);

        let mut new_args: Vec<String> = args.clone().into_iter().skip(index_open + 1).collect();
        for _i in 0..(args.len() - index_close) {
            new_args.pop();
        }
        // println!("new args: {:?}", new_args);
        match calculate_section(&msg, &ctx, &new_args, verbose, is_bongal).await {
            Some(v) => {
                result_f = v;
            }
            None => (),
        };
        args[index_open] = result_f.to_string();

        // println!("args before () deletion: {:?}", args);
        for _i in 0..index_close - index_open {
            args.remove(index_open + 1);
        }
        // println!("args after () deletion: {:?}", args);
    }

    let mut result: f64 = 0.0;
    let mut numbers: Vec<f64> = Vec::new();
    let mut operators: Vec<String> = Vec::new();

    for i in 0..args.len() {
        match args[i].as_str() {
            "*" => operators.push(args[i].clone()),
            "+" => operators.push(args[i].clone()),
            "-" => operators.push(args[i].clone()),
            "/" => operators.push(args[i].clone()),
            _ => {
                if is_bongal {
                    match from_bongal(args[i].clone()) {
                        Some(v) => numbers.push(v),
                        None => {
                            msg.reply(&ctx, "invalid bongal->decimal conversion")
                                .await
                                .unwrap();
                            return None;
                        }
                    }
                    // numbers.push(from_bongal(args[i].clone()));
                } else {
                    match args[i].parse::<f64>() {
                        Ok(v) => numbers.push(v),
                        Err(e) => {
                            match msg
                                .channel_id
                                .send_message(&ctx, |m| m.content(e.to_string()))
                                .await
                            {
                                Ok(_v) => (),
                                Err(e) => println!("error: {}", e),
                            }

                            return None;
                        }
                    }
                }
            }
        }
    }

    if numbers.len() != operators.len() + 1 {
        match msg.reply(&ctx, "invalid syntax").await {
            Ok(_v) => (),
            Err(e) => println!("error: {}", e),
        }

        return None;
    }
    // println!("{:?}", numbers);
    // println!("{:?}", operators);

    while operators.contains(&"*".to_string()) || operators.contains(&"/".to_string()) {
        let mut _index: usize = 0;
        let mut index_m: usize = 10000;
        let mut index_d: usize = 10000;
        let mut is_div: bool = true;
        for i in 0..operators.len() {
            if operators[i] == "*" {
                index_m = i;
                break;
            }
        }
        for i in 0..operators.len() {
            if operators[i] == "/" {
                index_d = i;
                break;
            }
        }
        if index_d < index_m {
            _index = index_d;
            // is_div = true;
        } else {
            _index = index_m;
            is_div = false;
        }

        let num: f64 = if is_div {
            numbers[_index] / numbers[_index + 1]
        } else {
            numbers[_index] * numbers[_index + 1]
        };

        if verbose {
            let message: String = if is_div { "dividing " } else { "multiplying " }.to_string()
                + numbers[_index].to_string().as_str()
                + " by "
                + numbers[_index + 1].to_string().as_str();
            match msg
                .channel_id
                .send_message(&ctx, |m| m.content(message))
                .await
            {
                Ok(_v) => (),
                Err(e) => println!("error: {}", e),
            }
        }

        numbers.remove(_index + 1);
        numbers[_index] = num;
        operators.remove(_index);
    }

    for i in 0..numbers.len() {
        if i == 0 {
            result = numbers[0];
            continue;
        }
        match operators[i - 1].as_str() {
            "-" => {
                if verbose {
                    let message: String = "substracting ".to_string()
                        + numbers[i].to_string().as_str()
                        + " from "
                        + result.to_string().as_str();
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| m.content(message))
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("error: {}", e),
                    }
                }
                result = result - numbers[i];
            }
            "+" => {
                if verbose {
                    let message: String = "adding ".to_string()
                        + numbers[i].to_string().as_str()
                        + " to "
                        + result.to_string().as_str();
                    match msg
                        .channel_id
                        .send_message(&ctx, |m| m.content(message))
                        .await
                    {
                        Ok(_v) => (),
                        Err(e) => println!("error: {}", e),
                    }
                }
                result = result + numbers[i];
            }
            _ => {
                result = result;
            }
        }
    }

    Some(result)
}

#[command]
async fn bongal(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let mut bongal: String = String::new();
    match to_bongal(args[0].to_string()) {
        Some(v) => {
            bongal = v;
        }
        None => {
            msg.reply(&ctx, "invalid bongal").await.unwrap();
            return Ok(());
        }
    }

    msg.reply(&ctx, bongal).await?;

    Ok(())
}
#[command]
async fn decimal(ctx: &Context, msg: &Message) -> CommandResult {
    let args: Vec<&str> = msg.content.split(" ").skip(1).collect();
    let mut decimal: f64 = 0.0;

    match from_bongal(args[0].to_string()) {
        Some(v) => decimal = v,
        None => {
            msg.reply(&ctx, "invalid bongal->decimal conversion")
                .await
                .unwrap();
            return Ok(());
        }
    }
    msg.reply(&ctx, decimal).await?;

    Ok(())
}

fn to_bongal(decimal: String) -> Option<String> {
    let mut decimal = decimal;
    let mut result: String = String::new();

    let mut value_int: u128 = 0;
    let mut value_after = 0.0;
    let is_under: bool = decimal.contains("-");
    if is_under {
        decimal.remove(0);
    }

    if decimal.contains(".") {
        let cropped_decimal: Vec<&str> = decimal.split(".").collect();
        value_int = cropped_decimal[0]
            .parse::<u128>()
            .expect("error parsing whole part of decimal to bongal");

        value_after = ("0.".to_string() + cropped_decimal[1])
            .as_str()
            .parse::<f64>()
            .expect("error parsing the rest of decimal to bongal");
    } else {
        value_int = decimal
            .parse::<u128>()
            .expect("error parsing value to bongal");
    }
    if is_under {
        result += "-";
    }

    while value_int > 1 {
        // println!("{}", value_int);
        match value_int % 27 {
            0 => result += "0",
            1 => result += "1",
            2 => result += "2",
            3 => result += "3",
            4 => result += "4",
            5 => result += "5",
            6 => result += "6",
            7 => result += "7",
            8 => result += "8",
            9 => result += "9",
            10 => result += "α",
            11 => result += "β",
            12 => result += "γ",
            13 => result += "δ",
            14 => result += "ρ",
            15 => result += "F",
            16 => result += "η",
            17 => result += "∅",
            18 => result += "c",
            19 => result += "K",
            20 => result += "ʎ",
            21 => result += "u",
            22 => result += "V",
            23 => result += "Ś",
            24 => result += "O",
            25 => result += "π",
            26 => result += "P",
            _ => return None,
        }
        value_int = value_int / 27;
    }
    let result_arr: Vec<String> = result.chars().map(|f| f.to_string()).rev().collect();
    result = "".to_string();
    for i in 0..result_arr.len() {
        result += result_arr[i].as_str();
    }

    if decimal.contains(".") {
        result += ".";
        for _i in 0..5 {
            value_after *= 27.0;

            let value_after_string: String = value_after.to_string().clone();
            let value_str: Vec<&str> = value_after_string.split(".").collect();

            // println!("{value_after} {}", value_str[0]);

            match value_str[0] {
                "0" => result += "0",
                "1" => result += "1",
                "2" => result += "2",
                "3" => result += "3",
                "4" => result += "4",
                "5" => result += "5",
                "6" => result += "6",
                "7" => result += "7",
                "8" => result += "8",
                "9" => result += "9",
                "10" => result += "α",
                "11" => result += "β",
                "12" => result += "γ",
                "13" => result += "δ",
                "14" => result += "ρ",
                "15" => result += "F",
                "16" => result += "η",
                "17" => result += "∅",
                "18" => result += "c",
                "19" => result += "K",
                "20" => result += "ʎ",
                "21" => result += "u",
                "22" => result += "V",
                "23" => result += "Ś",
                "24" => result += "O",
                "25" => result += "π",
                "26" => result += "P",
                _ => return None,
            }
            value_after = value_after - value_after.floor();
        }
    }

    return Some(result);
}

fn from_bongal(bongal: String) -> Option<f64> {
    let mut result: f64 = 0.0;
    let mut is_below = false;
    // if is_below{

    // }
    let mut chars_after: Vec<String> = Vec::new();
    let mut chars_int: Vec<String> = Vec::new();
    if bongal.contains(".") {
        let cropped: Vec<&str> = bongal.split(".").collect();
        chars_int = cropped[0].chars().map(|f| f.to_string()).collect();
        chars_after = cropped[1].chars().map(|f| f.to_string()).collect();
    } else {
        chars_int = bongal.chars().map(|f| f.to_string()).collect();
    }
    if bongal.contains(&"-") {
        is_below = true;
        chars_int.remove(0);
    }
    chars_int.reverse();

    for i in 0..chars_int.len() {
        match chars_int[i].as_str() {
            "0" => result += 0.0 * (27u64.pow(i as u32)) as f64,
            "1" => result += 1.0 * (27u64.pow(i as u32)) as f64,
            "2" => result += 2.0 * (27u64.pow(i as u32)) as f64,
            "3" => result += 3.0 * (27u64.pow(i as u32)) as f64,
            "4" => result += 4.0 * (27u64.pow(i as u32)) as f64,
            "5" => result += 5.0 * (27u64.pow(i as u32)) as f64,
            "6" => result += 6.0 * (27u64.pow(i as u32)) as f64,
            "7" => result += 7.0 * (27u64.pow(i as u32)) as f64,
            "8" => result += 8.0 * (27u64.pow(i as u32)) as f64,
            "9" => result += 9.0 * (27u64.pow(i as u32)) as f64,
            "α" => result += 10.0 * (27u64.pow(i as u32)) as f64,
            "β" => result += 11.0 * (27u64.pow(i as u32)) as f64,
            "γ" => result += 12.0 * (27u64.pow(i as u32)) as f64,
            "δ" => result += 13.0 * (27u64.pow(i as u32)) as f64,
            "ρ" => result += 14.0 * (27u64.pow(i as u32)) as f64,
            "F" => result += 15.0 * (27u64.pow(i as u32)) as f64,
            "η" => result += 16.0 * (27u64.pow(i as u32)) as f64,
            "∅" => result += 17.0 * (27u64.pow(i as u32)) as f64,
            "c" => result += 18.0 * (27u64.pow(i as u32)) as f64,
            "K" => result += 19.0 * (27u64.pow(i as u32)) as f64,
            "ʎ" => result += 20.0 * (27u64.pow(i as u32)) as f64,
            "u" => result += 21.0 * (27u64.pow(i as u32)) as f64,
            "V" => result += 22.0 * (27u64.pow(i as u32)) as f64,
            "Ś" => result += 23.0 * (27u64.pow(i as u32)) as f64,
            "O" => result += 24.0 * (27u64.pow(i as u32)) as f64,
            "π" => result += 25.0 * (27u64.pow(i as u32)) as f64,
            "P" => result += 26.0 * (27u64.pow(i as u32)) as f64,
            _ => {
                return None;
            }
        }
        // println!("from {}", chars_int[i]);
    }

    for i in 0..chars_after.len() {
        match chars_after[i].as_str() {
            "1" => result += 1.0 / (27u64.pow(i as u32 + 1)) as f64,
            "2" => result += 2.0 / (27u64.pow(i as u32 + 1)) as f64,
            "3" => result += 3.0 / (27u64.pow(i as u32 + 1)) as f64,
            "4" => result += 4.0 / (27u64.pow(i as u32 + 1)) as f64,
            "5" => result += 5.0 / (27u64.pow(i as u32 + 1)) as f64,
            "6" => result += 6.0 / (27u64.pow(i as u32 + 1)) as f64,
            "7" => result += 7.0 / (27u64.pow(i as u32 + 1)) as f64,
            "8" => result += 8.0 / (27u64.pow(i as u32 + 1)) as f64,
            "9" => result += 9.0 / (27u64.pow(i as u32 + 1)) as f64,
            "α" => result += 10.0 / (27u64.pow(i as u32 + 1)) as f64,
            "β" => result += 11.0 / (27u64.pow(i as u32 + 1)) as f64,
            "γ" => result += 12.0 / (27u64.pow(i as u32 + 1)) as f64,
            "δ" => result += 13.0 / (27u64.pow(i as u32 + 1)) as f64,
            "ρ" => result += 14.0 / (27u64.pow(i as u32 + 1)) as f64,
            "F" => result += 15.0 / (27u64.pow(i as u32 + 1)) as f64,
            "η" => result += 16.0 / (27u64.pow(i as u32 + 1)) as f64,
            "∅" => result += 17.0 / (27u64.pow(i as u32 + 1)) as f64,
            "c" => result += 18.0 / (27u64.pow(i as u32 + 1)) as f64,
            "K" => result += 19.0 / (27u64.pow(i as u32 + 1)) as f64,
            "ʎ" => result += 20.0 / (27u64.pow(i as u32 + 1)) as f64,
            "u" => result += 21.0 / (27u64.pow(i as u32 + 1)) as f64,
            "V" => result += 22.0 / (27u64.pow(i as u32 + 1)) as f64,
            "Ś" => result += 23.0 / (27u64.pow(i as u32 + 1)) as f64,
            "O" => result += 24.0 / (27u64.pow(i as u32 + 1)) as f64,
            "π" => result += 25.0 / (27u64.pow(i as u32 + 1)) as f64,
            "P" => result += 26.0 / (27u64.pow(i as u32 + 1)) as f64,
            _ => {
                return None;
            }
        }
        // println!("from {}", chars_int[i]);
    }
    if is_below {
        result *= -1.0;
    }
    return Some(result);
}
