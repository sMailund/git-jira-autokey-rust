extern crate git2;

use git2::Repository;
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

    let branch_name = get_current_branch().expect("failed to get current branch");
    let issue_key = match get_jira_issue_key(branch_name) {
        Some(key) => key,
        None => process::exit(0)
    };

    let commit_message_file = &args[1];

    // Open the file
    let mut file = File::open(commit_message_file).expect("failed to open commit message file");

    // Read the file contents into a String
    let mut commit_message_raw = String::new();
    file.read_to_string(&mut commit_message_raw).expect("failed to read commit message file");

    add_jira_key_to_commit_message(commit_message_raw, issue_key);
}


fn add_jira_key_to_commit_message(commit_message_file: String, issue_key: String) -> String {
    let commit_message = get_commit_message_without_comments(commit_message_file);

    if commit_message.contains(&issue_key) {
        println!("Commit already contains issue key");
        return commit_message;
    }

    let combined = format!("[{}] {}", issue_key, commit_message);
    return combined;
}

fn get_current_branch() -> Option<String> {
    // Open the repository in the current directory
    if let Ok(repo) = Repository::open(".") {
        // Get the head reference
        if let Ok(head) = repo.head() {
            // Get the symbolic target of the head reference
            if let Some(branch) = head.shorthand() {
                return Some(branch.to_string());
            }
        }
    }
    None
}

fn get_commit_message_without_comments(commit_message_raw: String) -> String {
    return commit_message_raw
        .lines()
        .filter(|&line| !line.trim_start().starts_with('#'))
        .map(|line| line.to_string())
        .collect::<Vec<String>>()
        .join("\n");
}

fn get_jira_issue_key(branch_name: String) -> Option<String> {
    // Define the pattern
    let pattern = r"^(feature|bugfix)/([a-zA-Z]+-\d+)-.*$";

    // Create a regex
    let re = Regex::new(pattern).expect("Invalid regex pattern");

    // Check if the branch name matches the pattern
    if let Some(captures) = re.captures(&branch_name) {
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
    use crate::{add_jira_key_to_commit_message, get_commit_message_without_comments, get_jira_issue_key};

    #[test]
    fn test_get_issue_key() {
        let result = get_jira_issue_key("feature/NTA-1996-this-is-a-branch".to_string());
        let expect = Some("NTA-1996".to_string());
        assert_eq!(expect, result)
    }

    #[test]
    fn should_return_none_if_not_a_feature() {
        let result = get_jira_issue_key("NTA-1996-this-is-a-branch".to_string());
        assert!(result.is_none())
    }

    #[test]
    fn  without_comments_should_return_string() {
        let message = "this is a \nmultiline string\nwithout any\ncomments".to_string();
        let expect = message.clone();
        let result = get_commit_message_without_comments(message);
        assert_eq!(expect, result);
    }

    #[test]
    fn  with_comments_should_return_string_without_comments() {
        let message = "this is a \nmultiline string\n#with \n# a few comments".to_string();
        let result = get_commit_message_without_comments(message);
        assert_eq!(result, "this is a \nmultiline string");
    }

    #[test]
    fn should_add_issue_key() {
        let issue_key = "NTA-2027".to_string();
        let message = "commit message".to_string();
        let expect = "[NTA-2027] commit message".to_string();

        let result = add_jira_key_to_commit_message(message, issue_key);

        assert_eq!(expect, result)
    }

    #[test]
    fn should_ignore_issue_key_when_already_present() {
        let issue_key = "NTA-2027".to_string();
        let message = "[NTA-2027] commit message".to_string();
        let expect = "[NTA-2027] commit message".to_string();

        let result = add_jira_key_to_commit_message(message, issue_key);

        assert_eq!(expect, result)
    }

}
