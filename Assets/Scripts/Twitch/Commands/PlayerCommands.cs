using Character;
using GameEventSystem;
using GameEventSystem.Events.Voting;
using Processors;
using Core;
using Pets.Enumerations;
using System;
using System.Linq;
using Twitch.Utils;
using TwitchLib.Client.Events;
using Utils;
using Reflex.Attributes;
using ScriptablesProcessorInfrastructure;

namespace Twitch.Commands
{
    /// <summary>
    /// Handles all Twitch chat commands related to basic Player functions.
    /// </summary>
    public class PlayerCommands
    {
        private PlayerProcessor _playerProcessor;
        private GameEventProcessor _gameEventProcessor;
        private LabelDisplayProcessor _utilDisplayProcessor;
        private TwitchClientProcessor _twitchClientProcessor;
        private Processors.TwitchChatProcessor _twitchChatProcessor;
        private EventCommands _eventCommands;
        private RoleProcessor _roleProcessor;
        private GameSettings _gameSettings;

        public PlayerCommands(PlayerProcessor playerProcessor, GameEventProcessor gameEventProcessor,
            LabelDisplayProcessor utilDisplayProcessor, TwitchClientProcessor twitchClientProcessor,
            Processors.TwitchChatProcessor twitchChatProcessor, EventCommands eventCommands,
            RoleProcessor roleProcessor, GameSettings gameSettings)
        {
            _playerProcessor = playerProcessor;
            _gameEventProcessor = gameEventProcessor;
            _utilDisplayProcessor = utilDisplayProcessor;
            _twitchClientProcessor = twitchClientProcessor;
            _twitchChatProcessor = twitchChatProcessor;
            _eventCommands = eventCommands;
            _roleProcessor = roleProcessor;
            _gameSettings = gameSettings;
        }

        /// <summary>
        /// Attempts to create a player and will set it's role if provided in the arguments.
        /// </summary>
        /// <param name="e">The chat command received arguments.</param>
        public Player TryCreatePlayer(OnChatCommandReceivedArgs e)
        {
            string command = e.Command.CommandText.ToLower();
            string[] args = e.Command.ArgumentsAsList.ToArray();
            bool isSub = e.Command.ChatMessage.IsSubscriber;
            bool isBroadcaster = e.Command.ChatMessage.IsBroadcaster;

            // Default role to builder.
            PlayerRole role = PlayerRole.Builder;

            // Check if the user has picked a role to start as.
            if (args.Length > 0)
            {
                if (Enum.TryParse(args[0], true, out role))
                    if ((int)role > (int)PlayerRole.Count)
                        role = PlayerRole.Builder;
            }

            // Create the Twitch user data.
            TwitchUser user = new TwitchUser(e.Command.ChatMessage.UserId, e.Command.ChatMessage.Username);
            if(isBroadcaster)
                user.IsBroadcaster = true;

            // Assign the user their twitch role (Broadcaster, Moderator, etc)
            // If the user is a GameMaster, they will be given that role.
            if (_gameSettings.GM_IDs.Contains(user.UserID))
            {
                user.GameUserType = GameUserType.GameMaster;
            }
            else if (e.Command.ChatMessage.IsBroadcaster)
            {
                user.GameUserType = GameUserType.Broadcaster;
            }
            else if (e.Command.ChatMessage.IsModerator)
            {
                user.GameUserType = GameUserType.Moderator;
            }
            else if (e.Command.ChatMessage.IsSubscriber)
            {
                user.GameUserType = GameUserType.Subscriber;
            }
            else
            {
                user.GameUserType = GameUserType.Normal;
            }

            user.TwitchUserType = e.Command.ChatMessage.UserType;

            Player player = new Player(user);

            _playerProcessor.AddNewPlayer(player, role);

            if (isSub)
                _twitchClientProcessor.UserIsSubscribed(player.TwitchUser.UserID);

            _twitchChatProcessor.SendPreBuiltMessage(user.Username, "characterCreated");
            return player;
        }

        /// <summary>
        /// Changes the user's Hair Style.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the hair style index.</param>
        public void ChangeHairStyle(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetHairByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Hair Style Changed!");
            }
        }

