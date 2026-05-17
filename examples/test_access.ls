say "=== 1. List at access ===".
let fruits be a list containing "apple", "banana", "cherry".
say fruits at 0.
say fruits at 1.
say fruits at 2.

say "=== 2. Map at access ===".
let config be a map with "volume" as 80, "name" as "Alice".
say config at "volume".
say config at "name".

say "=== 3. List at assignment ===".
fruits at 0 becomes "orange".
say fruits at 0.

say "=== 4. Map at assignment ===".
config at "volume" becomes 100.
say config at "volume".

say "=== 5. Object member access (of) ===".
define a Person:
    it has name which is "Hero".
    it has age which is 0.

    on create with n, a:
        name becomes n.
        age becomes a.
    end.

    make name public.
    make age public.
end.

let p be instantiate Person with "Bob", 30.
say name of p.
say age of p.

say "=== 6. Object member assignment (of) ===".
age of p becomes 31.
say age of p.

say "=== 7. Chained access ===".
let data be a list containing config, config.
say (name of p) + " is " + age of p + " years old".

say "=== 8. Operator overload - accessed_at ===".
define a Vector:
    it has x which is 0.
    it has y which is 0.
    it has z which is 0.

    on create with vx, vy, vz:
        x becomes vx.
        y becomes vy.
        z becomes vz.
    end.

    when accessed at with index:
        when index is equal to 0:
            return x.
        end.
        when index is equal to 1:
            return y.
        end.
        when index is equal to 2:
            return z.
        end.
        raise "index out of range".
    end.

    when assigned at with index, value:
        when index is equal to 0:
            x becomes value.
        end.
        when index is equal to 1:
            y becomes value.
        end.
        when index is equal to 2:
            z becomes value.
        end.
    end.

    make accessed_at public.
    make assigned_at public.
end.

let v be instantiate Vector with 1, 2, 3.
say v at 0.
say v at 1.
say v at 2.
v at 0 becomes 10.
say v at 0.

say "=== ALL NEW FEATURE TESTS PASSED ===".
stop.
