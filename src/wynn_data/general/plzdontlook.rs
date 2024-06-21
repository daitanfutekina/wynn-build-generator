//! What have I done

use std::ops;

/// Represents 5 12-bit ints
/// 
/// Note that values are within the range \[-2048, 2047]
#[derive(Clone, Copy, PartialEq, Default)]
pub struct I12x5 {
    pub(crate) data: i64,
}
impl I12x5 {
    pub const MAX: I12x5 = Self{data: i64::MAX & -2252074725150721};
    pub const MIN: I12x5 = Self{data: i64::MIN & -2252074725150721};
    pub const ZERO: I12x5 = Self{data: 0};
    pub const fn fill_data(val: i32) -> I12x5{
        Self{data: 0x10008004002001*val as i64}
    }
    pub fn filled<T: std::convert::Into<i64>>(val: T) -> I12x5{
        Self{data: 0x10008004002001*val.into()}
    }
    /// Gets the `idx`th value
    /// 
    /// This function will automatically convert the type of idx, and can return any type that can be made from an `i64`
    /// # Example
    /// ```
    /// let vals: I12x5 = I12x5::from([5,10,15,20,25]);
    /// let val_at_idx_2: i16 = vals.get(2);
    /// asserteq!(vals.get(2),15);
    /// asserteq!(vals.get(enums::Skill::Int),15);
    /// asserteq!(vals.get(2),enums::Sets::Outlaw);
    /// ```
    pub fn get<I: TryIntoI12x5Idx, T: std::convert::TryFrom<i64>>(&self, idx: I) -> T{
        let i: I12x5Idx = idx.try_into().unwrap();
        match T::try_from(self.data << 13 * i as u8 >> 52){
            Ok(v) => v,
            Err(_) => panic!("Could not parse {} into {}",self.data << 13 * i as u8 >> 52,std::any::type_name::<T>())
        }
    }
    pub fn set<I: TryIntoI12x5Idx, T: std::convert::Into<i64>>(&mut self, idx: I, val: T){
        let i: I12x5Idx = idx.try_into().unwrap();
        self.data = !(0xFFF8004002001000u64 >> 13 * i as u8) as i64 & self.data | val.into() << 13 * (4-i as u8);
    }
    /// Sums all integers
    /// # Example
    /// ```
    /// asserteq!(I12x5::from([5,-23,43,-12,98]).sum(),111);
    pub fn sum(&self) -> i32 {
        ((self.data << 52 >> 52)
            + (self.data << 39 >> 52)
            + (self.data << 26 >> 52)
            + (self.data << 13 >> 52)
            + (self.data >> 52)) as i32
    }
    /// Makes a copy of this `I12x5` but with values clamped to range \[0, 2047]
    /// # Example
    /// ```
    /// asserteq!(I12x5::from([5,-23,43,-12,98]).get_pos(),I12x5::from([5,0,43,0,98]));
    pub fn get_pos(&self) -> Self{
        Self{data: (((!(self.data as u64) & 0x8004002001000800) >> 11) * 0xFFF) as i64 & self.data}
    }
    /// Makes a copy of this `I12x5` but with values clamped to range \[-2048, 0]
    /// # Example
    /// ```
    /// asserteq!(I12x5::from([5,-23,43,-12,98]).get_negs(),I12x5::from([0,-23,0,-12,0]));
    pub fn get_negs(&self) -> Self{
        Self{data: ((((self.data as u64) & 0x8004002001000800) >> 11) * 0xFFF) as i64 & self.data}
    }
    /// Returns whether all values in this `I12x5` are positive 
    /// # Example
    /// ```
    /// assert!(I12x5::from([43,23,5,12,98]).is_pos());
    /// assert!(!I12x5::from([43,-23,5,-12,98]).is_pos());
    pub fn is_pos(&self) -> bool{
        self.data & -9222245999492200448 == 0 //0x8004002001000800
    }
    /// Returns whether all values in this `I12x5` are negative 
    /// # Example
    /// ```
    /// assert!(I12x5::from([-43,-23,-5,-12,-98]).is_neg());
    /// assert!(!I12x5::from([-43,23,-5,12,-98]).is_neg());
    pub fn is_neg(&self) -> bool{
        !self.data & -9222245999492200448 == 0
    }
    /// Makes a copy of of this `I12x5` with all values points greater than `max` to `max`
    /// # Example
    /// ```
    /// asserteq!(I12x5::from([192,124,-28,162,64]).set_max(150),I12x5::from([150,124,-28,150,64]))
    /// ```
    pub fn with_max(&self, max: i64) -> Self{
        let temp = max & 0xFFF;
        // subtract current skill points from "[max; 5]", then filters out the negative values
        let ones = ((temp * 0x10008004002001 - self.data) as u64 & 0x8004002001000800) >> 11;
        // filters out all positive skills, then or's it with the skills that were set to 'max'
        Self{data: ones as i64 * temp | ((ones ^ 0x10008004002001) * 0xFFF) as i64 & self.data}
    }
    /// Makes a copy of of this `I12x5` with all values points less than `min` to `min`
    /// # Example
    /// ```
    /// asserteq!(I12x5::from([-70,97,-28,3,64]).set_min(0),I12x5::from([0,97,0,3,64]))
    /// ```
    pub fn with_min(&self, min: i64) -> Self{
        let temp = min & 0xFFF;
        let ones = ((self.data - temp * 0x10008004002001) as u64 & 0x8004002001000800) >> 11;
        Self{data: ones as i64 * temp | ((ones ^ 0x10008004002001) * 0xFFF) as i64 & self.data}
    }
    pub fn num_negs(&self) -> u32{
        (self.data & -9222245999492200448).count_ones()
    }
    pub fn min(&self, other: I12x5) -> I12x5{
        let ones = ((other.data - self.data) as u64 & 0x8004002001000800) >> 11;
        Self{data: (ones * 0xFFF) as i64 & other.data | ((ones ^ 0x10008004002001) * 0xFFF) as i64 & self.data}
    }
    pub fn max(&self, other: I12x5) -> I12x5{
        let ones = ((self.data - other.data) as u64 & 0x8004002001000800) >> 11;
        Self{data: (ones * 0xFFF) as i64 & other.data | ((ones ^ 0x10008004002001) * 0xFFF) as i64 & self.data}
    }
    pub fn mask_negs(&self) -> i64{
        (((self.data as u64 & 0x8004002001000800) >> 11) * 0xFFF) as i64
    }
    pub fn mask_pos(&self) -> i64{
        (((-self.data as u64 & 0x8004002001000800) >> 11) * 0xFFF) as i64
    }
    pub fn mask_eq(&self, other: I12x5) -> i64{
        let diff = *self-other;
        let neg = -diff;
        (((!(diff.data | neg.data) as u64 & 0x8004002001000800) >> 11) * 0xFFF) as i64
    }
    /// Sets all positive values to 1, all negative values to -1, and keeps 0 as 0
    pub fn normalize(&self) -> Self{
        Self{data: self.mask_negs() | ((-self.data as u64 & 0x8004002001000800) >> 11) as i64}
    }
    pub fn pos_avg(&self) -> i32{
        let pos_sum = self.get_pos().sum();
        if pos_sum==0{0}else{pos_sum/(-*self).num_negs() as i32}
    }
    pub fn avg(&self) -> i32{
        self.sum()/5
    }
    pub fn iter<T: std::convert::TryFrom<i64>>(&self) -> I12x5Iter<T>{
        I12x5Iter::from(*self)
    }
    pub fn iter_unique<T: std::convert::TryFrom<i64>>(&self) -> I12x5UniqueIter<T>{
        I12x5UniqueIter::from(*self)
    }
    pub fn get_max(&self) -> i32{
        (self.data>>52).max(self.data<<13>>52).max(self.data<<26>>52).max(self.data<<39>>52).max(self.data<<52>>52) as i32
    }
}
impl ops::Add for I12x5 {
    type Output = I12x5;
    /// adds two `I12x5` together, and maintains the 0 gap between each 12 bit integer (to prevent overflow)
    fn add(self, rhs: Self) -> Self::Output {
        I12x5 {
            data: (self.data + rhs.data) & -2252074725150721
        }
    }
}
impl <I: TryIntoI12x5Idx, T: std::convert::Into::<i64>> ops::Add<(I, T)> for I12x5 {
    type Output = Self;
    fn add(self, rhs: (I, T)) -> Self::Output {
        let i: I12x5Idx = rhs.0.try_into().unwrap();
        I12x5 {
            data: (self.data + ((rhs.1.into() & 0xFFF) << 13 * (4-i as u8))) & -2252074725150721
        }
    }
}
impl ops::AddAssign for I12x5 {
    fn add_assign(&mut self, rhs: Self) {
        self.data = (self.data + rhs.data) & -2252074725150721
    }
}
impl <I: TryIntoI12x5Idx, T: std::convert::Into::<i64>> ops::AddAssign<(I, T)> for I12x5{
    fn add_assign(&mut self, rhs: (I, T)){
        let i: I12x5Idx = rhs.0.try_into().unwrap();
        self.data = (self.data + ((rhs.1.into() & 0xFFF) << 13 * (4-i as u8))) & -2252074725150721
    }
}
impl ops::Sub for I12x5 {
    type Output = I12x5;
    // not entirely sure why 0x8004002001001 is used for negation instead of 0x10008004002001, but i'm not gonna question it...
    fn sub(self, rhs: Self) -> Self::Output {
        I12x5 {
            data: self.data + (!rhs.data + 0x8004002001001) & -2252074725150721
        }
    }
}
impl <I: TryIntoI12x5Idx, T: std::convert::Into::<i64>> ops::Sub<(I, T)> for I12x5 {
    type Output = Self;
    fn sub(self, rhs: (I, T)) -> Self::Output {
        let i: I12x5Idx = rhs.0.try_into().unwrap();
        I12x5 {
            data: (self.data + ((-rhs.1.into() & 0xFFF) << 13 * (4-i as u8))) & -2252074725150721
        }
    }
}
impl ops::SubAssign for I12x5 {
    fn sub_assign(&mut self, rhs: Self) {
        self.data = self.data + (!rhs.data + 0x8004002001001) & -2252074725150721
    }
}
impl <I: TryIntoI12x5Idx, T: std::convert::Into::<i64>> ops::SubAssign<(I, T)> for I12x5 {
    fn sub_assign(&mut self, rhs: (I, T)){
        let i: I12x5Idx = rhs.0.try_into().unwrap();
        self.data = (self.data + ((-rhs.1.into() & 0xFFF) << 13 * (4-i as u8))) & -2252074725150721
    }
}
impl ops::Neg for I12x5 {
    type Output = I12x5;

