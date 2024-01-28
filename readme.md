# git jira autokey

A common request from clients is that each commit is tagged with the jira issue key for the feature I am working on.
This is very easy to forget, so it is better to automate it.

## demo
```shell
$ git branch --show-current
feature/ISSUE-1337-the-key-should-always-be-included-in-commit-message
$ git add src/main.rs
$ git commit -m "issue key will be added automatically now"
[feature/ISSUE-1337-the-key-should-always-be-included-in-commit-message 699c022] [ISSUE-1337] issue key will be added automatically now
 1 file changed, 1 insertion(+)
$ git log -n 1
commit 699c02224787ee1fc08afc6edeaa75f50df2ff58 (HEAD -> feature/ISSUE-1337-the-key-should-always-be-included-in-commit-message)
Author: Author name <author@mail.com>
Date:   <date>

    [ISSUE-1337] issue key will be added automatically now


```

## usage
`cargo build --release`

move the binary to your global git hooks folder and rename to `commit-msg`
