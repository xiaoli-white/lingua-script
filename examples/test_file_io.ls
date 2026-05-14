say "=== File IO Test ===".
write "Hello from LinguaScript!" to "/tmp/ls_test_io.txt".
say "write ok".
read "/tmp/ls_test_io.txt" and save to content.
say content.
write "line1\nline2\nline3" to "/tmp/ls_test_io2.txt".
read "/tmp/ls_test_io2.txt" and save to lines.
say lines.
say "=== IO Test Done ===".
stop.
