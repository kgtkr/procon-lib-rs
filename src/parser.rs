macro_rules! input {
  ($s:expr=>$($t:tt)*) => {
    let mut lines=$s.split("\n");
    $(
        line_parse!(lines,$t);
    )*
  };
}

macro_rules! line_parse {
  ($lines:expr,($($name:ident:$t:tt)*)) => {
    let mut line=$lines.next().unwrap().split_whitespace();
    $(value_def!(line,$name,$t);)*
  };

  //複数行
  ($lines:expr,{$n:expr;$name:ident:$t:tt}) => {
    values_def!($lines,$n,$name,$t);
  };
}

macro_rules! value_def {
  ($line:expr,$name:ident,$t:tt) => {
    let $name=value!($line,$t);
  };
}

macro_rules! values_def {
  ($lines:expr,$n:expr,$name:ident,$t:tt) => {
    let $name={
      let mut vec=Vec::new();
      for i in 0..$n{
        let mut next=$lines.next().unwrap().split_whitespace();
        vec.push(value!(next,$t));
      }
      vec
    };
  };
}

macro_rules! value {
  //配列
  ($line:expr,[$t:tt]) => {
    $line.map(|x|{
      let mut iter=::std::iter::once(x);
      value!(iter,$t)
    }).collect::<Vec<_>>()
  };
  //タプル
  ($line:expr,($($t:tt),*)) => {
    ($(value!($line,$t),)*)
  };
  //文字列
  ($line:expr,#) => {
    $line.next().unwrap()
  };
  //単一値
  ($line:expr,$type:ty) => {
    $line.next().unwrap().parse::<$type>().unwrap()
  };
}

#[test]
fn test1() {
  {
    input!(
    "3
5 2
2 3 4 5 6
10
20
30
1 2
8 1
2 3
4 1
1283 23 43 32
"=>
    (n:usize) //単一値
    (k:i64 p:i64) //複数値
    (list1:[i64]) //配列
    {n;list2:i64} //N回繰り返し
    (tup:(i64,i64)) //タプル
    {n;list3:(i64,i64)}
    (i:i64 list4:[i64])
  );
    assert_eq!(n, 3);
    assert_eq!(k, 5);
    assert_eq!(p, 2);
    assert_eq!(list1, vec![2, 3, 4, 5, 6]);
    assert_eq!(list2, vec![10, 20, 30]);
    assert_eq!(tup, (1, 2));
    assert_eq!(list3, vec![(8, 1), (2, 3), (4, 1)]);
    assert_eq!(i, 1283);
    assert_eq!(list4, vec![23, 43, 32]);
  }

  {
    input!(
    "3
5 2
2 3 4 5 6
10
20
30
1 2
8 1
2 3
4 1
1283 23 43 32
"=>
    (n:usize) //単一値
    (k:# p:#) //複数値
    (list1:[#]) //配列
    {n;list2:#} //N回繰り返し
    (tup:(#,#)) //タプル
    {n;list3:(#,#)}
    (i:# list4:[#])
  );
    assert_eq!(n, 3);
    assert_eq!(k, "5");
    assert_eq!(p, "2");
    assert_eq!(list1, vec!["2", "3", "4", "5", "6"]);
    assert_eq!(list2, vec!["10", "20", "30"]);
    assert_eq!(tup, ("1", "2"));
    assert_eq!(list3, vec![("8", "1"), ("2", "3"), ("4", "1")]);
    assert_eq!(i, "1283");
    assert_eq!(list4, vec!["23", "43", "32"]);
  }
}
