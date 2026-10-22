# Trust-key owner / Owner trust-key

Own the user-only `MGC_POLICY_TRUST_KEYS` input and parse 16-digit BLAKE3 hex key IDs. Invalid values are discarded, preserving the closed behavior when no key remains.

Chịu trách nhiệm env trust do user cấu hình và parse ID key hex 16 ký tự. Test gần owner bao phủ loại entry sai và chuẩn hóa chữ hoa/thường.
