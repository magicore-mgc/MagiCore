# Web container CI operations

This owner contains separate scaffold/build and run/probe operations for the generated React Web and Express containers. The existing seeded Node fixture remains under `common/` because it is an older repository fixture, distinct from the generated project lanes.

Owner này chứa các thao tác riêng scaffold/build và run/probe cho container Web React và Express được sinh. Fixture Node có sẵn vẫn nằm trong `common/` vì đó là fixture repository cũ, khác với lane dự án sinh mới.

Preserve MGC-only install, generated `.mgc.core`/`mgc.lock` checks, runner ownership correction, Dockerfile/Compose assertions, published ports, response markers and container cleanup. The React lane's generated Kubernetes files are exercised by the separate `kubernetes/` owner.
