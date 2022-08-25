# 数列をシャッフルするコード

cargo run 1 で任意の最大値を持つ初項１，等差１の整数列を生成できる．

cargo run 2 で生成した数列の順番をシャッフルできる．

## 例えば

: 最大値１００００を指定して生成すると => [example_numbers.txt](example_numbers.txt)

: シャッフルを実行すると => [example_ShuffledNums.txt](example_ShuffledNums.txt)

のようになる．



# 実装

シャッフルする際のアルゴリズムとして Fisher-Yates のアルゴリズムを採用した．

実際の実装は以下のよう
```
let length = number.len();
let mut rng = rand::thread_rng();
for i in 0..length {
    let j = rng.gen_range(0..length);
    let tmp = number[i];
    number[i] = number[j];
    number[j] = tmp;
}
```

参考 : [Quiita記事 Fisher-Yates shufflアルゴリズムを用いて自作で配列をシャッフルさせる](https://qiita.com/shoheiyokoyama/items/4a49674f1e65671ea52b)

# 目的

今後，ソートアルゴリズムのビジュアライズができるコードを実装したいと考えており，
その布石として今回，数列のシャッフルするためのアルゴリズムの勉強を目的として
実装してみた．(2022/08/25 時点)

# todo

シャッフルアルゴリズムの改良