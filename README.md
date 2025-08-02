# Язык программирования Next
## HelloWorld на Next
```
module example
import std.io

[entry]
fn main {
  const userName = std.io.readln("Enter your name:")

  std.io.println("Hello, %!" % userName)
}

```
## Features
### Трансляция в C
Next транслируется в C код. Возможен экспорт имен(ключевое слово export) для использования в C коде.
#### example.next
```
[ImportCHeader("stdio.h")]
import cstdio;

public fn add<TArgFirst, TArgSecond>(a: TArgFirst, TArgSecond) => a + b;

export add<int, int>

export addf = add<float, float> // мы можем указать, чтобы экспортировать объект под другим именем
export add<int, float>
export add<float, int>
export fn print(a: float) => cstdio.prinf("%f", a)
```
#### usage.c
```c
#include "example.generated.h"

int main(int argc, const char** argv){
  int a = addii(10, 10);
  float b = addif(a, 10.3f);
  float c = addf(b, 0.1f);
  print(c); // если у функции нет перегрузок, то к имени не добавляются имена типов
  return 0;
}
```
