package conf

import (
	"context"
	"leetcode-bot/discord"
	"leetcode-bot/models"
	"log"
	"os"

	"github.com/redis/go-redis/v9"
)

type Config struct {
	DiscordService discord.DiscordService
}

func NewConfig() Config {
	redisAddr := os.Getenv("REDIS_ADDR")
	redisPass := os.Getenv("REDIS_PASS")
	if redisAddr == "" || redisPass == "" {
		log.Fatalln("REDIS_ADDR or REDIS_PASS not found")
	}
	discordToken := os.Getenv("DISCORD_TOKEN")
	if discordToken == "" {
		log.Fatalln("No $BOT_TOKEN given.")
	}

	client := redis.NewClient(&redis.Options{
		Addr:     redisAddr,
		Password: redisPass,
		DB:       0, // Redis database number
	})
	_, err := client.Ping(context.Background()).Result()
	if err != nil {
		log.Fatalln(err.Error())
	}

	return Config{
		DiscordService: discord.DiscordService{
			DiscordToken: discordToken,
			Handler: discord.Handler{
				Model: models.Model{DB: client},
			},
		},
	}
}
