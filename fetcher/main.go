package main

import (
	"log"

	"github.com/Tanish2002/leetcode-bot/fetcher/conf"
	"github.com/Tanish2002/leetcode-bot/fetcher/service"
)

func main() {
	newConfig := conf.NewConfig()
	// Range over all users
	for _, user := range newConfig.USERS {

		//Check for valid user
		userResp, err := newConfig.Service.GetUser(user)
		if err != nil {
			continue
		}

		// Get lastTimestamp for user
		lastTimestamp, err := newConfig.Model.GetLatestTimestamp(user)
		if err != nil && err.Error() != "Timestamp not found" {
			log.Printf("Error while getting timestamp for user: %s. Error: %v", user, err)
			continue
		}

		// Get submissions for user
		userSubmissions, err := newConfig.Service.GetRecentACSubmissions(user)
		if err != nil {
			log.Printf("Error while fetching user submissions for user: %s. Error: %v", user, err)
			continue
		}
		var filteredUserSubmissions service.RecentAcSubmissionResp
		// Filter submissions so we don't send duplicate data
		for _, submission := range userSubmissions.Data.RecentAcSubmissionList {

			// If last timestamp occured then we have already sent the rest submissions
			if submission.Timestamp == lastTimestamp {
				break
			}
			filteredUserSubmissions.Data.RecentAcSubmissionList = append(filteredUserSubmissions.Data.RecentAcSubmissionList, submission)
		}

		// We have no new submissions
		if len(filteredUserSubmissions.Data.RecentAcSubmissionList) == 0 {
			continue
		}

		// Update the latestTimestamp
		err = newConfig.Model.AddOrUpdateTimestamp(user, filteredUserSubmissions.Data.RecentAcSubmissionList[0].Timestamp)
		if err != nil {
			log.Println("Error while saving latest Timestamp.", err)
			continue
		}

		// Send Data to SQS
		err = newConfig.Service.SendToSQS(service.SQSMessage{
			Username:    user,
			UserAvatar:  userResp.Data.MatchedUser.Profile.UserAvatar,
			Submissions: &filteredUserSubmissions,
		})
		if err != nil {
			// revert back to older timestamp
			newConfig.Model.AddOrUpdateTimestamp(user, lastTimestamp)
		}
	}
}
