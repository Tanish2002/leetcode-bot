package services

import (
	"fmt"

	"github.com/Tanish2002/leetcode-bot/sender/utils"
	"github.com/diamondburned/arikawa/v3/api/webhook"
	"github.com/diamondburned/arikawa/v3/discord"
)

type Service struct {
	Client *webhook.Client
}

func (s *Service) SendEmbedToDiscord(sqsMessage utils.SQSMessage) error {
	var embedFields []discord.EmbedField
	for _, submission := range sqsMessage.Submissions.Data.RecentAcSubmissionList {
		embedFields = append(embedFields, discord.EmbedField{
			Name:   submission.Title,
			Value:  fmt.Sprintf("[Link](https://leetcode.com/problems/%s)", submission.TitleSlug),
			Inline: false,
		})
	}
	return s.Client.Execute(webhook.ExecuteData{
		Username:  "Leetcode",
		AvatarURL: "https://upload.wikimedia.org/wikipedia/commons/8/8e/LeetCode_Logo_1.png",
		Embeds: []discord.Embed{{
			Title:       fmt.Sprintf("%s's New Submissions", sqsMessage.Username),
			Thumbnail:   &discord.EmbedThumbnail{URL: sqsMessage.UserAvatar},
			Author:      &discord.EmbedAuthor{Name: sqsMessage.Username, Icon: sqsMessage.UserAvatar},
			Timestamp:   discord.NowTimestamp(),
			Description: fmt.Sprintf("%s has done these questions in the last hour", sqsMessage.Username),
			Fields:      embedFields,
		},
		},
	})

}
