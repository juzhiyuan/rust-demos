fn main () {
    // 通常情况下，`{}`会被任意变量内容替换
    // 值内容会转换为字符串
    println!("{} days", 31);

    // 不加后缀的话，31自动成为 I32 类型。
    // 你可以添加后缀来改变 31 的原来类型。

    // 位置参数
    println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

    // 可以使用赋值语句
    println!("{subject}, {verb}, {object}",
        object = "the lazy dog",
        subject = "the quick brown fox",
        verb = "jumps over"
    );

    // 特殊格式实现可以加`:`符号
    println!("{} of {:b} people know binary, the other half don't", 1, 2);

    // 可以指定宽度以对齐文本
    // 5个空格连接1
    println!("{number:>width$}",
        number = 1,
        width = 6
    );

    // 补齐
    // 000001
    println!("{number:>0width$}",
        number = 1,
        width = 6
    );
}
