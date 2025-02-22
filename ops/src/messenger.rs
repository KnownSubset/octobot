use std::sync::Arc;

use crate::slack::{self, SlackAttachment, SlackRequest};
use crate::util;
use crate::worker::Worker;
use octobot_lib::config::Config;
use octobot_lib::github;
use octobot_lib::slack::SlackRecipient;

pub struct Messenger {
    config: Arc<Config>,
    slack: Arc<dyn Worker<SlackRequest>>,
}

pub fn new(config: Arc<Config>, slack: Arc<dyn Worker<SlackRequest>>) -> Messenger {
    Messenger {
        slack: slack.clone(),
        config,
    }
}

impl Messenger {
    // TODO
    #[allow(clippy::too_many_arguments)]
    pub fn send_to_all<T: github::CommitLike>(
        &self,
        msg: &str,
        attachments: &[SlackAttachment],
        item_owner: &github::User,
        sender: &github::User,
        repo: &github::Repo,
        participants: &[github::User],
        branch: &str,
        commits: &[T],
    ) {
        self.send_to_channel(msg, attachments, repo, branch, commits);

        let mut slackbots: Vec<github::User> = vec![item_owner.clone()];

        slackbots.extend(
            participants
                .iter()
                .filter(|a| a.login != item_owner.login)
                .cloned(),
        );

        // make sure we do not send private message to author of that message
        slackbots.retain(|u| u.login != sender.login && u.login() != "octobot");

        self.send_to_slackbots(slackbots, msg, attachments);
    }

    pub fn send_to_owner<T: github::CommitLike>(
        &self,
        msg: &str,
        attachments: &[SlackAttachment],
        item_owner: &github::User,
        repo: &github::Repo,
        branch: &str,
        commits: &[T],
    ) {
        self.send_to_channel(msg, attachments, repo, branch, commits);
        self.send_to_slackbots(vec![item_owner.clone()], msg, attachments);
    }

    pub fn send_to_channel<T: github::CommitLike>(
        &self,
        msg: &str,
        attachments: &[SlackAttachment],
        repo: &github::Repo,
        branch: &str,
        commits: &[T],
    ) {
        for channel in self.config.repos().lookup_channels(repo, branch, commits) {
            let channel_msg = format!(
                "{} ({})",
                msg,
                util::make_link(&repo.html_url, &repo.full_name)
            );
            self.slack.send(slack::req(
                SlackRecipient::new(&channel, &channel),
                &channel_msg,
                attachments,
            ));
        }
    }

    fn send_to_slackbots(
        &self,
        users: Vec<github::User>,
        msg: &str,
        attachments: &[SlackAttachment],
    ) {
        for user in users {
            if let Some(channel) = self.config.users().slack_direct_message(user.login()) {
                self.slack.send(slack::req(channel, msg, attachments));
            }
        }
    }
}
