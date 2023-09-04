package discord

import (
	"context"
	"errors"
	"leetcode-bot/service"
	"strings"

	"github.com/diamondburned/arikawa/v3/api"
	"github.com/diamondburned/arikawa/v3/api/cmdroute"
	"github.com/diamondburned/arikawa/v3/discord"
	"github.com/diamondburned/arikawa/v3/utils/json/option"
)

func (h *Handler) cmdAddUser(ctx context.Context, data cmdroute.CommandData) *api.InteractionResponseData {
	var options struct {
		Arg string `discord:"argument"`
	}

	if err := data.Options.Unmarshal(&options); err != nil {
		return errorResponse(err)
	}

	if user, err := service.GetUser(options.Arg); user.Data.MatchedUser.Username == "" || err != nil {
		errorResponse(errors.New("Failed to find user on leetcode"))
	}

	if err := h.Model.AddUser(options.Arg); err != nil {
		return errorResponse(err)
	}

	return &api.InteractionResponseData{
		Content: option.NewNullableString("User Added"),
	}
}

func (h *Handler) cmdGetUsers(ctx context.Context, data cmdroute.CommandData) *api.InteractionResponseData {
	users, err := h.Model.GetUsers()
	if err != nil {
		return errorResponse(err)
	}
	userList := ""
	for _, v := range users {
		userList += v + "\n"
	}
	return &api.InteractionResponseData{
		Content: option.NewNullableString(userList),
	}
}

func (h *Handler) cmdSetup(ctx context.Context, data cmdroute.CommandData) *api.InteractionResponseData {
	var options struct {
		ChannelID string `discord:"channelid"`
	}

	if err := data.Options.Unmarshal(&options); err != nil {
		return errorResponse(err)
	}

	// Parse the snowflake
	options.ChannelID = strings.TrimPrefix(options.ChannelID, "<#")
	options.ChannelID = strings.TrimSuffix(options.ChannelID, ">")

	// Store the channel in DB
	if err := h.Model.SetChannel(options.ChannelID); err != nil {
		return errorResponse(err)
	}

	return &api.InteractionResponseData{
		Content: option.NewNullableString("Channel Setup Complete"),
	}
}

func errorResponse(err error) *api.InteractionResponseData {
	return &api.InteractionResponseData{
		Content:         option.NewNullableString("**Error:** " + err.Error()),
		Flags:           discord.EphemeralMessage,
		AllowedMentions: &api.AllowedMentions{ /* none */ },
	}
}