        /// <summary>
        /// Changes the user's Eye Type.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the eye type index.</param>
        public void ChangeEyes(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetEyesByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Eye Style Changed!");
            }
        }

        /// <summary>
        /// Changes the user's Facial Hair style.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the facial hair style index.</param>
        public void ChangeFacialHair(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetFacialHairByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Facial Hair Style Changed!");
            }
        }

        /// <summary>
        /// Changes the user's body type.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the body type index.</param>
        public void ChangeBodyType(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetBodyTypeByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Body Type Changed!");
            }
        }

        /// <summary>
        /// Changes the user's hair color.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the hair color index.</param>
        public void ChangeHairColor(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetHairColorByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Hair Color Changed!");
            }
        }

        /// <summary>
        /// Changes the user's eye color.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the eye color index.</param>
        public void ChangeEyeColor(Player player, string command, params string[] args)
        {
            if (int.TryParse(args[0], out int index))
            {
                if (player.EquipmentHandler.SetEyeColorByIndex(index))
                    _twitchChatProcessor.SendPlayerMessage(player, "Eye Color Changed!");
            }
        }

        /// <summary>
        /// Pings the player's location on the map.
        /// </summary>
        /// <param name="player">The player.</param>
        public void PingPlayer(Player player)
        {
            _utilDisplayProcessor.AddPingObject(player);
        }

        /// <summary>
        /// Casts a vote in the current active vote event.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the vote option.</param>
        public void Vote(Player player, string command, params string[] args)
        {
            if (args == null || args.Length == 0 || string.IsNullOrWhiteSpace(args[0]))
            {
                const string usageMessage = "Failed - Use !vote followed by an option number, for example !vote 3";
                _twitchChatProcessor.RecordCommandResult(command, usageMessage);
                _twitchChatProcessor.SendPlayerMessage(player, usageMessage);
                return;
            }

            var currentEvent = _gameEventProcessor.CurrentEvent;

            if (!(currentEvent is VoteEvent voteEvent))
            {
                const string noVoteMessage = "Failed - No vote is active yet";
                _twitchChatProcessor.RecordCommandResult(command, noVoteMessage);
                _twitchChatProcessor.SendPlayerMessage(player, noVoteMessage);
                return;
            }

            string option = args[0].Trim();
            if (!voteEvent.TryAddVote(new PlayerVote(player, new VoteOption(option, null)), out string failureReason))
            {
                string failureMessage = $"Vote failed - {failureReason}";
                _twitchChatProcessor.RecordCommandResult(command, failureMessage);
                _twitchChatProcessor.SendPlayerMessage(player, failureMessage);
                return;
            }

            string successMessage = $"Vote {option} accepted";
            _twitchChatProcessor.RecordCommandResult(command, successMessage);
        }

        /// <summary>
        /// Teleports the player to the spawn point if they are stuck.
        /// </summary>
        /// <param name="player">The player.</param>
        public void Unstuck(Player player)
        {
            player.Character.transform.position = UnityEngine.Vector3.zero;
        }

        /// <summary>
        /// Praises the Fish God event.
        /// </summary>
        /// <param name="player">The player.</param>
        public void Praise(Player player)
        {
            _eventCommands.HandleFishGodEvent();
        }

        /// <summary>
        /// Prints a list of the player's unlocked pets.
        /// </summary>
        /// <param name="player">The player.</param>
        public void PrintPetsList(Player player)
        {
            string petsString = "Pets: ";
            bool hasPet = false;
            foreach (var v in player.PetsUnlocked)
            {
                if (v.Value)
                {
                    if (v.Key != PetType.None)
                    {
                        hasPet = true;
                        petsString += $"{v.Key}, ";
                    }
                }
            }

            if (!hasPet)
                petsString = "You have no pets";

            _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} {petsString}");
        }

        /// <summary>
        /// Switches the player's active pet.
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the pet type.</param>
        public void SwitchPet(Player player, string command, params string[] args)
        {
            PetType type = TwitchUtils.GetPetTypeFromString(args[0]);

            if (type == PetType.Count)
                return;

            if (player.PetsUnlocked[type])
            {
                player.Pet.TrySetActivePet(type);
                _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} pet switched!");
            }
        }

        /// <summary>
        /// Revives the player with a food cost.
        /// </summary>
        /// <param name="player">The player.</param>
        public void ReviveWithCost(Player player)
        {
            if (player.HealthHandler.Dead)
                if (player.HealthHandler.TryRevive(ReviveType.Self))
                    _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you have been successfully revived!");
                else
                    _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you cannot afford to revive (requires 400 food)!");
            else
                _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you have to be dead to revive");
        }

        /// <summary>
        /// Revives a target player with a food cost (Priest/Paladin only).
        /// </summary>
        /// <param name="player">The player.</param>
        /// <param name="command">The command.</param>
        /// <param name="args">The arguments containing the target player name.</param>
        public void RevivePlayerWithCost(Player player, string command, params string[] args)
        {
            if (player.RoleHandler.CurrentRole == PlayerRole.Priest || player.RoleHandler.CurrentRole == PlayerRole.Paladin)
                if (Utils.TwitchUtils.TryGetPlayer(args[0], out Player targetPlayer))
                    if (targetPlayer.HealthHandler.Dead && targetPlayer != player)
                        if (targetPlayer.HealthHandler.TryRevive(ReviveType.Others))
                        {
                            player.RoleHandler.PlayerRoleData.IncreaseExperience(targetPlayer.HealthHandler.MaxHealth);
                            _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you have successfully revived {targetPlayer.TwitchUser.Username}! how nice...");
                        }
                        else
                            _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you cannot afford to revive {targetPlayer.TwitchUser.Username} (requires 200 food)!");
                    else
                        _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} to revive others they must be dead! silly.");
                else
                    _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} cannot find player '{targetPlayer.TwitchUser.Username}'");
            else
                _twitchChatProcessor.SendMessage($"{player.TwitchUser.Username} you need to be role {PlayerRole.Priest} or {PlayerRole.Paladin} to revive other players!");
        }
    }
}
