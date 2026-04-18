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
        /// <summary>
        /// The player processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private PlayerProcessor _playerProcessor;
        /// <summary>
        /// The game event processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private GameEventProcessor _gameEventProcessor;
        /// <summary>
        /// The util display processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private UtilDisplayProcessor _utilDisplayProcessor;
        /// <summary>
        /// The Twitch client. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private TL_Client _tlClient;
        /// <summary>
        /// The message sender. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private MessageSender _messageSender;
        /// <summary>
        /// The event commands. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private EventCommands _eventCommands;
        /// <summary>
        /// The role processor. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private RoleProcessor _roleProcessor;
        /// <summary>
        /// The game coordinator. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private Coordinator _coordinator;
        /// <summary>
        /// The game settings scriptable. Injected via Reflex dependency injection.
        /// </summary>
        [Inject] private GameSettings _gameSettings;

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
                _tlClient.UserIsSubscribed(player.TwitchUser.UserID);

            _messageSender.SendPreBuiltMessage(user.Username, "characterCreated");
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
                    _messageSender.SendPlayerMessage(player, "Hair Style Changed!");
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
                    _messageSender.SendPlayerMessage(player, "Eye Style Changed!");
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
                    _messageSender.SendPlayerMessage(player, "Facial Hair Style Changed!");
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
                    _messageSender.SendPlayerMessage(player, "Body Type Changed!");
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
                    _messageSender.SendPlayerMessage(player, "Hair Color Changed!");
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
                    _messageSender.SendPlayerMessage(player, "Eye Color Changed!");
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
            var currentEvent = _gameEventProcessor.CurrentEvent;

            if (currentEvent == null || !(currentEvent is VoteEvent))
            {
                _messageSender.SendPlayerMessage(player, "Failed - No Vote Active");
                return;
            }

            VoteEvent voteEvent = (VoteEvent)currentEvent;

            if (voteEvent.HasVoted(player))
            {
                _messageSender.SendPlayerMessage(player, "Failed - You have already voted!");
                return;
            }

            voteEvent.Action(new PlayerVote(player, new VoteOption(args[0], null)));
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

            _messageSender.SendMessage($"{player.TwitchUser.Username} {petsString}");
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
                _messageSender.SendMessage($"{player.TwitchUser.Username} pet switched!");
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
                    _messageSender.SendMessage($"{player.TwitchUser.Username} you have been successfully revived!");
                else
                    _messageSender.SendMessage($"{player.TwitchUser.Username} you cannot afford to revive (requires 400 food)!");
            else
                _messageSender.SendMessage($"{player.TwitchUser.Username} you have to be dead to revive");
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
                            _messageSender.SendMessage($"{player.TwitchUser.Username} you have successfully revived {targetPlayer.TwitchUser.Username}! how nice...");
                        }
                        else
                            _messageSender.SendMessage($"{player.TwitchUser.Username} you cannot afford to revive {targetPlayer.TwitchUser.Username} (requires 200 food)!");
                    else
                        _messageSender.SendMessage($"{player.TwitchUser.Username} to revive others they must be dead! silly.");
                else
                    _messageSender.SendMessage($"{player.TwitchUser.Username} cannot find player '{targetPlayer.TwitchUser.Username}'");
            else
                _messageSender.SendMessage($"{player.TwitchUser.Username} you need to be role {PlayerRole.Priest} or {PlayerRole.Paladin} to revive other players!");
        }
    }
}
