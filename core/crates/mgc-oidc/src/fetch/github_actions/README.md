# GitHub Actions OIDC request

English: Owns request URL construction, removal/replacement of duplicate `audience` query pairs, bearer request and extraction of the nonempty `value` JWT.

Tiếng Việt: Sở hữu tạo URL request, thay các query `audience` cũ, gửi bearer và đọc JWT không rỗng từ trường `value`.

URL policy and HTTP size/timeout/redirect safeguards are delegated to the endpoint owner. Request URL tests are pure and offline.
