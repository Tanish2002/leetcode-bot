package service

import (
	"bytes"
	"encoding/json"
	"io"
	"net/http"
)

const LIMIT uint = 15
const URL = "https://leetcode.com/graphql"

// Submission Request
const GraphQLReqRecentAcSubmission string = `
query recentAcSubmissions($username: String!, $limit: Int!) {
  recentAcSubmissionList(username: $username, limit: $limit) {
    title
    titleSlug
    timestamp
  }
}`

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
type GraphQLReq struct {
	Query     string `json:"query"`
	Variables struct {
		Username string `json:"username"`
		Limit    uint   `json:"limit"`
	} `json:"variables"`
}

func (s *Service) GetRecentACSubmissions(username string) (*RecentAcSubmissionResp, error) {
	query := GraphQLReq{
		Query: GraphQLReqRecentAcSubmission,
		Variables: struct {
			Username string `json:"username"`
			Limit    uint   `json:"limit"`
		}{
			Username: username,
			Limit:    LIMIT,
		},
	}
	request, err := json.Marshal(query)
	payload := bytes.NewReader(request)

	client := &http.Client{}
	req, err := http.NewRequest(http.MethodPost, URL, payload)

	if err != nil {
		return nil, err
	}
	req.Header.Add("Content-Type", "application/json")

	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()
	body, err := io.ReadAll(res.Body)
	response := new(RecentAcSubmissionResp)
	err = json.Unmarshal(body, response)
	return response, err
}

// User Request
const GraphQLReqUser string = `
query getUserProfile($username: String!) {
  matchedUser(username: $username) {
    username
    profile {
      userAvatar
    }
  }
}
`

type UserResp struct {
	Data struct {
		MatchedUser struct {
			Username string `json:"username"`
			Profile  struct {
				UserAvatar string `json:"userAvatar"`
			} `json:"profile"`
		} `json:"matchedUser"`
	} `json:"data"`
}

func (s *Service) GetUser(username string) (*UserResp, error) {
	query := GraphQLReq{
		Query: GraphQLReqUser,
		Variables: struct {
			Username string `json:"username"`
			Limit    uint   `json:"limit"`
		}{
			Username: username,
		},
	}
	request, err := json.Marshal(query)
	payload := bytes.NewReader(request)

	client := &http.Client{}
	req, err := http.NewRequest(http.MethodPost, URL, payload)

	if err != nil {
		return nil, err
	}
	req.Header.Add("Content-Type", "application/json")

	res, err := client.Do(req)
	if err != nil {
		return nil, err
	}
	defer res.Body.Close()
	body, err := io.ReadAll(res.Body)
	if err != nil {
		return nil, err
	}

	var response UserResp
	err = json.Unmarshal(body, &response)
	if err != nil {
		return nil, err
	}
	return &response, nil
}