    fn neg(self) -> Self::Output {
        Self {
            data: !self.data + 0x8004002001001,
        }
    }
}
impl ops::BitAnd<i64> for I12x5{
    type Output = I12x5;

    fn bitand(self, rhs: i64) -> Self::Output {
        Self{data: self.data & rhs & -2252074725150721}
    }
}
impl std::fmt::Display for I12x5 {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let temp: [i64; 5] = (*self).into();
        write!(
            f,
            "[{}, {}, {}, {}, {}]",
            temp[0_usize], temp[1_usize], temp[2_usize], temp[3_usize], temp[4_usize]
        )
    }
}
impl std::fmt::Debug for I12x5{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let temp: [i32; 5] = (*self).into();
        write!(
            f,
            "I12x5[{}, {}, {}, {}, {}]",
            temp[0_usize], temp[1_usize], temp[2_usize], temp[3_usize], temp[4_usize]
        )
    }
}
impl <T> std::convert::From<[T; 5]> for I12x5 where T: std::convert::Into<i64> + Copy + Default{
    fn from(data: [T; 5]) -> Self {
        Self {
            data: (data[0].into() & 0xFFF) << 52
                | (data[1].into() & 0xFFF) << 39
                | (data[2].into() & 0xFFF) << 26
                | (data[3].into() & 0xFFF) << 13
                | (data[4].into() & 0xFFF),
        }
    }
}
impl <T> std::convert::From<(T,T,T,T,T)> for I12x5 where T: std::convert::Into<i64> + Copy + Default{
    fn from(data: (T,T,T,T,T)) -> Self {
        Self {
            data: (data.0.into() & 0xFFF) << 52
                | (data.1.into() & 0xFFF) << 39
                | (data.2.into() & 0xFFF) << 26
                | (data.3.into() & 0xFFF) << 13
                | (data.4.into() & 0xFFF),
        }
    }
}
impl <T: std::convert::From<i16>> std::convert::Into<[T; 5]> for I12x5{
    fn into(self) -> [T; 5] {
        [
            T::from((self.data >> 52) as i16),
            T::from((self.data << 13 >> 52) as i16),
            T::from((self.data << 26 >> 52) as i16),
            T::from((self.data << 39 >> 52) as i16),
            T::from((self.data << 52 >> 52) as i16),
        ]
    }
}
impl <T: std::convert::From<i16>> std::convert::Into<(T, T, T, T, T)> for I12x5{
    fn into(self) -> (T, T, T, T, T) {
        (
            T::from((self.data >> 52) as i16),
            T::from((self.data << 13 >> 52) as i16),
            T::from((self.data << 26 >> 52) as i16),
            T::from((self.data << 39 >> 52) as i16),
            T::from((self.data << 52 >> 52) as i16),
        )
    }
}
pub struct I12x5Iter<T: std::convert::TryFrom<i64>>{
    idx: usize,
    data: I12x5,
    _phantom: std::marker::PhantomData<T>
}
impl <T: std::convert::TryFrom<i64>>std::convert::From<I12x5> for I12x5Iter<T>{
    fn from(value: I12x5) -> Self {
        Self{idx: 0, data: value, _phantom: std::marker::PhantomData}
    }
}
impl <T: std::convert::TryFrom<i64>>Iterator for I12x5Iter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;
        if self.idx<5{
            res = Some(self.data.get(self.idx));
            self.idx+=1;
        }
        res
    }
}
pub struct I12x5UniqueIter<T: std::convert::TryFrom<i64>>{
    data: u64,
    _phantom: std::marker::PhantomData<T>
}
impl <T: std::convert::TryFrom<i64>>std::convert::From<I12x5> for I12x5UniqueIter<T>{
    fn from(value: I12x5) -> Self {
        Self{data: value.data as u64, _phantom: std::marker::PhantomData}
    }
}
impl <T: std::convert::TryFrom<i64>>Iterator for I12x5UniqueIter<T>{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let mut res = None;
        if self.data>0{
            let tmp = (self.data & 0xFFF) * 0x8004002001000800;
            let shift = (self.data-tmp).trailing_zeros()/13;
            res = Some(match T::try_from((self.data as i64) << 52 >> 52){
                Ok(n) => n, 
                Err(_) => panic!("Could not parse {} into {}",(self.data as i64) << 52 >> 52,std::any::type_name::<T>())
            });
            self.data>>=shift*13;
        }
        res
    }
}
#[derive(Clone, Copy)]
pub enum I12x5Idx{_0, _1, _2, _3, _4}
impl std::convert::TryFrom<u8> for I12x5Idx{
    type Error = String;
    fn try_from(n: u8) -> Result<Self, Self::Error>{
        match n{
            0 => Ok(Self::_0),
            1 => Ok(Self::_1),
            2 => Ok(Self::_2),
            3 => Ok(Self::_3),
            4 => Ok(Self::_4),
            _ => Err(format!("Index {:#?} not in range [0,4]",n))
        }
    }
}
impl std::fmt::Display for I12x5Idx{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{Self::_0 => "0", Self::_1 => "1", Self::_2 => "2", Self::_3 => "3", Self::_4 => "4"})
    }
}
impl std::fmt::Debug for I12x5Idx{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self{Self::_0 => "0", Self::_1 => "1", Self::_2 => "2", Self::_3 => "3", Self::_4 => "4"})
    }
}
pub trait TryIntoI12x5Idx{
    type Error: std::fmt::Debug;
    fn try_into(self) -> Result<I12x5Idx, Self::Error>;
}
impl <T> TryIntoI12x5Idx for T where T: std::convert::From<u8> + std::convert::TryInto<u8> + std::fmt::Debug + Copy{
    type Error = String;
    fn try_into(self) -> Result<I12x5Idx, Self::Error>{
        // let tmp: Result<u8, Self::Error> = self.try_into();
        match self.try_into(){
            Ok(v) => I12x5Idx::try_from(v),
            Err(_) => Err(format!("Index {:#?} not in range [0,4]",self))
        }
    }
}