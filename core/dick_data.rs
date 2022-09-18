
// since: 2022-09-18
// author: Joshua Conero
// build: 2022-09-18 11:31:41

// 有python 脚本生产
// 拼音字典
use crate::Dk;

// 字典数据
pub(crate) static PY_DICKS: &[Dk] = include!(concat!(env!("OUT_DIR"), "/py_dicks.rs"));


// 结构体
pub(crate) static PY_DICKS2: &[Dk] = &[
Dk {u8: "3007", py:"líng,yuán,xīng", al: "ling,yuan,xing", wd: "〇"},
Dk {u8: "3A6D", py:"bó", al: "bo", wd: "㩭"},
Dk {u8: "3A6E", py:"qián", al: "qian", wd: "㩮"},
Dk {u8: "3A6F", py:"pó", al: "po", wd: "㩯"},
Dk {u8: "3A70", py:"jiǎo", al: "jiao", wd: "㩰"},
Dk {u8: "2CB2D", py:"lún", al: "lun", wd: "𬬭"},
];
