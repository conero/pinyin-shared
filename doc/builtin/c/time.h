struct tm {
   int tm_sec;         /* 秒，范围从 0 到 59        */
   int tm_min;         /* 分，范围从 0 到 59        */
   int tm_hour;        /* 小时，范围从 0 到 23        */
   int tm_mday;        /* 一月中的第几天，范围从 1 到 31    */
   int tm_mon;         /* 月，范围从 0 到 11        */
   int tm_year;        /* 自 1900 年起的年数        */
   int tm_wday;        /* 一周中的第几天，范围从 0 到 6    */
   int tm_yday;        /* 一年中的第几天，范围从 0 到 365    */
   int tm_isdst;       /* 夏令时                */
};


char *asctime(const struct tm *timeptr);
// 返回一个指向字符串的指针，它代表了结构 timeptr 的日期和时间。

clock_t clock(void);
// 返回程序执行起（一般为程序的开头），处理器时钟所使用的时间。

char *ctime(const time_t *timer);
// 返回一个表示当地时间的字符串，当地时间是基于参数 timer。

double difftime(time_t time1, time_t time2);
// 返回 time1 和 time2 之间相差的秒数 (time1-time2)。

struct tm *gmtime(const time_t *timer);
// timer 的值被分解为 tm 结构，并用协调世界时（UTC）也被称为格林尼治标准时间（GMT）表示。

struct tm *localtime(const time_t *timer);
// timer 的值被分解为 tm 结构，并用本地时区表示。

time_t mktime(struct tm *timeptr);
// 把 timeptr 所指向的结构转换为一个依据本地时区的 time_t 值。

size_t strftime(char *str, size_t maxsize, const char *format, const struct tm *timeptr);
// 根据 format 中定义的格式化规则，格式化结构 timeptr 表示的时间，并把它存储在 str 中。

time_t time(time_t *timer);
// 计算当前日历时间，并把它编码成 time_t 格式。