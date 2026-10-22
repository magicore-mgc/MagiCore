mod error;
mod publish_discovery;
pub(super) use publish_discovery::publish_all;
#[cfg(test)]
pub(super) use publish_discovery::published_template_cores;
