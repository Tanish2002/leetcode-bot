package utils

import "encoding/json"

type SQSMessage struct {
	Username    string
	UserAvatar  string
	Submissions *RecentAcSubmissionResp
}

type RecentAcSubmission struct {
	Title     string `json:"title"`
	TitleSlug string `json:"titleSlug"`
	Timestamp string `json:"timestamp"`
}

type RecentAcSubmissionResp struct {
	Data struct {
		RecentAcSubmissionList []RecentAcSubmission `json:"recentAcSubmissionList"`
	} `json:"data"`
}

func UnmarshalSQSMessage(sqsMessage string) (SQSMessage, error) {
	var message SQSMessage
	err := json.Unmarshal([]byte(sqsMessage), &message)
	if err != nil {
		return SQSMessage{}, err
	}
	return message, nil
}
