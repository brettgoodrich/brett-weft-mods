Mod Spec: THREAT!
=

*I'm still getting a handle on Weft/Rust, so I've decided to write a spec before I start the mod so I can get feedback from the jump.*

THREAT! Implements a Threat mechanic for each hostile mob, akin to World of Warcraft. Every mob should have an amount of Threat, a number, for every player currently nearby. They choose who to aggro by their Threat rule (ie highest).

Events like damage done by the player, healing done by the player, invisibility changes, and others should be able to change the amount of Threat the enemy has towards that player, and Threat level changes should be able to change a mob's aggro target according to rules.

It should be implemented in the most extensible way possible. It should be easy to adjust the values for what threatens a mob and how much, easy for new mobs to hook into and use the system, and when initially implemented function very similarly to vanilla.

Some initial ideas on what adds Threat: seeing the player, player damage to mob, player healing.

Players should have a Goad factor that affects how much their personal actions affect Threat on another player, such that certain weapons/enchantments or wearing certain armor types affects a player's Goad factor, and, thus, how much Threat their actions add or subtract.

It should be possible for the rules on switching targets to be made more complicated than simple most-first; for instance, a mob might require 10 more Threat rather than 1 more Threat to consider changing targets, or always wait five more seconds, instead of being willing to change target right after a new Threat leader emerges.

An illustrative idea on a rule for Threat change from player damage to mob:
`actor_damage_done_to_mob x actor_goad_factor = added Threat`

Extension: all these values and actions should be by Actor instead of by "Player", such that mobs can have threat against each other. For instance, a player's dog, or a skeleton that accidentally shoots another mob.

Working on a Github and init for this.
