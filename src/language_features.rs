//! Chapter 2. 言語機能

use std::iter::Step;
use std::fmt;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Ipv4 {
    value: u32,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ParseIpv4Error {
    // 不正な値
    InvalidValue(String),
    // フォーマットエラー
    InvalidFormat,
}

impl Ipv4 {
    /// 8bitの4組(a1.a2.a3.a4)に相当する Ipv4 を返す
    pub fn from_dot_decimals(a1: u8, a2: u8, a3: u8, a4: u8) -> Self {
        Ipv4 { value: (a1 as u32)  << 24 | (a2 as u32) << 16 | (a3 as u32) << 8 | a4 as u32 }
    }

    /// ドットアドレス形式の文字列に相当する Ipv4 を返す
    pub fn parse(ipv4: &str) -> Result<Self, ParseIpv4Error> {
        let v: Vec<&str> = ipv4.split('.').collect();
        if v.len() == 4 {
            Ok(Ipv4::from_dot_decimals(
                v[0].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[0].to_owned()))?,
                v[1].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[1].to_owned()))?,
                v[2].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[2].to_owned()))?,
                v[3].parse::<u8>().map_err(|_| ParseIpv4Error::InvalidValue(v[3].to_owned()))?
            ))
        } else {
            Err(ParseIpv4Error::InvalidFormat)
        }
    }
}

impl fmt::Display for Ipv4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:>03}.{:>03}.{:>03}.{:>03}", 
            (self.value >> 24) & 0x00ff,
            (self.value >> 16) & 0x00ff,
            (self.value >> 8) & 0x00ff,
            self.value & 0x00ff
        )
    }
}

impl Step for Ipv4 {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        if end.value <= start.value {
            Some((end.value - start.value) as usize)
        } else {
            None
        }
    }

    fn replace_one(&mut self) -> Self {
        Ipv4 { value: 1 }        
    }

    fn replace_zero(&mut self) -> Self {
        Ipv4 { value: 0 }
    }
    
    fn add_one(&self) -> Self {
        Ipv4 { value: self.value + 1 }
    }

    fn sub_one(&self) -> Self {
        Ipv4 { value: self.value - 1 }
    }

    fn add_usize(&self, n: usize) -> Option<Self> {
        if self.value as usize + n <= std::u32::MAX as usize {
            Some(Ipv4 { value: self.value + n as u32 })
        } else {
            None
        }
    }
}

#[test]
fn test_ipv4_parse() {
    assert_eq!(Ipv4::parse("1.2.3.4").unwrap(), Ipv4::from_dot_decimals(1,2,3,4));
}

#[test]
fn test_ipv4_rangeiter() {
    let a1 = Ipv4::from_dot_decimals(1, 2, 3, 4);
    let a2 = Ipv4::from_dot_decimals(1, 2, 3, 6);

    let mut v = Vec::new();
    for a in a1..=a2 {
        v.push(a);
    }

    assert_eq!(v,
        [
            Ipv4::from_dot_decimals(1, 2, 3, 4), 
            Ipv4::from_dot_decimals(1, 2, 3, 5),
            Ipv4::from_dot_decimals(1, 2, 3, 6)
        ]);
}


pub mod array2d {

    use std::ops;
    use std::iter;

    /// 2次元配列
    #[derive(Debug)]
    pub struct Array2d<T> {
        i_max: usize,
        j_max: usize,
        values: Box<[T]>,
    }

    impl<T> Array2d<T> where T: Clone {

        /// サイズ i x j の2次元配列を作る
        pub fn new(i: usize, j: usize) -> Self where T: Default {
            Array2d {
                i_max: i,
                j_max: j,
                values: vec![Default::default(); i * j].into_boxed_slice()
            }
        }

        /// 座標(i, j)の要素にアクセスする
        pub fn at(&self, i: usize, j: usize) -> Option<&T> {
            if i < self.i_max && j < self.j_max {
                Some(&self.values[i * self.j_max + j])
            } else {
                None
            }
        }

        /// 座標(i, j)の要素にアクセスする
        pub fn at_mut(&mut self, i: usize, j: usize) -> Option<&mut T> {
            if i < self.i_max && j < self.j_max {
                Some(&mut self.values[i * self.j_max + j])
            } else {
                None
            }
        }

        /// データ列を取得する
        pub fn data(&self) -> &[T] {
            &self.values
        }

        /// データ列を取得する
        pub fn data_mut(&mut self) -> &mut [T] {
            &mut(self.values)
        }

        /// 全ての要素に一律に値を設定する
        pub fn fill(&mut self, value: T) {
            for i in 0..self.values.len() {
                self.values[i] = value.clone();
            }
        }
    }

