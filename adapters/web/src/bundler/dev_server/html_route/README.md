# HTML and HMR client routes / Route HTML và HMR client

Wires two focused route owners: `index_document/` builds or reads the HTML page, escapes fallback titles and injects the HMR script; `hmr_client/` serves the embedded client with its content type. Each owner keeps its own nearest tests.

Nối hai owner route riêng: `index_document/` đọc hoặc sinh HTML, escape title dự phòng và chèn script HMR; `hmr_client/` phục vụ client nhúng cùng content type. Mỗi owner có test gần nhất.

The project-root title remains escaped as text, endpoint bodies and route registrations remain unchanged. Tests are registered at their respective owners.

Title lấy từ thư mục project vẫn được escape theo ngữ cảnh text; body endpoint và route registration giữ nguyên. Test đăng ký tại từng owner.
