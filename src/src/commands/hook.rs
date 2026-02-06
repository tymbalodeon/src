const HOOK: &str = include_str!("./hook.nu");

pub fn hook() {
    print!("{HOOK}");
}
