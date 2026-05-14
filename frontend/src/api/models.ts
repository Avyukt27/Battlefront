export enum Status {
  Bleed,
  Poison,
}

export enum PlayerClass {
  Gunslinger,
  Arsenist,
  Mage,
  Knight,
  Assassin,
}

export type CardAbility = { DamageMul: { multiplier: number; threshold: number } };

export type CardEffect =
  | { Damage: { power: number } }
  | { Heal: { amount: number } }
  | { SkillCheck: { threshold: number } }
  | { ApplyStatus: { status: Status; duration: number } }
  | { CureStatus: { status: Status } }
  | { Range: { max_range: number } }
  | { Shield: { value: number } }
  | { Ability: { ability: CardAbility; cooldown: number } };

export interface Card {
  id: string;
  name: string;
  is_signature: boolean;
  cooldown: number;
  effects: CardEffect[];
}

export interface ActiveEffect {
  status: Status;
  duration: number;
}

export interface Player {
  id: number;
  colour: 'Red' | 'Blue' | 'Green';
  x: number;
  y: number;
  health: number;
  max_health: number;
  shield: number;
  status_effects: ActiveEffect[];
  class: PlayerClass;
  cards: Card[];
}

export interface GameState {
  players: Player[];
  current_turn: string;
  last_roll: number;
  width: number;
  height: number;
}
