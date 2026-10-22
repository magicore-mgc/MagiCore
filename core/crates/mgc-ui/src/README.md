# mgc-ui source owners

The crate root is wiring and public re-exports. Each presentation responsibility lives in its same-name owner.
Crate root chỉ wiring/re-export công khai. Mỗi trách nhiệm presentation nằm trong owner cùng tên.

| Owner | English responsibility | Trách nhiệm |
|---|---|---|
| banner | Startup banner output. | In banner khởi động. |
| command_style | Command-name styling for display. | Định dạng tên command để hiển thị. |
| help | Help text assembly and section rendering. | Ghép help text và render từng section. |
| prompt | confirm/input/password/select/multi_select terminal interactions. | Tương tác terminal confirm/input/password/select/multi_select. |
| install_summary | Package/cache installation summary. | Summary cài package và cache. |
| next_steps | Project-created panel and suggested commands. | Bảng hoàn tất project và command gợi ý. |
| progress | Progress bars, spinners, multi-progress, and ProgressManager. | Progress bar, spinner, multi-progress và ProgressManager. |
| quiet | Shared process-local quiet state. | State quiet dùng chung trong tiến trình. |
| section | Step heading output. | Output heading từng bước. |
| status | Success/error/warning/info/hint messages. | Thông báo thành công/lỗi/cảnh báo/info/hint. |

table/ is the public Table API owner; table/table/ owns model state and methods while table/render/ owns line construction and stdout rendering.
table/ là owner API Table public; table/table/ sở hữu model state và method, còn table/render/ sở hữu dựng dòng và render stdout.

docs/ contains this source owner's progress report and checklist.
docs/ chứa báo cáo tiến độ và checklist của source owner này.
