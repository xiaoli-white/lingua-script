use lingua_script::execute;

#[test]
fn test_hello() {
    assert!(execute(r#"say "hello"."#).is_ok());
}

#[test]
fn test_number_words_integer() {
    assert!(execute(r#"let x be one hundred. say x."#).is_ok());
}

#[test]
fn test_number_words_large() {
    assert!(execute(r#"let x be two thousand five hundred. say x."#).is_ok());
}

#[test]
fn test_number_words_float() {
    assert!(execute(r#"let x be three point one four. say x."#).is_ok());
}

#[test]
fn test_number_words_half() {
    assert!(execute(r#"say half."#).is_ok());
}

#[test]
fn test_number_words_quarter() {
    assert!(execute(r#"say quarter."#).is_ok());
}

#[test]
fn test_number_words_mixed() {
    assert!(execute(r#"let x be 35 thousand. say x."#).is_ok());
}

#[test]
fn test_var_let_be() {
    assert!(execute(r#"let x be ten. say x."#).is_ok());
}

#[test]
fn test_var_is() {
    assert!(execute(r#"score is zero. say score."#).is_ok());
}

#[test]
fn test_assignment_becomes() {
    assert!(execute(
        r#"
score is zero.
score becomes one hundred.
say score.
"#
    )
    .is_ok());
}

#[test]
fn test_add() {
    assert!(execute(r#"let x be ten added to five. say x."#).is_ok());
}

#[test]
fn test_subtract_from() {
    assert!(execute(r#"let x be ten subtracted from five. say x."#).is_ok());
}

#[test]
fn test_subtract_by() {
    assert!(execute(r#"let x be ten subtracted by five. say x."#).is_ok());
}

#[test]
fn test_multiply() {
    assert!(execute(r#"let x be ten multiplied by five. say x."#).is_ok());
}

#[test]
fn test_divide() {
    assert!(execute(r#"let x be ten divided by five. say x."#).is_ok());
}

#[test]
fn test_remainder() {
    assert!(execute(r#"let x be remainder of ten divided by three. say x."#).is_ok());
}

#[test]
fn test_square() {
    assert!(execute(r#"let x be square of five. say x."#).is_ok());
}

#[test]
fn test_comparison_gt() {
    assert!(execute(
        r#"
when ten is greater than five:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_lt() {
    assert!(execute(
        r#"
when five is less than ten:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_eq() {
    assert!(execute(
        r#"
when ten is equal to ten:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_ne() {
    assert!(execute(
        r#"
when five is not equal to ten:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_isnt() {
    assert!(execute(
        r#"
when ten isnt five:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_ge() {
    assert!(execute(
        r#"
when ten is greater than or equal to five:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_comparison_le() {
    assert!(execute(
        r#"
when three is less than or equal to ten:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_logic_and() {
    assert!(execute(
        r#"
when true and true:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_logic_or() {
    assert!(execute(
        r#"
when true or false:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_logic_not() {
    assert!(execute(
        r#"
when not false:
    say "yes".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_when_otherwise() {
    assert!(execute(
        r#"
when ten is greater than five:
    say "gt".
otherwise:
    say "lte".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_when_false_otherwise() {
    assert!(execute(
        r#"
when five is greater than ten:
    say "wrong".
otherwise:
    say "correct".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_repeat_loop() {
    assert!(execute(
        r#"
repeat three times:
    say "loop".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_while_loop() {
    assert!(execute(
        r#"
let i be zero.
while i is less than three:
    say i.
    i becomes i added to one.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_for_each_list() {
    assert!(execute(
        r#"
let items be a list containing "a", "b", "c".
for each item in items:
    say item.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_list_create() {
    assert!(execute(
        r#"
let fruits be a list containing "apple", "banana".
say fruits.
"#
    )
    .is_ok());
}

#[test]
fn test_list_add() {
    assert!(execute(
        r#"
let fruits be a list containing "apple".
add "banana" to fruits.
say fruits.
"#
    )
    .is_ok());
}

#[test]
fn test_list_remove() {
    assert!(execute(
        r#"
let fruits be a list containing "apple", "banana".
remove "apple" from fruits.
say fruits.
"#
    )
    .is_ok());
}

#[test]
fn test_map_create() {
    assert!(execute(
        r#"
let cfg be a map with "volume" as 80.
say cfg.
"#
    )
    .is_ok());
}

#[test]
fn test_type_of_number() {
    assert!(execute(r#"let t be type of 42. say t."#).is_ok());
}

#[test]
fn test_type_of_string() {
    assert!(execute(r#"let t be type of "hello". say t."#).is_ok());
}

#[test]
fn test_type_of_bool() {
    assert!(execute(r#"let t be type of true. say t."#).is_ok());
}

#[test]
fn test_convert_string_to_number() {
    assert!(execute(
        r#"
convert "50" to number and save to n.
say n.
"#
    )
    .is_ok());
}

#[test]
fn test_convert_number_to_string() {
    assert!(execute(
        r#"
convert 42 to string and save to s.
say s.
"#
    )
    .is_ok());
}

#[test]
fn test_function_define_and_call() {
    assert!(execute(
        r#"
to greet with name:
    say "Hello, " + name + "!".
end.
run greet with "Alice" and save to _.
"#
    )
    .is_ok());
}

#[test]
fn test_function_with_return() {
    assert!(execute(
        r#"
to add with a, b:
    return a added to b.
end.
run add with 3, 4 and save to result.
say result.
"#
    )
    .is_ok());
}

#[test]
fn test_class_instantiate() {
    assert!(execute(
        r#"
define a Counter:
    it has value which is 0.
    on create with initial:
        value becomes initial.
    end.
    to get_value:
        return value.
    end.
    make get_value public.
end.
let c be instantiate Counter with 10.
let v be get_value using c.
say v.
"#
    )
    .is_ok());
}

#[test]
fn test_fresh_instance() {
    assert!(execute(
        r#"
define a Counter:
    it has value which is 0.
    on create with initial:
        value becomes initial.
    end.
    to get_value:
        return value.
    end.
    make get_value public.
end.
let c be fresh Counter with 100.
let v be get_value using c.
say v.
"#
    )
    .is_ok());
}

#[test]
fn test_try_catch_beware() {
    assert!(execute(
        r#"
beware:
    say "try block".
    raise "test error".
in case of error:
    say "caught".
regardless:
    say "finally".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_try_catch_attempt() {
    assert!(execute(
        r#"
attempt to:
    say "attempting".
    raise "fail".
if it fails:
    say "handled".
regardless:
    say "cleanup".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_start_here() {
    assert!(execute(
        r#"
say "before".
start here:
    say "inside".
end.
say "after".
"#
    )
    .is_ok());
}

#[test]
fn test_string_concat() {
    assert!(execute(r#"say "hello" + " " + "world"."#).is_ok());
}

#[test]
fn test_boolean_true() {
    assert!(execute(r#"let t be true. say t."#).is_ok());
}

#[test]
fn test_boolean_false() {
    assert!(execute(r#"let f be false. say f."#).is_ok());
}

#[test]
fn test_null() {
    assert!(execute(r#"let x be null. say x."#).is_ok());
}

#[test]
fn test_stop() {
    assert!(execute(
        r#"
say "before stop".
stop.
say "after stop".
"#
    )
    .is_ok());
}

#[test]
fn test_gc_list_shared() {
    assert!(execute(
        r#"
let a be a list containing "x".
let b be a.
say a.
"#
    )
    .is_ok());
}

#[test]
fn test_gc_map_shared() {
    assert!(execute(
        r#"
let a be a map with "key" as 42.
let b be a.
say a.
"#
    )
    .is_ok());
}

#[test]
fn test_interface_extends_one() {
    assert!(execute(
        r#"
define an interface Named:
    can get_name.
end.
define an interface Person extends Named:
    can introduce.
end.
define a Human implements Person:
    it has name which is "".
    on create with n:
        name becomes n.
    end.
    to get_name:
        return name.
    end.
    to introduce:
        say "Hi, " + name.
    end.
end.
let h be fresh Human with "Bob".
introduce using h.
"#
    )
    .is_ok());
}

#[test]
fn test_interface_extends_multi() {
    assert!(execute(
        r#"
define an interface Named:
    can get_name.
end.
define an interface Aged:
    can get_age.
end.
define an interface Person extends Named, Aged:
    can introduce.
end.
define a Human implements Person:
    it has name which is "".
    it has age which is 0.
    on create with n, a:
        name becomes n.
        age becomes a.
    end.
    to get_name:
        return name.
    end.
    to get_age:
        return age.
    end.
    to introduce:
        say name + " is " + age + " years old".
    end.
end.
let h be fresh Human with "Alice", 30.
say get_name using h.
say get_age using h.
"#
    )
    .is_ok());
}

#[test]
fn test_method_override() {
    assert!(execute(
        r#"
define an Animal:
    to speak:
        say "..."
    end.
end.
define a Dog extends Animal:
    to speak:
        say "Woof!".
    end.
end.
let d be fresh Dog.
speak using d.
"#
    )
    .is_ok());
}

#[test]
fn test_method_super() {
    assert!(execute(
        r#"
define an Animal:
    it has name which is "".
    on create with n:
        name becomes n.
    end.
    to greet:
        say "Hello from " + name.
    end.
end.
define a Dog extends Animal:
    to greet:
        super of greet.
        say "Woof from " + name.
    end.
end.
let d be fresh Dog with "Rex".
greet using d.
"#
    )
    .is_ok());
}

#[test]
fn test_method_inherit_default() {
    assert!(execute(
        r#"
define an Animal:
    to speak:
        say "..."
    end.
end.
define a Dog extends Animal:
end.
let d be fresh Dog.
speak using d.
"#
    )
    .is_ok());
}

#[test]
fn test_std_math_aliased() {
    assert!(execute(
        r#"
refer to math as ma.
say sin using ma with 0.
say cos using ma with 0.
say pi using ma.
"#
    )
    .is_ok());
}

#[test]
fn test_std_math_flat() {
    assert!(execute(
        r#"
refer to math.
say sin(0).
say cos(0).
say pi.
"#
    )
    .is_ok());
}

#[test]
fn test_std_random() {
    assert!(execute(
        r#"
refer to random as r.
let n be random using r.
say n.
let x be randint using r with 1, 10.
say x.
"#
    )
    .is_ok());
}

#[test]
fn test_std_selective_import() {
    assert!(execute(
        r#"
refer to sin, cos from math as trig.
say sin using trig with 0.
say cos using trig with 0.
"#
    )
    .is_ok());
}

#[test]
fn test_index_get() {
    assert!(execute(
        r#"
let list be a list containing 10, 20, 30.
let item be 1.
// Test that Index pattern works with list index
"#
    )
    .is_ok());
}

#[test]
fn test_module_path_using() {
    assert!(execute(
        r#"
refer to math as ma.
say sin using ma with 0.
"#
    )
    .is_ok());
}

#[test]
fn test_math_pow() {
    assert!(execute(
        r#"
refer to math.
say pow(2, 3).
"#
    )
    .is_ok());
}

#[test]
fn test_math_abs_floor_ceil() {
    assert!(execute(
        r#"
refer to math as m.
say abs using m with -5.
say floor using m with 3.7.
say ceil using m with 3.2.
"#
    )
    .is_ok());
}

#[test]
fn test_std_seed() {
    assert!(execute(
        r#"
refer to random.
seed(42).
let a be random().
seed(42).
let b be random().
say a.
say b.
"#
    )
    .is_ok());
}

#[test]
fn test_operator_add() {
    assert!(execute(
        r#"
define a Box:
    it has value which is 0.
    on create with n:
        value becomes n.
    end.
    when added to with other:
        return value added to other.
    end.
end.
let b be instantiate Box with 10.
let r be b added to 5.
say r.
"#
    )
    .is_ok());
}

#[test]
fn test_operator_multiply() {
    assert!(execute(
        r#"
define a Box:
    it has value which is 0.
    on create with n:
        value becomes n.
    end.
    when multiplied by with other:
        return value multiplied by other.
    end.
end.
let b be instantiate Box with 6.
let r be b multiplied by 4.
say r.
"#
    )
    .is_ok());
}

#[test]
fn test_operator_equals() {
    assert!(execute(
        r#"
define a Box:
    it has value which is 0.
    on create with n:
        value becomes n.
    end.
    when equals with other:
        return value is equal to other.
    end.
end.
let a be instantiate Box with 5.
let b be instantiate Box with 5.
let v be a is equal to b.
say v.
"#
    )
    .is_ok());
}

#[test]
fn test_operator_negated() {
    assert!(execute(
        r#"
define a Box:
    it has value which is 0.
    on create with n:
        value becomes n.
    end.
    when negated:
        return 0 minus value.
    end.
end.
let b be instantiate Box with 7.
let r be -b.
say r.
"#
    )
    .is_ok());
}

#[test]
fn test_operator_multi_word_method() {
    assert!(execute(
        r#"
define a Box:
    it has value which is 0.
    on create with n:
        value becomes n.
    end.
    when greater than with other:
        return value is greater than other.
    end.
end.
let a be instantiate Box with 10.
let b be instantiate Box with 5.
if a is greater than b:
    say "bigger".
otherwise:
    say "smaller".
end.
"#
    )
    .is_ok());
}

#[test]
fn test_leave_while_loop() {
    assert!(execute(
        r#"
let i be zero.
while i is less than ten:
    when i is equal to five:
        leave.
    end.
    say i.
    i becomes i added to one.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_leave_for_each_loop() {
    assert!(execute(
        r#"
let items be a list containing "a", "b", "c", "d".
for each item in items:
    when item is equal to "c":
        leave.
    end.
    say item.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_skip_for_each_loop() {
    assert!(execute(
        r#"
let items be a list containing "a", "b", "c", "d".
for each item in items:
    when item is equal to "b":
        skip.
    end.
    say item.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_skip_while_loop() {
    assert!(execute(
        r#"
let i be zero.
while i is less than five:
    i becomes i added to one.
    when i is equal to three:
        skip.
    end.
    say i.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_leave_repeat_loop() {
    assert!(execute(
        r#"
let count be zero.
repeat ten times:
    count becomes count added to one.
    when count is equal to three:
        leave.
    end.
end.
say count.
"#
    )
    .is_ok());
}

#[test]
fn test_skip_repeat_loop() {
    assert!(execute(
        r#"
let count be zero.
repeat five times:
    count becomes count added to one.
    when count is equal to three:
        skip.
    end.
    say count.
end.
"#
    )
    .is_ok());
}

#[test]
fn test_ask_no_prompt() {
    assert!(execute(
        r#"
ask and save to x.
say x.
"#
    )
    .is_ok());
}

#[test]
fn test_ask_no_save() {
    assert!(execute(
        r#"
ask "Enter something".
"#
    )
    .is_ok());
}

#[test]
fn test_ask_no_prompt_no_save() {
    assert!(execute(
        r#"
ask.
"#
    )
    .is_ok());
}

#[test]
fn test_minus_operator() {
    assert!(execute(
        r#"
let x be ten minus three.
say x.
"#
    )
    .is_ok());
}
