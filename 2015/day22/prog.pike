#!/usr/bin/env pike

int MAGIC_MISSILE = 0;
int DRAIN = 1;
int SHIELD = 2;
int POISON = 3;
int RECHARGE = 4;
int MAX_SPELLS = 5;

array(string) SPELL_NAMES = ({ "magic missile", "drain", "shield", "poison", "recharge" });
array(int) SPELL_COSTS = ({ 53, 73, 113, 173, 229 });
array(function(State,int:void)) SPELL_CAST_PROCS = ({
  cast_magic_missile, cast_drain, cast_shield, cast_poison, cast_recharge
});

void run(string filename) {
  State state = load_init_state(filename);
  array(State) final_states = run_all_rounds_with_optimizing(state);
  array(State) winning_states = Array.filter(final_states, lambda(State s) { return s.is_victory(); });
  array(int) winning_state_spent = Array.map(winning_states, lambda(State s) { return s.spent_mana; });
  Array.sort(winning_state_spent, winning_states);
  write("Cheapest winner:\n");
  winning_states[0]->show();
  write("\n");
  /*
  // doesn't do much given the optimizations:
  write("Most expensive winner:\n");
  winning_states[-1]->show();
  write("\n");
  */
}

class State {
  int boss_hp;
  array(int) boss_hp_history = ({});
  int boss_damage;

  int pc_hp;
  array(int) pc_hp_history = ({});
  int pc_mana;
  
  int difficulty;

  array(int) timers = ({ 0, 0, 0, 0, 0 });

  array(int) spells_cast = ({});
  int spent_mana;
  int known_best;

  int is_final() {
    return is_victory() || is_loss() || /*is_trending_bad() ||*/ doesnt_beat_known_best();
  }

  int is_victory() {
    return this.boss_hp <= 0;
  }

  int is_loss() {
    return this.pc_hp <= 0;
  }

  int doesnt_beat_known_best() {
    // this optimization is the only way I got it to finish in reasonable time
    return this.spent_mana > this.known_best;
  }

  int is_trending_bad() {
    int skip_first = 3;
    int window = 4;
    if (sizeof(this.pc_hp_history) < skip_first + window) {
      return false;
    }
    
    int total = 0;
    for (int i = 0; i < window; i++) {
      int idx = sizeof(this.pc_hp_history) - 1 - i;
      total += (this.pc_hp_history[idx] - this.boss_hp_history[idx]);
    }
    total /= 4;

    return (total < 2);
  }

  State copy() {
    State state = State();
    state.boss_hp = this.boss_hp;
    state.boss_hp_history = this.boss_hp_history[0..];
    state.boss_damage = this.boss_damage;
    state.pc_hp = this.pc_hp;
    state.pc_hp_history = this.pc_hp_history[0..];
    state.pc_mana = this.pc_mana;
    state.difficulty = this.difficulty;
    state.timers = this.timers[0..];
    state.spells_cast = this.spells_cast[0..];
    state.spent_mana = this.spent_mana;
    state.known_best = this.known_best;
    return state;
  }

  void show() {
    write("Boss hp: %d, damage: %d\n", this.boss_hp, this.boss_damage);
    write("PC hp: %d, mana: %d (spent %d)\n", this.pc_hp, this.pc_mana, this.spent_mana);
    write("Active effects:");
    for (int i = 0; i < MAX_SPELLS; i++) {
      if (this.timers[i] > 0) {
        write(" %s (%d turns)", SPELL_NAMES[i], this.timers[i]);
      }
    }
    write("\n");
    write("Spells cast:");
    for (int i = 0; i < sizeof(this.spells_cast); i++) {
      write(" %s", SPELL_NAMES[this.spells_cast[i]]);
    }
    write("\n");
  }
}

State load_init_state(string filename) {
  State state = State();
  state.pc_hp = 50;
  state.pc_mana = 500;

  Stdio.FILE file = Stdio.FILE(filename);
  while (string line = file.gets()) {
    if (!sscanf(line, "Hit Points: %d", state.boss_hp) &&
        !sscanf(line, "Damage: %d", state.boss_damage) &&
        !sscanf(line, "Difficulty: %d", state.difficulty) &&
        !sscanf(line, "Hero HP: %d", state.pc_hp) &&
        !sscanf(line, "Hero Mana: %d", state.pc_mana)) {
      write("Bad line: %s\n", line);
    }
  }

  return state;
}

