pub trait MutualExclusivity<T> {
    fn check_for_exclusivity(&self) -> anyhow::Result<T>;
}
