say "=== 1. Number Words ===".
let a be one hundred.
say a.
let b be two thousand five hundred.
say b.
let c be three point one four.
say c.
let d be half.
say d.
let e be 35 thousand.
say e.

say "=== 2. Variable Definition ===".
let x be ten.
say x.
score is zero.
say score.

say "=== 3. Assignment ===".
score becomes one hundred.
say score.

say "=== 4. Arithmetic ===".
let p be ten.
let q be five.
say p added to q.
say p subtracted from q.
say p subtracted by q.
say p multiplied by q.
say p divided by q.
let r6 be remainder of p divided by q.
say r6.
let r7 be square of q.
say r7.

say "=== 5. Comparison ===".
when ten is greater than five:
    say "gt ok".
end.
when five is less than ten:
    say "lt ok".
end.
when ten is equal to ten:
    say "eq ok".
end.
when five is not equal to ten:
    say "ne ok".
end.
when ten isnt five:
    say "isnt ok".
end.
when ten is greater than or equal to five:
    say "ge ok".
end.
when three is less than or equal to ten:
    say "le ok".
end.

say "=== 6. Logic ===".
when true and true:
    say "and ok".
end.
when true or false:
    say "or ok".
end.
when not false:
    say "not ok".
end.

say "=== 7. Control Flow ===".
when ten is greater than five:
    say "when ok".
otherwise:
    say "when else fail".
end.

let count be zero.
repeat three times:
    count becomes count added to one.
end.
say count.

let i be zero.
while i is less than three:
    i becomes i added to one.
end.
say i.

let items be a list containing "a", "b", "c".
for each item in items:
    say item.
end.

say "=== 8. List ===".
let fruits be a list containing "apple", "banana".
say fruits.
add "cherry" to fruits.
say fruits.
remove "banana" from fruits.
say fruits.

say "=== 9. Map ===".
let cfg be a map with "volume" as 80, "difficulty" as "Hard".
say cfg.

say "=== 10. Type Query and Conversion ===".
let type1 be type of 42.
say type1.
let type2 be type of "hello".
say type2.
let type3 be type of true.
say type3.
convert "50" to number and save to n.
say n.
convert 42 to string and save to s.
say s.

say "=== 11. Functions ===".
to greet with name:
    say "Hello, " + name + "!".
end.
run greet with "Alice" and save to _.

to add with a, b:
    return a added to b.
end.
run add with 3, 4 and save to sum.
say sum.

say "=== 12. Classes and Objects ===".
define a Counter:
    it has value which is 0.

    on create with initial:
        value becomes initial.
    end.

    to increment with amount:
        value becomes value added to amount.
    end.

    to get_value:
        return value.
    end.

    make increment public.
    make get_value public.
end.

let c1 be instantiate Counter with 10.
let v1 be get_value using c1.
say v1.
increment using c1 with 5.
let v2 be get_value using c1.
say v2.

say "=== 13. Fresh ===".
let c2 be fresh Counter with 100.
let v3 be get_value using c2.
say v3.

say "=== 14. Exception Handling ===".
beware:
    say "try block".
    raise "test error".
    say "should not print".
in case of error:
    say "caught in beware".
regardless:
    say "beware finally".
end.

attempt to:
    say "attempt block".
    raise "attempt fail".
if it fails:
    say "attempt caught".
regardless:
    say "attempt finally".
end.

say "=== ALL TESTS PASSED ===".
stop.
