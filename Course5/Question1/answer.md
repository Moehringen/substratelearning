# 列出3个常用的宏
- decl_storage 定义存储单元
- decl_module 包含可调用函数
- decl_event 事件
- decl_error 错误信息
- construct_runtime 添加模块到 Runtime

# 列出3个常用的存储数据结构
- 单值,存储某种单一类型的值，如布尔，数值，枚举，结构体等
   ● 数值：u8,i8,u32,i32,u64,i64,u128,i128
   ● 大整数：U128,U256,U512
   ● 布尔：bool
   ● 集合：Vec<T>, BTreeMap, BTreeSet
   ● 定点小数：Percent,Permill,Perbill
   ● 定长哈希：H128,H256,H512
   ● 其它复杂类型：Option<T>,tuple,enum,struct
   ● 内置自定义类型：Moment,AccountId
   以bool为例: 
   bool 类型定义：
    decl_storage! {
        trait Store for Module<T: Trait> as DataTypeModule {
        // init to false, store boolean value
    MyBool get(fn my_bool): bool;
        }
    }

- 简单映射: map 类型，用来保存键值对，单值类型都可以用作key或者value
    decl_storage! {
        trait Store for Module<T: Trait> as DataTypeModule {
        MyMap get(fn my_map): map hasher(twox_64_concat) u8 =>
        Vec<u8>;
        }
    }

- 双键映射: double_map 类型，使用两个key来索引value，用于快速删除key1对应的任意记
录，也可以遍历key1对应的所有记录
    decl_storage! {
    trait Store for Module<T: Trait> as DataTypeModule {
    MyDoubleMap get(fn my_double_map): double_map hasher
    (blake2_128_concat) T::AccountId, hasher(blake2_128_concat) u32 => Vec<u8>;
        }
    }