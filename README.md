Minimum Durable (Riichi) Mahjong Arrays

This repository defines a storage data format for Mahjong hands.
Notably, this contains all private data for all players, and is only suitable for long term storage or server-side state tracking.

This does not track individual players choosing not to take certain interrupt actions, such as pon or chii, which will need to be contextually rehydrated by consumers.