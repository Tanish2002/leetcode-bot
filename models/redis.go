package models

import (
	"context"

	"github.com/redis/go-redis/v9"
)

type Model struct {
	DB *redis.Client
}

func (m Model) AddUser(username string) error {
	ctx := context.Background()
	err := m.DB.SAdd(ctx, "user_list", username).Err()
	if err != nil {
		return err
	}
	return nil
}

func (m Model) GetUsers() ([]string, error) {
	ctx := context.Background()
	return m.DB.SMembers(ctx, "user_list").Result()
}

func (m Model) SetUserReport(username string, json string) error {
	ctx := context.Background()
	return m.DB.Set(ctx, username, json, 0).Err()
}

func (m Model) GetUserReport(username string) (string, error) {
	ctx := context.Background()
	return m.DB.Get(ctx, username).Result()
}

func (m Model) SetChannel(channelID string) error {
	ctx := context.Background()
	return m.DB.Set(ctx, "channelID", channelID, 0).Err()
}

func (m Model) GetChannel() (string, error) {
	ctx := context.Background()
	return m.DB.Get(ctx, "channelID").Result()
}
