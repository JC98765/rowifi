use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};

use crate::user::RoGuildUser;

lazy_static! {
    static ref TEMPLATE_REGEX: Regex = Regex::new(r"\{(.*?)\}").unwrap();
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Template(pub String);

impl Template {
    pub fn nickname(
        &self,
        roblox_username: &str,
        user: &RoGuildUser,
        discord_nick: &str,
    ) -> String {
        let roblox_id = user.roblox_id.to_string();
        let discord_id = user.discord_id.to_string();

        let template_str = &self.0;
        let mut parts = vec![];

        let mut matches = TEMPLATE_REGEX
            .find_iter(template_str)
            .map(|m| (m.start(), m.end()))
            .peekable();
        let first = match matches.peek() {
            Some((start, _)) => *start,
            None => return template_str.clone(),
        };

        if first > 0 {
            parts.push(&template_str[0..first]);
        }

        let mut previous_end = first;
        for (start, end) in matches {
            if previous_end != start {
                parts.push(&template_str[previous_end..start]);
            }

            let arg = &template_str[start..end];
            let arg_name = &arg[1..arg.len() - 1];
            match arg_name {
                "roblox-username" => parts.push(roblox_username),
                "roblox-id" => parts.push(&roblox_id),
                "discord-id" => parts.push(&discord_id),
                "discord-name" => parts.push(discord_nick),
                _ => parts.push(arg),
            }

            previous_end = end;
        }

        if previous_end < template_str.len() {
            parts.push(&template_str[previous_end..]);
        }

        parts.join("")
    }
}

impl Display for Template {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "{}", self.0)
    }
}