class Retry {
  int new_best = -1;
}

array(State) run_all_rounds_with_optimizing(State init_state) {
  int cur_best = 9999999;
  while (1) {
    State cur_init_state = init_state.copy();
    cur_init_state.known_best = cur_best;
    
    mixed error = catch {
      return run_all_rounds(cur_init_state);
    };

    if (error == 0) {
      write("Shouldn't ever get here!\n");
    } else {
      if (arrayp(error)) {
        throw(error);
      // not sure how to test for Retry specifically
      } else if (objectp(error)) {
        cur_best = error.new_best;
        continue;
      } else {
        throw(error); // should never happen, but what do I know
      }
    }
  }

  write("Shouldn't ever get here!\n");
  return 0;
}

array(State) run_all_rounds(State init_state) {
  // since our round-running code runs the PC's preturn effects
  // at the end, we need to run it once before we begin
  State preturn_state = init_state.copy();
  tick_effects(preturn_state, 1);
  
  if (preturn_state.is_final()) {
    return ({preturn_state});
  }

  array(State) final_states = ({});
  
  array(State) working_states = ({preturn_state});
  while (sizeof(working_states) > 0) {
    State cur = working_states[0];
    working_states -= ({cur});
    for (int s = 0; s < MAX_SPELLS; s++) {
      State new_state = run_round_with(cur, s);
      if (new_state == 0) {
        continue;
      } else if (new_state.is_final()) {
        final_states += ({new_state});
        if (new_state.is_victory() && new_state.spent_mana < init_state.known_best) {
          write("Found better new state with %d mana\n", new_state.spent_mana);
          new_state.show();
          write("\n");
          Retry retry = Retry();
          retry.new_best = new_state.spent_mana;
          throw(retry);
        }
      } else {
        working_states += ({new_state});
      }
    }
  }

  return final_states;
}

State run_round_with(State prev_state, int spell_num) {
  if (SPELL_COSTS[spell_num] > prev_state.pc_mana ||
      prev_state.timers[spell_num] > 0) {
    // this is an illegal branch, give up
    return 0;
  }

  State cur_state = prev_state.copy();
  cur_state.pc_mana -= SPELL_COSTS[spell_num];
  cur_state.spent_mana += SPELL_COSTS[spell_num];
  cur_state.spells_cast += ({spell_num});
  SPELL_CAST_PROCS[spell_num](cur_state, 1);

  if (cur_state.is_final()) {
    return cur_state;
  }

  // now the boss's turn starts:
  tick_effects(cur_state, 0);

  if (cur_state.is_final()) {
    return cur_state;
  }

  int boss_damage = cur_state.boss_damage;
  if (cur_state.timers[SHIELD] > 0) {
    boss_damage -= 7;
  }
  cur_state.pc_hp -= boss_damage;

  if (cur_state.is_final()) {
    return cur_state;
  }

  // now start player's next turn
  tick_effects(cur_state, 1);

  if (cur_state.is_final()) {
    return cur_state;
  }

  cur_state.boss_hp_history += ({cur_state.boss_hp});
  cur_state.pc_hp_history += ({cur_state.pc_hp});

  return cur_state;
}

void tick_effects(State state, int pc_turn) {
  if (state.difficulty > 0 && pc_turn) {
    state.pc_hp -= 1;
  }

  for (int t = 0; t < MAX_SPELLS; t++) {
    if (state.timers[t] > 0) {
      SPELL_CAST_PROCS[t](state, 0);
      state.timers[t]--;
    }
  }
}

void cast_magic_missile(State state, int casting) {
  if (casting) {
    state.boss_hp -= 4;
  }
}

void cast_drain(State state, int casting) {
  if (casting) {
    state.boss_hp -= 2;
    state.pc_hp += 2;
  }
}

void cast_shield(State state, int casting) {
  if (casting) {
    state.timers[SHIELD] += 6;
  }
}

void cast_poison(State state, int casting) {
  if (casting) {
    state.timers[POISON] += 6;
  } else {
    state.boss_hp -= 3;
  }
}

void cast_recharge(State state, int casting) {
  if (casting) {
    state.timers[RECHARGE] += 5;
  } else {
    state.pc_mana += 101;
  }
}

int main(int argc, array(string) argv) {
  run(argv[1]);
  return 0;
}
