define an interface Named:
    can get_name.
end.

define an interface Aged:
    can get_age.
end.

define a Animal:
    it has name which is "unknown".
    it has age which is 0.
    on create with n, a:
        name becomes n.
        age becomes a.
    end.
    to get_name:
        return name.
    end.
    to make_sound:
        say name + " makes a sound."
    end.
    make get_name public.
    make make_sound public.
end.

define a Dog extends Animal implements Aged:
    on create with n, a:
        name becomes n.
        age becomes a.
    end.
    to get_age:
        return age.
    end.
    to make_sound:
        say name + " barks!"
    end.
    make get_age public.
    make make_sound public.
end.

let d be instantiate Dog with "Rex", 3.
let n be get_name using d.
say "name: " + n.
let a be get_age using d.
say "age: " + a.
make_sound using d.
