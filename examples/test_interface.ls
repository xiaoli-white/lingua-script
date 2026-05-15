define an interface Speakable:
    can speak.
    can get_name.
end.

define an interface Runnable:
    can start_run.
end.

define a Dog implements Speakable, Runnable:
    it has name which is "Doggy".
    on create with n:
        name becomes n.
    end.
    to speak:
        say name + " says woof!"
    end.
    to get_name:
        return name.
    end.
    to start_run:
        say name + " is running!"
    end.
    make speak public.
    make get_name public.
    make start_run public.
end.

let d be instantiate Dog with "Rex".
speak using d.
let n be get_name using d.
say "name: " + n.
start_run using d.
