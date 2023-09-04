package discord

import (
	"context"
	"encoding/json"
	"fmt"
	"leetcode-bot/models"
	"leetcode-bot/service"
	"log"
	"os"
	"os/signal"

	"github.com/diamondburned/arikawa/v3/api"
	"github.com/diamondburned/arikawa/v3/api/cmdroute"
	"github.com/diamondburned/arikawa/v3/discord"
	"github.com/diamondburned/arikawa/v3/gateway"
	"github.com/diamondburned/arikawa/v3/state"
	"github.com/redis/go-redis/v9"
	"github.com/robfig/cron/v3"
)

type Handler struct {
	*cmdroute.Router
	*state.State
	Model models.Model
}

type DiscordService struct {
	DiscordToken string
	Handler
}

var commands = []api.CreateCommandData{
	{
		Name:        "adduser",
		Description: "Add a user to track",
		Options: []discord.CommandOption{
			&discord.StringOption{
				OptionName:  "argument",
				Description: "LeetCode Username",
				Required:    true,
			},
		},
	},
	{
		Name:        "getusers",
		Description: "Get a list of users that are tracked",
	},
	{
		Name:        "channelsetup",
		Description: "Setup bot to send analytics on provided channel",
		Options: []discord.CommandOption{
			&discord.StringOption{
				OptionName:  "channelid",
				Description: "ChannelID to send messages to",
				Required:    true,
			},
		},
	},
}

func (d DiscordService) SetupBot() error {
	h := d.newHandler(state.New("Bot " + d.DiscordToken))
	h.State.AddInteractionHandler(h)
	h.State.AddIntents(gateway.IntentGuilds)
	h.State.AddHandler(func(*gateway.ReadyEvent) {
		me, _ := h.State.Me()
		log.Println("connected to the gateway as", me.Tag())
	})

	if err := overwriteCommands(h.State); err != nil {
		return err
	}

	// Hourly tracker function
	c := cron.New()
	c.AddFunc("@hourly", h.tracker)
	c.Start()

	ctx, cancel := signal.NotifyContext(context.Background(), os.Interrupt)
	defer cancel()

	if err := h.State.Connect(ctx); err != nil {
		return err
	}
	return nil
}

func (h Handler) tracker() {
	channelID, err := h.Model.GetChannel()
	if err != nil {
		log.Println("Error in getting channel from DB" + err.Error())
		return
	}
	snwflk, err := discord.ParseSnowflake(channelID)
	if err != nil {
		log.Println("Error in parsing snowflake" + err.Error())
		return
	}

	userList, err := h.Model.GetUsers()

	if len(userList) == 0 {
		return
	}

	for _, user := range userList {
		report, err := h.Model.GetUserReport(user)
		if err != nil && err != redis.Nil {
			h.SendMessage(discord.ChannelID(snwflk), "Unable to get cached report from DB, for user: "+user+"error: "+err.Error())
		} else if report == "" {
			recentReport, err := service.GetRecentACSubmissions(user)
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to get report from leetcode for user: "+user+"error: "+err.Error())
			}
			recentReportStr, err := json.Marshal(recentReport)
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to set report from leetcode for user: "+user+"error: "+err.Error())
			}
			err = h.Model.SetUserReport(user, string(recentReportStr))
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to set report from leetcode for user: "+user+"error: "+err.Error())
			}
		} else {
			var cachedReport service.RecentAcSubmissionResp
			err := json.Unmarshal([]byte(report), &cachedReport)
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to unmarshal cached report for user: "+user+"error: "+err.Error())
			}
			recentReport, err := service.GetRecentACSubmissions(user)
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to get report from leetcode for user: "+user+"error: "+err.Error())
			}
			extraItem := findExtraItems(cachedReport.Data.RecentAcSubmissionList, recentReport.Data.RecentAcSubmissionList)
			if len(extraItem) == 0 {
				continue
			}
			var embedFields []discord.EmbedField
			for _, item := range extraItem {
				embedFields = append(embedFields, discord.EmbedField{
					Name:   item.Title,
					Value:  fmt.Sprintf("[Link](https://leetcode.com/problems/%s)", item.TitleSlug),
					Inline: false,
				})
			}

			userInfo, err := service.GetUser(user)
			if err != nil {
				log.Println("Unable to get user information for the user: " + user)
			}

			recentReportStr, err := json.Marshal(recentReport)
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to set report from leetcode for user: "+user+"error: "+err.Error())
			}
			err = h.Model.SetUserReport(user, string(recentReportStr))
			if err != nil {
				h.SendMessage(discord.ChannelID(snwflk), "Unable to set report from leetcode for user: "+user+"error: "+err.Error())
			}

			h.SendEmbeds(discord.ChannelID(snwflk), discord.Embed{
				Title:       fmt.Sprintf("%s's New Submissions", user),
				Image:       &discord.EmbedImage{URL: userInfo.Data.MatchedUser.Profile.UserAvatar},
				Description: fmt.Sprintf("%s has done these questions in the last hour", user),
				Fields:      embedFields,
			})
		}
	}
}
func findExtraItems(existingList, newList []service.RecentAcSubmissionList) []service.RecentAcSubmissionList {
	extraItems := []service.RecentAcSubmissionList{}
	existingMap := make(map[string]struct{})

	// Populate the map with existing titles
	for _, item := range existingList {
		existingMap[item.Title] = struct{}{}
	}

	// Check each item in the new list and add it to extraItems if it doesn't exist in existingMap
	for _, item := range newList {
		if _, exists := existingMap[item.Title]; !exists {
			extraItems = append(extraItems, item)
		}
	}
	return extraItems
}

func (d DiscordService) newHandler(s *state.State) *Handler {
	d.Handler.State = s

	d.Handler.Router = cmdroute.NewRouter()
	// Automatically defer handles if they're slow.
	d.Handler.Use(cmdroute.Deferrable(s, cmdroute.DeferOpts{}))
	d.Handler.AddFunc("adduser", d.Handler.cmdAddUser)
	d.Handler.AddFunc("getusers", d.Handler.cmdGetUsers)
	d.Handler.AddFunc("channelsetup", d.Handler.cmdSetup)

	return &d.Handler
}

func overwriteCommands(s *state.State) error {
	return cmdroute.OverwriteCommands(s, commands)
}
