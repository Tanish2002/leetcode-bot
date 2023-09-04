package main

import (
	"leetcode-bot/conf"
	"log"
)

func main() {
	config := conf.NewConfig()

	if err := config.DiscordService.SetupBot(); err != nil {
		log.Fatalln("Error setting up bot" + err.Error())
	}
}
