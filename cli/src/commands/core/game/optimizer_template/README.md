# Bevy optimizer transaction / Transaction optimizer Bevy

Owns the stable optimizer setup entry and coordinates lock acquisition, staging, manifest mutation, publish and cleanup. It delegates project validation, manifest snapshot/rollback, destination validation and dependency editing to sibling owners without changing their order.

Owner này sở hữu entry setup optimizer ổn định, điều phối lock, staging, sửa manifest, publish và cleanup. Nó delegate validation project, snapshot/rollback manifest, kiểm tra đích và sửa dependency cho sibling owner, giữ nguyên thứ tự hiện có.
