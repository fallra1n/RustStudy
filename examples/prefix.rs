use prefix;

fn main() {
    println!(
        "{}",
        prefix::longest_common_prefix(vec![" ( ͡❛ ͜ʖ ͡❛) ✊", " ( ͡❛ ͜ʖ ͡❛)✊"])
    );
    println!(
        "{}",
        prefix::longest_common_prefix(vec!["hi!✊ Ìha", "hi!✊ Ýha"])
    );
}
