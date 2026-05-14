define a Player:
    it has name which is "Hero".
    it has hp which is 100.
    on create with player_name:
        name becomes player_name.
        hp becomes 100.
    end.
    to take_damage with amount:
        hp becomes hp subtracted by amount.
        when hp is less than zero:
            hp becomes zero.
            say name + " has fallen."
        otherwise:
            say name + " has " + hp + " HP left."
        end.
    end.
    to get_hp:
        return hp.
    end.
    make take_damage public.
    make get_hp public.
end.
let warrior be instantiate Player with "Arthur".
say "--- test get_hp ---".
let h be get_hp using warrior.
say "HP: " + h.
say "--- test take_damage ---".
take_damage using warrior with 30.
take_damage using warrior with 80.