    impl<T> ops::Index<(usize,usize)> for Array2d<T> {
        type Output = T;

        fn index(&self, index: (usize,usize)) -> &Self::Output {
            &self.values[index.0 * self.j_max + index.1]
        }
    }

    impl<T> ops::IndexMut<(usize,usize)> for Array2d<T> {

        fn index_mut(&mut self, index: (usize,usize)) -> &mut Self::Output {
            &mut(self.values[index.0 * self.j_max + index.1])
        }
    }

    /// Array2d<T> イテレーター
    pub struct IntoIter<T> {
        curr: usize,
        array2d: Array2d<T>,
    }

    impl<T> iter::Iterator for IntoIter<T> 
        where T: Clone {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            let curr = self.curr;
            if curr < self.array2d.values.len() {
                self.curr += 1;
                Some(self.array2d.values[curr].clone())
            } else {
                None
            }
        }
    }

    /// &Array2d イテレーター
    pub struct Iter<'a, T> {
        curr: usize,
        array2d: &'a Array2d<T>,
    }

    impl<'a, T> iter::Iterator for Iter<'a, T> 
    {
        type Item = &'a T;

        fn next(&mut self) -> Option<Self::Item> {
            let curr = self.curr;
            if curr < self.array2d.values.len() {
                self.curr += 1;
                Some(&self.array2d.values[curr])
            } else {
                None
            }
        }
    }

    impl<T> iter::IntoIterator for Array2d<T> 
        where T: Clone {
        type Item = T;
        type IntoIter = IntoIter<T>;

        fn into_iter(self) -> Self::IntoIter {
            IntoIter {
                curr: 0,
                array2d: self
            }
        }
    }

    impl<'a, T> iter::IntoIterator for &'a Array2d<T> where T: Clone {
        type Item = &'a T;
        type IntoIter = Iter<'a, T>;
    
        fn into_iter(self) -> Self::IntoIter {
            Iter {
                curr: 0,
                array2d: self
            }
        }
    }

    #[test]
    fn test_array2d() {
        let mut ma = Array2d::new(2, 3);

        for i in 0..2 {
            for j in 0..3 {
                ma[(i,j)] = ((i+1) * 10 + (j+1)) as u8;
            }
        }
        assert_eq!([11,12,13,21,22,23], ma.data());   
        assert_eq!(Some(&12), ma.at(0, 1));
        assert_eq!(None, ma.at(2, 2));

        {
            let a = &ma;
            assert_eq!([11,12,13,21,22,23], a.data());
            assert_eq!(Some(&12), a.at(0, 1));
            assert_eq!(None, a.at(2, 2));
                
            //a[(1,1)] = 10;
            //a.fill(255);
        }
    
        ma.fill(255);
        assert_eq!([255,255,255,255,255,255], ma.data());
    }

    #[test]
    fn test_array2d_intoiter() {
        let mut ma = Array2d::new(2,3);
        for i in 0..2 {
            for j in 0..3 {
                ma[(i,j)] = ((i+1) * 10 + (j+1)) as u8;
            }
        }

        let mut t = Vec::new();
        for v in ma {
            t.push(v);
        }
        assert_eq!(vec![11,12,13,21,22,23], t);
    }

    #[test]
    fn test_array2d_iter() {
        let mut ma = Array2d::new(2,3);
        for i in 0..2 {
            for j in 0..3 {
                ma[(i,j)] = ((i+1) * 10 + (j+1)) as u8;
            }
        }

        let mut t = Vec::new();
        for v in &ma {
            t.push(v);
        }
        assert_eq!(vec![&11,&12,&13,&21,&22,&23], t);
    }
}

/// 任意個数の要素から最小値を求めるマクロ
#[macro_export]
macro_rules! my_min {
    ( $x1:expr, $x2:expr ) => { if $x1 < $x2 { $x1 } else { $x2 } };
    ( $x1:expr, $x2:expr, $(  $x3:expr ),+ ) => { my_min!($x1, my_min!($x2, $( $x3 ),* )) };
}

#[test]
fn test_my_min() {
    assert_eq!(my_min!(5, 2, 1, 3), 1);
    assert_eq!(my_min!(1.0f64, -2.1f64, 0f64), -2.1f64);
}

/// 任意個数の要素をコンテナに追加するマクロ
#[macro_export]
macro_rules! push_back {
    ( $vec:expr, $( $x:expr ),+ ) => { $( $vec.push($x); )+ };
}

#[test]
fn test_push_back() {
    let mut v = vec![1,2,3];
    push_back!(v, 4, 5);
    assert_eq!(v, vec![1,2,3,4,5]);
}



















