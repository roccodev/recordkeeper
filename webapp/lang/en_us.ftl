open = Open
save = Save
undo = Undo
redo = Redo
download = Save & Download
ok = OK
yes = Yes
no = No

# The project's info page
nav_about = About
# The project's source code
nav_source = Source
# The project's software license
nav_license = License

-arts = Arts
-skills = Skills
-class = Class

# Objects without a registered name.
# $id: the ID of the object
unnamed = #{ $id } (No Name)

# Data related to the base game (no DLC)
menu_category_base = Game Data

# Data related to DLCs 1-3
menu_category_dlc = DLC Data

# Data related to Future Redeemed (DLC 4)
menu_category_dlc4 = Future Redeemed Data

# Data related to the save file itself
menu_category_meta = Metadata

# Dangerous settings that should not be changed by the average user
menu_category_danger = Danger Zone

menu_base_characters = Characters
menu_base_ouroboros = Ouroboros
menu_base_items = Items
menu_base_field = Field
menu_base_quests = Quests
menu_base_ums = Unique Monsters
menu_base_formations = Party Formations

menu_dlc_powaugment = Inoswap
menu_dlc_challenge = Challenge Battle
menu_dlc_gauntlet = Archsage's Gauntlet
menu_dlc_masha = Accessory Crafting

menu_dlc4_growth = Affinity Growth
menu_dlc4_collepedia = Collectopedia
menu_dlc4_enemypedia = Enemypedia

menu_meta_meta = Save Info

menu_danger_flags = Flags

## Save info translations

hours = Hours
minutes = Minutes
seconds = Seconds
date = Date
time = Time

difficulty = Difficulty
difficulty_easy = Easy
difficulty_normal = Normal
difficulty_hard = Hard
difficulty_veryhard = Very Hard (Unreleased)

scenario_flag_flag = Flag
scenario_flag_chapter = Chapter
# $id: the chapter ID
scenario_flag_chapter_id = Chapter { $id }

meta_playtime = Play Time
# When the game was last saved
meta_savetime = Save Time
meta_scenario_flag = Scenario Flag
meta_ngp = New Game Plus
meta_settings = Settings
meta_clear = Game Cleared
meta_fr_complete = Future Redeemed Cleared

## Ouroboros translations

ouroboros_sp = SP
ouroboros_linked_skills = Linked { -skills }
ouroboros_skill_tree = Skill Tree

## Flag screen

# The flag ID
flag_index = Flag Index
# How many bits the flag uses
flag_bits = Flag Bits
# The value stored in the flag
flag_value = Value
# Placeholder text for the flag ID input field.
# Allows the user to jump to the page that contains the flag
flag_jump_page = Go to flag...

## Items screen

item_type = Item Type
item_slot_index = Slot ID
item_item = Item
item_amount = Amount
item_actions = Actions

# You are searching item slots for items
item_search = Search item slots...
item_first_empty = Go to empty slot

# Item types

item_type_collection = Collectibles
item_type_accessory = Accessories
item_type_precious = Key Items
item_type_gem = Gems
item_type_cylinder = Cylinders
item_masha = Crafted Accessory
item_unnamed = (No Name)

# Item rarities

item_rarity_common = Common
item_rarity_rare = Rare
item_rarity_legendary = Legendary

# Accessory crafting

masha_item_type = Accessory Name
masha_level = Craft Level
masha_enhance = Effect
masha_boost_stat = Statistic
masha_boost_value = Value

masha_stat_hp = HP
masha_stat_str = Strength
masha_stat_heal = Healing Power
masha_stat_dex = Dexterity
masha_stat_agi = Agility
masha_stat_crit = Crit Rate
masha_stat_block = Block Rate

## Quest screen

quest_id = ID
quest_name = Name
quest_status = Status
quest_actions = Actions

quest_status_unstarted = Not Started
quest_status_progress = In Progress
quest_status_complete_a = Completed (A)
quest_status_complete_b = Completed (B)

quest_purpose = Edit Quest Objectives
quest_purpose_id = Objective ID
quest_purpose_status = Objective Status
quest_purpose_tasks_a = Tasks (Branch A)
quest_purpose_tasks_b = Tasks (Branch B)

quest_task_ask = Ask
quest_task_battle = Battle
quest_task_chase = Chase
quest_task_collect = Collect
quest_task_collepedia = Collectopedia
quest_task_condition = Condition
quest_task_event = Event
quest_task_follow = Follow
quest_task_gimmick = Gimmick
quest_task_reach = Location
quest_task_request = Item List
quest_task_talk = Talk


## Character screen

character_character = Character
character_party = Party Setup
character_level = Level
character_exp = EXP
character_bexp = Bonus EXP
character_selected_class = Edit/Selected Class

character_set_selectable = Selectable
character_set_permanent = Permanent
character_set_temp = Temporary

# The character's selected costume
character_costume = Costume

# The level the character first joined the party at
character_arrival_level = Initial Level

# Character clothes dirty level
character_dirt = Dirtiness

# Weapon decoration/eyepatch
character_attachment = Attachment

# $id: the art slot ID (0-6). 
# 0 = talent art
# 1-3 = Keves arts
# 4-6 = Agnus arts
character_art = 
    { $id ->
        [0] Talent Art
        [4] Agnus Art 1
        [5] Agnus Art 2
        [6] Agnus Art 3
       *[other] Keves Art { $id }
    }
