define a Animal:
    it has name which is "unknown".
    it has age which is 0.
    on create with n, a:
        name becomes n.
        age becomes a.
    end.
    to speak:
        say name + " makes a sound."
    end.
    to get_age:
        return age.
    end.
    make speak public.
    make get_age public.
end.

define a Dog extends Animal:
    it has breed which is "unknown".
    on create with n, a, b:
        name becomes n.
        age becomes a.
        breed becomes b.
    end.
    to speak:
        say name + " barks!"
    end.
    make speak public.
end.

let a be instantiate Animal with "Generic", 5.
say "--- animal ---".
speak using a.
let age_a be get_age using a.
say "age: " + age_a.

let d be instantiate Dog with "Rex", 3, "Husky".
say "--- dog ---".
speak using d.
let age_d be get_age using d.
say "age: " + age_d.
