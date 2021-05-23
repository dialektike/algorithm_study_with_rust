# algorithm study with rust

## make sample data

```r
no_1 <- sample(1:10000, 10000)
no_2 <- sample(1:10000, 10000)
no_3 <- sample(1:10000, 10000)
no_4 <- sample(1:10000, 10000)
no_5 <- sample(1:10000, 10000)
test <- data.frame(no_1, no_2, no_3, no_4, no_5)
```

```r
> head(test)
  no_1 no_2 no_3 no_4 no_5
1 7157 2274 3309 8273 7035
2 7276 3945  934  802 1792
3 6820 7268  664 3137 7044
4 7755 5660 2080 3340  521
5 9159 5670 6060  848 7618
6 2378 2499 1806 6309 2472
```

```r
install.packages("jsonlite")
library(jsonlite)
write_json(test, "test.json")
```

## cd change_txt_to_list

```bash
cargo new change_txt_to_list
cd change_txt_to_list 
```

```bash
cargo run
```

## bubble_sort

```bash
cargo new bubble_sort
```
