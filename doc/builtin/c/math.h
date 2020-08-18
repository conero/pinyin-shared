double acos(double x);
// 返回以弧度表示的 x 的反余弦。

double asin(double x);
// 返回以弧度表示的 x 的反正弦。

double atan(double x);
// 返回以弧度表示的 x 的反正切。

double atan2(double y, double x);
// 返回以弧度表示的 y/x 的反正切。y 和 x 的值的符号决定了正确的象限。

double cos(double x);
// 返回弧度角 x 的余弦。

double cosh(double x);
// 返回 x 的双曲余弦。

double sin(double x);
// 返回弧度角 x 的正弦。

double sinh(double x);
// 返回 x 的双曲正弦。

double tanh(double x);
// 返回 x 的双曲正切。

double exp(double x);
// 返回 e 的 x 次幂的值。

double frexp(double x, int *exponent);
// 把浮点数 x 分解成尾数和指数。返回值是尾数，并将指数存入 exponent 中。所得的值是 x = mantissa * 2 ^ exponent。

double ldexp(double x, int exponent);
// 返回 x 乘以 2 的 exponent 次幂。

double log(double x);
// 返回 x 的自然对数（基数为 e 的对数）。

double log10(double x);
// 返回 x 的常用对数（基数为 10 的对数）。

double modf(double x, double *integer);
// 返回值为小数部分（小数点后的部分），并设置 integer 为整数部分。

double pow(double x, double y);
// 返回 x 的 y 次幂。

double sqrt(double x);
// 返回 x 的平方根。

double ceil(double x);
// 返回大于或等于 x 的最小的整数值。

double fabs(double x);
// 返回 x 的绝对值。

double floor(double x);
// 返回小于或等于 x 的最大的整数值。

double fmod(double x, double y);
// 返回 x 除以 y 的余数。