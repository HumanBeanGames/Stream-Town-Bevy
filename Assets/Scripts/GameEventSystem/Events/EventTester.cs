using Character;
using GameEventSystem.Events.Voting;
using Twitch;
using Processors;
using UnityEngine;
using UnityEngine.InputSystem;

namespace GameEventSystem.Events
{
    /// <summary>
    /// Test class for game events. Used for debugging and testing event functionality.
    /// </summary>
	public class EventTester : MonoBehaviour
	{
        /// <summary>
        /// The game event processor.
        /// </summary>
		private GameEventProcessor processor;

        /// <summary>
        /// Initializes the event tester.
        /// </summary>
		private void Awake()
		{
			processor = GetComponent<GameEventProcessor>();
		}

        /// <summary>
        /// Updates the event tester and processes events.
        /// </summary>
		private void Update()
		{
			processor.ProcessEvents();
			//Start vote
			//if (Keyboard.current.oKey.wasReleasedThisFrame)
			//{
			//	//VoteEvent ev = new VoteEvent(0, 60);			
			//	//ev.AddOption(new VoteOption("yes", null));
			//	//ev.AddOption(new VoteOption("no", null));
			//	//ev.EventEnded += PrintVoteResults;
			//	//processor.AddEvent(ev);
			//	//ev.AddOption(new VoteOption("playerName", null));

			//	// New King Vote
			//	NewKingVote ev = new NewKingVote(0, 20);
			//	ev.EventEnded += PrintVoteResults;
			//	processor.AddEvent(ev);
			//	Debug.Log("New King Vote Event Added!");
			//}

			//if (Keyboard.current.kKey.wasReleasedThisFrame)
			//{
			//	////Create fake vote
			//	//VoteEvent currentEvent = processor.CurrentEvent as VoteEvent;
			//	//string randomVal = Random.Range(0, 1000).ToString();
			//	//bool yes = Random.Range(0, 2) == 1 ? true : false;
			//	//currentEvent.Action(new PlayerVote(new Player(new TwitchUser(randomVal, randomVal)), new VoteOption(yes ? "yes" : "no", null)));

			//	// Fish God
			//	if(processor.CurrentEvent == null)
			//	{
			//		FishGodEvent fishgodEvent = new FishGodEvent(0);
			//		processor.AddEvent(fishgodEvent);
			//	}
			//	else
			//	{
			//		FishGodEvent ev = processor.CurrentEvent as FishGodEvent;
			//		ev.Action();
			//	}
			//}
		}

        /// <summary>
        /// Prints the vote results to the debug log.
        /// </summary>
        /// <param name="b">Whether the vote was successful.</param>
        /// <param name="t">The event type.</param>
        /// <param name="data">The vote data.</param>
		private void PrintVoteResults(bool b, GameEvent.EventType t, object data)
		{
			if(data == null)
			{
				Debug.Log($"No Votes Found");
				return;

			}
			VoteOption voteOption = data as VoteOption;

			Debug.Log($"Winning Vote: '{voteOption.OptionName}' with {voteOption.Votes}");
		}
	}
}
