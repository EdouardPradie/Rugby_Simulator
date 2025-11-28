pub fn take_decision(message: &str) -> String {
    let mut result = String::new();
    let mut time = 0;
    let mut action = "";

    for line in message.lines() {
        // Skip the "start" line
        if line.trim() == "start" {
            continue;
        }

        if line .starts_with("time:") {
            if let Some((_, value)) = line.split_once(':') {
                if let Ok(parsed_time) = value.trim().parse::<u64>() {
                    time = parsed_time;
                    action = if time % 2 == 0 { "RE" } else { "RW" };
                }
            }

            result.push_str(&format!("play\n"));
            continue;
        }

        if let Some((key, _)) = line.split_once(':') {
            result.push_str(&format!("{}:{}\n", key, action));
        }
    }

    return result;
}