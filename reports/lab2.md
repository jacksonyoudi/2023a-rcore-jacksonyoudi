先占位

Ref
- 相关链接集合 rust-based-os-comp2023 /relatedinfo.md
- BlogOS:  Writing an OS in Rust  https://os.phil-opp.com/zh-CN/
- 拿到自己的github仓库
    - rCore Tutorial ClassRoom邀请链接
    - https://github.com/LearningOS/2023a-rcore-hky1999
- 课程幻灯片
  全面&复杂的指导书
  rCore-Tutorial-Book 第三版
  面向本次课程的指导书（简化版）
  rCore-Tutorial-Guide 2023 秋季学期
  课程安排
- 四节课+作业讲解
- 阶段任务：五个大实验
# setup build&run environment first
$ git clone https://github.com/LearningOS/${Your Repository Name}.git
$ cd ${Path Your Repository Name}
$ rm -rf ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Checker-2023A.git ci-user
$ git clone https://github.com/LearningOS/rCore-Tutorial-Test-2023A.git ci-user/user
$ git checkout ch$ID# check&grade OS in ch$ID with more tests
$ cd ci-user && make test CHAPTER=$ID
- Notice: $ID is from [3,4,5,6,8]
1. 从一个Bare Metal APP开始 (published)10月23日 20:00
2. rCore内存管理和页表 (published)10月25日 20:00
3. rCore进程管理 (published)10月27日 20:00
4. rCore文件系统 easy-fs  (published)本周一 20:00
5. rCore作业讲解和答疑 (published)本周三 20:00
