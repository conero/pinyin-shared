# 使用 python 调用 dll 等动态库
# 2020.12.04
import os
import sys
from ctypes import *


def get_current_path(vpath=None):
    """
    获取当前目录
    :param vpath: str
    :return:
    """
    paths = sys.path
    current_file = os.path.basename(__file__)
    for path in paths:
        try:
            if current_file in os.listdir(path):
                if not vpath:
                    path = f'{path}{vpath}'
                path = path.replace("\\", "/")
                return path
        except (FileExistsError, FileNotFoundError) as e:
            print(e)
    return None


def test_load_lib():
    """
    依赖库加载
    :return:
    """
    dlibFile = get_current_path() + "/../../target/release/pinyin.dll"
    dlib = cdll.LoadLibrary(dlibFile)

    print(dlib)
    # 函数调用
    dlib.test()

    # source_date
    # sd = dlib.source_date()
    # print(sd.values)
    # 读取字符串
    dlib.source_date.restype = c_uint64  # 修改lib.bar返回类型
    rst = dlib.source_date()
    print(rst)
    rst = string_at(rst)
    print(rst.decode('utf-8'))

    # source_len
    print(dlib.source_len())

    # source_text
    # dlib.source_text.restype = c_uint64  # 修改lib.bar返回类型
    # rst = dlib.source_text()
    # print(rst)
    # rst = string_at(rst)
    # print(rst.decode('utf-8'))
    
    #
    dlib.pinyin_words.restype = c_uint64  # 修改lib.bar返回类型
    args = b'12.990.12' # 无法传入中文
    # args = create_string_buffer('古丞秋', 100)
    args = bytes("古丞秋","utf8")
    #args = '12.990.12' # 无效
    rst = dlib.pinyin_words(args)
    print(rst)
    rst = string_at(rst)
    print(rst.decode('utf-8'))

    


if __name__ == "__main__":
    test_load_lib()
