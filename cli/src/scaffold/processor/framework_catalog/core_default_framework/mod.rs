// Shared core-default wiring — expose the mapping to the facade owner.
// Wiring default core dùng chung — cung cấp mapping cho owner facade.
mod core_default_framework;
#[cfg(test)]
mod test;

pub(super) use core_default_framework::default_framework;
