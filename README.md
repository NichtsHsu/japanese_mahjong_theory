# Japanese Mahjong Theory

日麻牌理分析器

用来练手Rust的项目。

## 重要更新日志

##### Ver 0.91 2020/2/25

* 第一个能够算得上是程序的版本。
* 完成基本的牌理功能，支持任意3*k+2的输入。

## 编译

没什么好说的，会Rust都知道`cargo build --release`就完事了。

## 使用

打开程序后输入牌谱即可，按照约定俗称的缩写：

* `m`->万子
* `p`->饼子
* `s`->索子
* `z`->字牌

作为扩展，允许使用`[]`表示副露的牌，这些牌的数量会从听牌数中减掉。

##### 输入样例

* 比较标准的形式：`1m2m3m5m9m9m2p2p4s5s1z[5z5z5z]`
* 省略多余的标记：`123599m22p45s1z[555z]`
* 空格将会被无视：`123599m 22p 45s 1z [555z]`
* 3*k+2不包含副露，可以加入杠：`123599m 22p 45s 1z [5555z]`
* 输入顺序可以随便：`99m2p [5555z] 1z12m 2p45s35m`
