say "=== Tests Start ===".
let a be one hundred.
say a.
let b be two thousand five hundred.
say b.
let c be three point one four.
say c.
score is zero.
say score.
score becomes one hundred.
say score.
let x be ten.
let y be five.
let z be x added to y.
say "x + y: " + z.
let w be x subtracted from y.
say "y - x: " + w.
let v be x multiplied by y.
say "x * y: " + v.
let u be x divided by y.
say "x / y: " + u.
when x is greater than y:
    say "x > y: yes".
otherwise:
    say "x > y: no".
end.
say "=== repeat ===".
let count be zero.
repeat five times:
    count becomes count added to one.
    say count.
end.
let fruits be a list containing "apple", "banana".
say fruits.
add "cherry" to fruits.
say fruits.
let cfg be a map with "volume" as 80, "difficulty" as "Hard".
say cfg.
to greet with name:
    say "Hello, " + name + "!".
end.
run greet with "Alice" and save to _.
let t1 be type of 42.
say "type of 42: " + t1.
let t2 be type of "hello".
say "type of hello: " + t2.
convert "50" to number and save to n.
say "converted 50: " + n.
say "=== while ===".
let i be zero.
while i is less than three:
    say i.
    i becomes i added to one.
end.
stop.
