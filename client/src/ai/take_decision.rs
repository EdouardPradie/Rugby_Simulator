pub fn start_test(message: &str) -> String {
    let mut result = String::new();
    let mut action = "";
    let mut started = false;

    for line in message.lines() {
        // Skip the "start" line
        if line.trim() == "start" || line.trim() == "play" {
            if line.trim() == "start" {
                print!("GAME START DETECTED\n");
                started = true;
            }
            result.push_str(&format!("play\n"));
            continue;
        }

        if line.trim() == "scrum" {
            let val = line.trim();
            result.push_str(&format!("{}\n", val));
            continue;
        }

        if line.trim().is_empty() {
            continue;
        }

        if line.starts_with("time:") {
            if let Some((_, value)) = line.split_once(':') {
                if let Ok(parsed_time) = value.trim().parse::<u64>() {
                    let time = parsed_time;
                    action = if time % 2 == 0 { "W135" } else { "W315" };
                }
            }
            continue;
        }

        if let Some((key, info)) = line.split_once(':') {
            if info.contains("/B:") {
                if started {
                    result.push_str(&format!("{}:P100\n", key));
                }
                result.push_str(&format!("{}:P90\n", key));
                continue;
            }
            result.push_str(&format!("{}:{}\n", key, action));
        }
    }

    return result;
}

pub fn scrum_test(message: &str) -> String {
    let mut result = String::new();
    let mut time = 0;

    for line in message.lines() {
        // Skip the "start" line
        if line.trim() == "start" || line.trim() == "play" {
            if line.trim() == "start" {
                print!("GAME START DETECTED\n");
            }
            result.push_str(&format!("play\n"));
            continue;
        }

        if line.trim() == "scrum" || line.trim() == "ruck" {
            let val = line.trim();
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
            if time == 0 && key == "H10" {
                result.push_str(&format!("{}:{}\n", key, "P0"));
                continue;
            }
            result.push_str(&format!("{}:{}\n", key, "S"));
        }
    }

    return result;
}

pub fn ruck_test(message: &str) -> String {
    let mut result = String::new();
    let mut time = 0;

    for line in message.lines() {
        // Skip the "start" line
        if line.trim() == "start" || line.trim() == "play" {
            if line.trim() == "start" {
                print!("GAME START DETECTED\n");
            }
            result.push_str(&format!("play\n"));
            continue;
        }

        if line.trim() == "ruck" {
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
//K45/15