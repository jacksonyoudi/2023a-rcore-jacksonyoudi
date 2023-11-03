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