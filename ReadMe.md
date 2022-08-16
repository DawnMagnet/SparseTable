# 使用Rust实现的ST表(Sparse Table)
[English Version](./ReadMe_en.md)  
ST表是一类高效的数组结构，在利用O(n*log2(n))的时间建完索引后，就可以使用O(1)的时间查询区间最大值/最小值/最大公约数等。是一个非常高效的数据结构。

用法：
- 在`Cargo.toml`里添加依赖
```toml
sparse_table = "*"
```

例子：  
```rust
use sparse_table::SparseTable;
use std::cmp::max;

fn main() {
    let spt = SparseTable::init(&vec![5, 3, 8, 10, 1], max);
    println!("{:?}", spt.dp);
    dbg!(spt.query(0, 4));
}

```

输出结果：
```text
[[5, 5, 10, 0], [3, 8, 10, 0], [8, 10, 0, 0], [10, 10, 0, 0], [1, 0, 0, 0]]
[src\main.rs:7] spt.query(0, 4) = 10
```

如果给入的函数是max，则query返回的就是区间最大值，如果给入的是gcd，返回的就是区间gcd