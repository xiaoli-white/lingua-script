define an interface Speakable:
    can speak.
    can get_name.
end.

define a Dog implements Speakable:
    it has name which is "Doggy".
    to speak:
        say name + " says woof!"
    end.
    to get_name:
        return name.
    end.
    make speak public.
    make get_name public.
end.

let d be instantiate Dog with "Rex".
speak using d.
let n be get_name using d.
say "name: " + n.
