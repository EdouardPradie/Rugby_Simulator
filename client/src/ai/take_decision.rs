pub fn start_test(message: &str) -> String {
    let mut result = String::new();
    let mut started = false;

    for line in message.lines() {
        // Skip the "start" line
        if line.starts_with("start") || line.starts_with("play") || line.starts_with("free-kick") {
            if line.starts_with("start") {
                print!("GAME START DETECTED\n");
                started = true;
            }
            result.push_str(&format!("play\n"));
            continue;
        }

        if line.starts_with("scrum") {
            let val = line.trim();
            result.push_str(&format!("{}\n", val));
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("time:") {
            continue;
        }

        if let Some((key, info)) = line.split_once(':') {
            if info.contains("/B:") && started {
                result.push_str(&format!("{}:K315/30\n", key));
                continue;
            }
            if info.contains("/B:") {
                result.push_str(&format!("{}:P135\n", key));
                continue;
            }
            result.push_str(&format!("{}:{}\n", key, "S"));
            continue;
        }
        // if line.trim() != "" {
        //     print!("don't know line: \"{}\"\n", line);
        // }
    }

    if !result.starts_with("play") {
        result.insert_str(0, "play\n");
    }
    return result;
}

pub fn scrum_test(message: &str) -> String {
    let mut result = String::new();

    for line in message.lines() {
        if line.starts_with("scrum") {
            let val = line.trim();
            result.push_str(&format!("{}\n", val));
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("time:") {
            continue;
        }

        if let Some((key, _)) = line.split_once(':') {
            result.push_str(&format!("{}:{}\n", key, "S"));
        }
    }

    return result;
}

pub fn ruck_test(message: &str) -> String {
    let mut result = String::new();
    let mut time = 0;

    for line in message.lines() {

        if line.starts_with("ruck") {
            let val: &str = line.trim();
            result.push_str(&format!("{}\n", val));
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("time:") {
            if let Some((_, value)) = line.split_once(':') {
                if let Ok(parsed_time) = value.trim().parse::<u64>() {
                    time = parsed_time;
                }
            }
            continue;
        }

        if let Some((key, _)) = line.split_once(':') {
            if time == 0 {
                if key == "A3" {
                    result.push_str(&format!("{}:{}\n", key, "R15"));
                    continue;
                }
                if key == "A5" {
                    result.push_str(&format!("{}:{}\n", key, "R180"));
                    continue;
                }
                if key == "H4" {
                    result.push_str(&format!("{}:{}\n", key, "R0"));
                    continue;
                }

                if key == "H5" {
                    result.push_str(&format!("{}:{}\n", key, "R0"));
                    continue;
                }
                if key == "H9" {
                    result.push_str(&format!("{}:{}\n", key, "R0"));
                    continue;
                }
            }
            if time == 25 && key == "H9" {
                result.push_str(&format!("{}:{}\n", key, "T"));
                continue;
            }
            if time == 50 && key == "H9" {
                result.push_str(&format!("{}:{}\n", key, "P135"));
                continue;
            }
            result.push_str(&format!("{}:{}\n", key, "S"));
        }
    }

    return result;
}

pub fn penalty_test(message: &str) -> String {
    let mut result = String::new();

    for line in message.lines() {
        // Skip the "start" line
        if line.starts_with("start") || line.starts_with("play") {
            if line.starts_with("start") {
                print!("GAME START DETECTED\n");
            }
            result.push_str(&format!("play\n"));
            continue;
        }

        if line.starts_with("set-penalty") {
            let val: &str = line.trim();
            result.push_str(&format!("{}\n", val));
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("time:") {
            continue;
        }
    }

    result.push_str(&format!("K/10/315/30\n"));

    return result;
}