use std::collections::HashMap;

pub fn parse_options(args: &mut Vec<String>, stop_when_non_arg: bool) -> HashMap<String, String> {
    let mut options_map = HashMap::new();

    let mut stop = false;
    args.retain(|orig| {
        if orig.starts_with('-') && !stop {
            let mut arg = orig.clone();

            // Get key/value pair.
            arg.remove(0);
            if arg.starts_with('-') {
                arg.remove(0);
            }

            // Split key/value pair.
            let mut arg = arg.split('=');
            let key = arg.next().unwrap();
            let value = arg.next().unwrap_or("");
            options_map.insert(key.to_string(), value.to_string());
            return false;
        } else if stop_when_non_arg {
            stop = true;
        }
        true
    });

    options_map
}
