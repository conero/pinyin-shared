// since: 2022-09-18
// author: Joshua Conero
// build: 2022-09-18 11:31:41

// 有python 脚本生产
// 拼音字典
use crate::Dk;

// 字典数据
pub(crate) static PY_DICKS: &[Dk] = include!(concat!(env!("OUT_DIR"), "/py_dicks.rs"));
