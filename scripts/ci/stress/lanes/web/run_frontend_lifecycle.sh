set -euo pipefail
if [[ "${RUNNER_OS}" == "Windows" ]]; then
  export MGC_TEST_DIRECT_PRODUCT=1
fi
set +e
failed=0
for target in \
  web_fw_fe_angular \
  web_fw_fe_astro \
  web_fw_fe_nextjs \
  web_fw_fe_react_vite \
  web_fw_fe_remix \
  web_fw_fe_sveltekit \
  web_fw_fe_vue_vite
do
  echo "::group::${target}"
  cargo test --locked -p mgc --test "${target}" -- --test-threads=1
  result=$?
  echo "::endgroup::"
  if [[ "${result}" -ne 0 ]]; then
    echo "FAILED: ${target} (exit ${result})"
    failed=1
  fi
done
exit "${failed}"
