define a Resource:
    it has name which is "unnamed".

    on create with label:
        name becomes label.
        say name + " created.".
    end.

    on destroy:
        say name + " destroyed.".
    end.
end.

say "--- create resource ---".
let res be instantiate Resource with "database".
say "--- resource in scope ---".
res becomes null.
say "--- destroy ran before this line ---".
stop.
