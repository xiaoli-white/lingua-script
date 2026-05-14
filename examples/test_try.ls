say "=== test beware ===".
beware:
    say "in try block".
    raise "something went wrong".
    say "after raise".
in case of error:
    say "caught: " + "error".
regardless:
    say "finally block".
end.
say "after try".
say "=== test attempt ===".
attempt to:
    say "attempting...".
    raise "fail".
if it fails:
    say "handled failure".
regardless:
    say "cleanup".
end.
say "done".
