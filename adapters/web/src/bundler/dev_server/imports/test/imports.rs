#![allow(clippy::unwrap_used)]
//! JavaScript string-literal escaping security tests.
//! Test an toàn escape JavaScript string literal.

use super::js_string_literal;

/// The JS string-literal embed (P1-3): CSS/error payloads ride as JSON
/// string literals — no template literal, so no interpolation, no
/// terminator, no escape-grammar race. What goes in as DATA cannot come
/// out as CODE.
/// (Nhúng string-literal JS (P1-3): payload CSS/lỗi đi dạng JSON string
/// literal — không template literal, nên không nội suy, không kết thúc,
/// không đua ngữ pháp escape. Vào là DỮ LIỆU thì không ra là CODE.)
#[test]
fn js_string_literal_is_jailproof() {
    // `${}` interpolation payload: embedded as plain text inside the
    // JSON string — a JS parser reads the two literal chars `$` `{`.
    // (Payload nội suy `${}`: nhúng dạng text thuần trong chuỗi JSON —
    // parser JS đọc 2 ký tự literal `$` `{`.)
    let css = "body { content: '${alert(1)}'; }";
    let lit = js_string_literal(css);
    assert!(
        lit.starts_with('"') && lit.ends_with('"'),
        "the embed must be a double-quoted string literal: {lit:?}"
    );
    // The payload sits INSIDE a double-quoted literal — `${}` has no
    // interpolation meaning there (only template literals interpolate),
    // and the literal must round-trip as pure data.
    // (Payload nằm TRONG literal trích dẫn kép — `${}` không có nghĩa
    // nội suy ở đó (chỉ template literal mới nội suy), và literal phải
    // round-trip như dữ liệu thuần.)
    let round: String = serde_json::from_str(&lit).expect("literal must be valid JSON");
    assert_eq!(round, css, "the interpolation payload must survive as DATA");
    // And there is NO template literal in the embed at all: no backtick
    // wraps the payload (the old injection vector).
    // (Và KHÔNG có template literal nào trong embed: không backtick bọc
    // payload (vector injection cũ).)
    assert!(
        !lit.starts_with('`') && !lit.ends_with('`'),
        "the embed must not use template-literal quoting: {lit:?}"
    );
    // String TERMINATORS are escaped: no raw `"` and no unbalanced `\`.
    // (Kết thúc chuỗi bị escape: không `"` thô và không `\` lơ lửng.)
    let nasty = "a \" b \\ c ` d ${e} f\n";
    let lit2 = js_string_literal(nasty);
    // serde_json escapes the inner quotes; the literal still round-trips.
    // (serde_json escape trích dẫn trong; literal vẫn round-trip.)
    let round: String = serde_json::from_str(&lit2).expect("literal must be valid JSON");
    assert_eq!(round, nasty, "the embed must round-trip the payload");
    // Raw backticks are irrelevant in a string literal (they are just
    // letters) — pinned so a future "simplification" back to template
    // literals is caught.
    // (Backtick thô vô nghĩa trong string literal (chỉ là ký tự) — ghim
    // để bắt ai "đơn giản hóa" quay lại template literal.)
    assert!(lit2.contains("` d "));
}
