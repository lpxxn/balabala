
跨包引用：payment/model.proto 引用了 user/model.proto 中的 Address 类型。
命名冲突：user 包和 payment 包都定义了自己的 Address 消息类型，但结构不同。
如果不使用 extern_path，生成的 Rust 代码可能会出现以下问题：

引用路径不正确，导致编译错误
两个不同的 Address 类型可能会相互冲突

用编译通过了，便是用了 extern_path 之后，还是报错了。