# $id: the slot ID
character_skill = 
    { $id ->
        [0] Class Skill 1
        [1] Class Skill 2
        [2] Class Skill 3
        [3] Class Skill 4
        [4] Inherited Skill 1
        [5] Inherited Skill 2
        [6] Inherited Skill 3
        *[7] Unused Skill
    }
# $id: the slot ID
character_gem =
    { $id ->
        [0] Gem Type 1
        [1] Gem Type 2
        *[2] Gem Type 3
    }
# $id: the slot ID
character_accessory =
    { $id ->
        [0] Accessory 1
        [1] Accessory 2
        *[2] Accessory 3
    }

character_class_cp = CP
character_class_unlock = Unlock Points
character_class_rank = Rank

character_flag_unload_dlc_costume = DLC Costume Unloaded
character_flag_eyepatch = Show Eyepatch
character_flag_dlc4_ma_1 = Master Art 1 Unlock (FR)
character_flag_dlc4_ma_2 = Master Art 2 Unlock (FR)
character_flag_dlc4_ma_3 = Master Art 3 Unlock (FR)
character_flag_dlc4_gem_2 = Gem 2 Unlock (FR)
character_flag_dlc4_gem_3 = Gem 3 Unlock (FR)
character_flag_dlc4_acc_2 = Accessory 2 Unlock (FR)
character_flag_dlc4_acc_3 = Accessory 3 Unlock (FR)


## Ouroboros screen

ouroboros_enable = Enable Interlink
ouroboros_character = Character
# $id: the art slot ID (0-4). 
ouroboros_art = 
    { $id ->
        [0] Talent Art
        [4] Extra Talent Art
       *[1] Art { $id }
    }
# $id: the linked skill slot ID (0-1). 
ouroboros_skill = 
    { $id ->
        [0] Linked Skill 1
       *[1] Linked Skill 2
    }

ouroboros_share_slot = Linked Skill 2 Unlocked

ouroboros_tree_skill_unlock = Unlock Skill
ouroboros_tree_art_unlock = Unlock Art
ouroboros_tree_skill_upgrade = Upgrade Skill
ouroboros_tree_art_upgrade = Upgrade Art


## Field screen

field_tab_player = Player
field_tab_locations = Locations
field_tab_maps = Maps

field_player_pos = Player Position
field_ship_pos = Ship Position
field_aboard_ship = Aboard Ship

field_map = Map
x = X
y = Y
z = Z

field_environment = Environment
field_time_hour = Map Time (Hours)
field_time_minute = Map Time (Minutes)
field_weather = Map Weather
field_time_lock = Lock Time
field_weather_lock = Lock Weather (Required for changes to take effect.)

field_stats = Player
field_gold = Gold
field_respawn_point = Respawn Point
field_ether_progress = Ether Cylinder Progress%
field_ether_progress_dx = DX Ether Cylinder Progress%

field_location_visited = Visited
field_location_id = ID
field_location_type = Type
field_location_name = Name
field_location_all_on = All On
field_location_all_off = All Off
field_location_actions = Actions
field_location_respawn = Set as respawn point
field_location_teleport = Teleport here

field_location_type_region = Region
field_location_type_location = Location
field_location_type_landmark = Landmark
field_location_type_secret = Secret Area
field_location_type_colony = Colony
field_location_type_camp = Rest Spot
field_location_type_respawn = Respawn Point


## Unique Monsters screen

enemy_id = ID
enemy_name = Name
enemy_seen = Seen
enemy_defeat = Defeated
enemy_rematch = Rematch Level
enemy_time = Best Time
enemy_rematch_time = Best Time (Max Level)

enemy_tab_records = Records
enemy_tab_soulhack_arts = Soulhacker Arts
enemy_tab_soulhack_skills = Soulhacker Skills

enemy_soul_hack_id = ID
enemy_soul_hack_name = Name
enemy_soul_hack_unlocked = Unlocked
enemy_soul_hack_progress = Upgrade Progress
enemy_soul_hack_upgraded = Upgraded


## Party Formations screens

formation_back = Back to list
formation_edit = Edit
formation_delete = Delete
formation_create = New from save
formation_copy = Copy existing
formation_empty = Empty
# $id: the formation ID
formation_current = Editing Formation { $id }

formation_tab_characters = Characters
formation_tab_ouroboros = Ouroboros

formation_class_change = Edit/Selected Class
formation_class_confirm = There is no data for the selected class. Would you like to copy it from the current state of the save file?

## Pow augment (Inoswap) screen

pow_augment_character = Character
pow_augment_tiers = Unlocked Tiers
pow_augment_node_skill_unlock = Unlock Skill
pow_augment_node_art_unlock = Unlock Art
pow_augment_node_skill_upgrade = Upgrade Skill
pow_augment_node_art_upgrade = Upgrade Art


## Challenge battle screen

challenge_id = ID
challenge_name = Name
challenge_bonus = 3x Bonus
challenge_rank = Rank
challenge_rank_none = --
challenge_rank_s = S
challenge_rank_a = A
challenge_rank_b = B
challenge_rank_c = C
challenge_time = Best Time
challenge_clear_count = Clear Count
challenge_stone = Red Noponstone