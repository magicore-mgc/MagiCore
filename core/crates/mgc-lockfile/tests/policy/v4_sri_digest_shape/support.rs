// V4 install readiness requires a decodable full-length digest.
// Gate không được coi chuỗi alphabet/hex/short digest là SRI dùng được.
use base64::Engine;
use mgc_lockfile::policy::check_install_sri;
