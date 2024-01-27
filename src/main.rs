use std::{env, process};
use std::fs::File;
use std::io::Read;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("No commit message file provided");
        process::exit(1);
    }

    let commit_message_file = &args[1];
    // Now you can use `commit_message_file` in your Rust code.

    // Open the file
    let mut file = File::open(commit_message_file).expect("failed to open commit message file");

    // Read the file contents into a String
    let mut commit_message_raw = String::new();
    file.read_to_string(&mut commit_message_raw).expect("failed to read commit message file");

    get_commit_message_without_comments(commit_message_file);
}

fn get_commit_message_without_comments(commit_message_raw: &String) -> String {
    return commit_message_raw
        .lines()
        .filter(|&line| !line.trim_start().starts_with('#'))
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n");
}

fn get_jira_issue_key(branch_name: &str) -> Option<String> {
    // Define the pattern
    let pattern = r"^(feature|bugfix)/([a-zA-Z]+-\d+)-.*$";

    // Create a regex
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Check if the branch name matches the pattern
    if let Some(captures) = re.captures(branch_name) {
        // Get the JIRA issue key
        if let Some(issue_key) = captures.get(2) {
            return Some(issue_key.as_str().to_string());
        }
    }

    // Branch name does not match the pattern
    None
}

#[cfg(test)]
mod tests {
    use crate::{get_commit_message_without_comments, get_jira_issue_key};

    #[test]
    fn test_get_issue_key() {
        let result = get_jira_issue_key("feature/NTA-1996-this-is-a-branch");
        let expect = Some("NTA-1996".to_string());
        assert_eq!(expect, result)
    }

    #[test]
    fn should_return_none_if_not_a_feature() {
        let result = get_jira_issue_key("NTA-1996-this-is-a-branch");
        assert!(result.is_none())
    }

    #[test]
    fn  without_comments_should_return_string() {
        let message = "this is a \nmultiline string\nwithout any\ncomments".to_string();
        let result = get_commit_message_without_comments(&message);
        assert_eq!(message, result);
    }

    #[test]
    fn  with_comments_should_return_string_without_comments() {
        let message = "this is a \nmultiline string\n#with \n# a few comments".to_string();
        let result = get_commit_message_without_comments(&message);
        assert_eq!(result, "this is a \nmultiline string");
    }

}
