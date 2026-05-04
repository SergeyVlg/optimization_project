## 1. Правка функции sum_even.

Данная функция при запуске через cargo run падает с паникой:

```plain
thread 'main' (21732) panicked at src\lib.rs:11:29:
unsafe precondition(s) violated: slice::get_unchecked requires that the index is within the slice
```

Данный стек уже намекает на то, что проблемы внутри unsafe блока, где вызывается функция get_unchecked(idx). Запуск через MIRI подтверждает это:

```plain
error: Undefined Behavior: `assume` called with `false`
  --> src\lib.rs:11:22
   |
11 |             let v = *values.get_unchecked(idx);
   |                      ^^^^^^^^^^^^^^^^^^^^^^^^^ Undefined Behavior occurred here
```

Явно утверждается, что индекс idx не удовлетворяет договоренностям не превышать размер values. После исправления через обычный итератор с подсчетом суммы запуск через cargo run выполняется уже без паники, с корректным завершением программы.