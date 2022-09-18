use std::collections::HashMap;
use std::fs::File;
use std::io::{BufWriter, Write};
use std::path::Path;
use std::{env, io};

const RAW_DATA: &str = include_str!(concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../source/dick/pinyin.txt"
));
type InputData = Vec<(u32, Vec<&'static str>)>;
type PinyinDataIndex = HashMap<&'static str, usize>;

fn create_out_file(name: &str) -> io::Result<impl Write> {
    let path = Path::new(&env::var("OUT_DIR").unwrap()).join(name);
    Ok(BufWriter::new(File::create(&path)?))
}

type ExtractData = Vec<(u32, Vec<&'static str>)>;

fn extract_data() -> ExtractData {
    let mut input_data = RAW_DATA
        .lines()
        .enumerate()
        // 移除注释和空格
        .map(|(i, line)| (i, line.trim()))
        // 移除空行
        .filter(|(_, line)| !line.is_empty())
        .filter(|(_, line)| line.rfind('#') != Some(0))
        .map(|(i, line)| {
            // Split the line by colon
            let colon_pos = match line.find(':') {
                Some(pos) => pos,
                None => unreachable!("no colon found in line {}", i),
            };
            let code_point = line[..colon_pos].trim();
            let pinyin_list: Vec<_> = line[colon_pos + 1..].trim().split(',').collect();

            // 解析码位
            const CODE_POINT_PREFIX: &str = "U+";
            assert!(code_point.starts_with(CODE_POINT_PREFIX));
            let code = &code_point[CODE_POINT_PREFIX.len()..];
            let code = match u32::from_str_radix(code, 16) {
                Ok(code) => code,
                Err(_) => unreachable!("invalid code point {} at line {}", code, i),
            };
            (code, pinyin_list)
        })
        .collect::<Vec<_>>();
    input_data.sort_by_key(|(code, _)| *code);
    input_data
}

fn generate_pinyin_dick(data: &InputData) -> io::Result<PinyinDataIndex> {
    let mut output = create_out_file("py_dicks.rs")?;
    let pinyin_data = HashMap::new();
    writeln!(output, "&[")?;
    let mut process_pinyin = |pinyin: &str| {
        write!(output, "    Dk {{ ")?;

        let hz_limiter = "#";
        let pos = pinyin.find(hz_limiter);
        let mut py = pinyin;
        let mut wd = "";
        let u8 = "";
        if pos > Some(0) {
            py = &pinyin[..pos.unwrap()].trim();
            wd = &pinyin[pos.unwrap() + 1..].trim()
        }

        write!(
            output,
            r#"u8:"{}", py:"{}", al:"{}", wd:"{}""#,
            u8, py, py, wd
        )?;
        writeln!(output, "}},")?;
        Ok(())
    };
    // 插入一个空的拼音数据作为零位
    process_pinyin("")?;
    data.iter()
        .flat_map(|(_, list)| list.iter().map(|s| *s))
        .map(process_pinyin)
        .collect::<io::Result<()>>()?;
    writeln!(output, "]")?;
    Ok(pinyin_data)
}

fn main() -> io::Result<()> {
    let data = extract_data(); // 提取日期
    generate_pinyin_dick(&data)?;

    // 输出这行以保证改动项目的其他文件不会触发编译脚本重新执行
    println!("cargo:rerun-if-changed=build.rs");
    Ok(())
}
