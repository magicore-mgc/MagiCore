//! GitHub Actions output implementation.
//! Triển khai output GitHub Actions.
//!
//! Honesty rules (spec D-A1..A6): never emit a placeholder `echo` step, take
//! build/test commands only from `mgc.toml [ci]`, refuse to write a file when
//! the configuration is missing, and only emit the trusted-publishing job when
//! the release/trusted-publishing inputs are present.
//! Quy tắc trung thực: không sinh bước `echo`, lệnh build/test chỉ lấy từ
//! `mgc.toml [ci]`, từ chối ghi file khi thiếu cấu hình, và chỉ sinh job
//! trusted publishing khi có đủ đầu vào.

use std::path::Path;

use anyhow::Result;

use crate::scaffold::processors::write_file;
use mgc_config::project::ProjectConfig;

const CHECKOUT_PIN: &str = "actions/checkout@11d5960a326750d5838078e36cf38b85af677262";
const UPLOAD_PIN: &str = "actions/upload-artifact@b4b15b8c7c6ac21ea08fcf65892d2ee8f75cf882";
const DOWNLOAD_PIN: &str = "actions/download-artifact@fa0a91b85d4f404e444e00e005971372dc801d16";

pub(in crate::scaffold::processors::cicd::implementation) fn write(
    target: &Path,
    config: &ProjectConfig,
) -> Result<()> {
    let ci = config.ci.as_ref();
    let Some(ci) = ci.filter(|ci| ci.verifiable()) else {
        // Missing build/test: stop with a message naming the field, never
        // write a workflow that would only prove an echo ran.
        // Thiếu build/test: dừng và nêu đúng trường cần khai, không ghi
        // workflow chỉ chứng minh một lệnh echo đã chạy.
        let missing = match config.ci.as_ref() {
            None => "ci.build and ci.test",
            Some(ci) if ci.build.trim().is_empty() && ci.test.trim().is_empty() => {
                "ci.build and ci.test"
            }
            Some(ci) if ci.build.trim().is_empty() => "ci.build",
            Some(_) => "ci.test",
        };
        return Err(crate::error::ci_config_required(
            missing,
            &config.ecosystem,
            config.frameworks.first().map(String::as_str).unwrap_or(""),
        ));
    };

    let mut yaml = String::from(
        "name: CI\n\npermissions:\n  contents: read\n\non:\n  push:\n  pull_request:\n\njobs:\n",
    );
    yaml.push_str("  verify:\n    runs-on: ubuntu-latest\n    permissions:\n      contents: read\n    steps:\n");
    yaml.push_str(&format!(
        "      # SHA pin does not prove the app builds or passes tests; the steps below do.\n      # Ghim SHA không chứng minh app build/test đúng; các bước dưới mới chứng minh.\n      - uses: {CHECKOUT_PIN}\n"
    ));
    yaml.push_str("      - run: mgc install --frozen\n");
    yaml.push_str(&format!(
        "      - name: Declared production build\n        run: {}\n",
        ci.build
    ));
    yaml.push_str(&format!(
        "      - name: Declared test\n        run: {}\n",
        ci.test
    ));

    if ci.verifies_production() {
        yaml.push_str(&format!(
            "      - name: Build production artifact\n        run: |\n          docker build -t app:${{{{ github.sha }}}} -f {dockerfile} .\n          docker save app:${{{{ github.sha }}}} -o \"$RUNNER_TEMP/artifact.tar\"\n        env:\n          DOCKER_BUILDKIT: '1'\n",
            dockerfile = ci.production
        ));
        yaml.push_str(&format!(
            "      - uses: {UPLOAD_PIN}\n        with:\n          name: app-image\n          path: ${{{{ runner.temp }}}}/artifact.tar\n          if-no-files-found: error\n"
        ));
        yaml.push_str("\n  stage-artifact:\n    needs: [verify]\n    runs-on: ubuntu-latest\n    permissions:\n      contents: read\n    outputs:\n      digest: ${{ steps.digest.outputs.value }}\n    steps:\n");
        yaml.push_str(&format!(
            "      # Staging uses its own credentials, never production secrets. It cannot\n      # promise the same data, load or hardware as production.\n      # Staging dùng credential riêng, không dùng secret production. Không bảo\n      # đảm giống production về dữ liệu, tải hay phần cứng.\n      - uses: {DOWNLOAD_PIN}\n        with:\n          name: app-image\n"
        ));
        yaml.push_str("      - run: docker load -i artifact.tar\n");
        yaml.push_str("      - name: Start artifact\n        run: |\n          docker run -d --name smoke -p 4315:4315 app:${{ github.sha }}\n          for _ in $(seq 1 30); do docker inspect -f '{{{{.State.Running}}}}' smoke | grep -q true && break; sleep 2; done\n          docker inspect -f '{{{{.State.Running}}}}' smoke | grep -q true\n");
        if ci.smoke.trim().is_empty() {
            yaml.push_str(
                "      # No smoke command declared: this workflow must not claim the artifact works.\n      # Chưa khai smoke: workflow không được tuyên bố artifact chạy được.\n      - run: |\n          echo \"::error::ci.smoke is required to verify the production artifact\"\n          exit 1\n",
            );
        } else {
            yaml.push_str(&format!(
                "      - name: Smoke check on the built artifact\n        run: {}\n",
                ci.smoke
            ));
        }
        yaml.push_str(
            "      - id: digest\n        run: echo \"value=$(docker inspect --format '{{.Id}}' app:${{ github.sha }})\" >> \"$GITHUB_OUTPUT\"\n",
        );
    }

    if !ci.production.trim().is_empty() {
        yaml.push_str("\n  publish:\n    if: startsWith(github.ref, 'refs/tags/') && vars.MAGICORE_TRUSTED_PUBLISH == 'true'\n    needs: [stage-artifact]\n    permissions:\n      contents: read\n      id-token: write\n    runs-on: ubuntu-latest\n    steps:\n");
        yaml.push_str(&format!(
            "      - uses: {CHECKOUT_PIN}\n        with:\n          persist-credentials: false\n"
        ));
        yaml.push_str(&format!(
            "      - uses: {DOWNLOAD_PIN}\n        with:\n          name: app-image\n"
        ));
        yaml.push_str("      - run: docker load -i artifact.tar\n");
        yaml.push_str(
            "      # Promote exactly the digest that passed staging; never rebuild here.\n      # Promote đúng digest đã qua staging; không build lại ở đây.\n      - name: Promote verified artifact\n        env:\n          MGC_REGISTRY_URL: ${{ vars.MAGICORE_REGISTRY_URL }}\n          MGC_OIDC_AUDIENCE: ${{ vars.MAGICORE_OIDC_AUDIENCE }}\n          MGC_PUBLISH_CORE: ${{ vars.MAGICORE_PUBLISH_CORE }}\n          MGC_PUBLISH_PACKAGE: ${{ vars.MAGICORE_PUBLISH_PACKAGE }}\n          VERIFIED_DIGEST: ${{ needs.stage-artifact.outputs.digest }}\n        run: mgc publish --trusted --protocol oci --core \"$MGC_PUBLISH_CORE\" --package \"$MGC_PUBLISH_PACKAGE\" --version \"${{ github.ref_name }}\" --artifact-digest \"$VERIFIED_DIGEST\" --registry \"$MGC_REGISTRY_URL\" --trusted-audience \"$MGC_OIDC_AUDIENCE\"\n",
        );
    }

    write_file(
        &target.join(".github").join("workflows").join("ci.yml"),
        &yaml,
    )
}
