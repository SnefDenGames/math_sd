/// A module for numbers, characters and sizes
/// 
/// `nss` stands for numbers, symbols and sizes.
/// It includes some basics, for mathematics and mathematical use
pub mod ncs {
    /// contains the Greek alphabet
    /// 
    /// An enumeration which includes all the characters of the Greek alphabet and offers some methods to deal with them.
    # [derive(Clone,Copy,PartialEq,PartialOrd,Debug)]
    pub enum GreekAlphabet {
        Alpha,  Beta,   Gamma,  Delta,  Epsilon,
        Zeta,   Eta,    Theta,  Iota,   Kappa,
        Lambda, My,     Ny,     Xi,     Omicron,
        Pi,     Rho,    Sigma,  Tau,    Ypsilon,
        Phi,    Chi,    Psi,    Omega,  Empty
    }
    impl GreekAlphabet {
        /// returns capital letter of `GreekAlphabet`
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Example:
        /// ```rust
        /// let sigma = GreekAlphabet::Pi;
        /// assert_eq!("Σ", sigma.uppercase());
        /// ```
        pub fn uppercase(&self) -> char {
            match self {
                GreekAlphabet::Alpha    =>  'Α',
                GreekAlphabet::Beta     =>  'Β',
                GreekAlphabet::Gamma    =>  'Γ',
                GreekAlphabet::Delta    =>  'Δ',
                GreekAlphabet::Epsilon  =>  'Ε',
                GreekAlphabet::Zeta     =>  'Ζ',
                GreekAlphabet::Eta      =>  'Η',
                GreekAlphabet::Theta    =>  'Θ',
                GreekAlphabet::Iota     =>  'Ι',
                GreekAlphabet::Kappa    =>  'Κ',
                GreekAlphabet::Lambda   =>  'Λ',
                GreekAlphabet::My       =>  'Μ',
                GreekAlphabet::Ny       =>  'Ν',
                GreekAlphabet::Xi       =>  'Ξ',
                GreekAlphabet::Omicron  =>  'Ο',
                GreekAlphabet::Pi       =>  'Π',
                GreekAlphabet::Rho      =>  'Ρ',
                GreekAlphabet::Sigma    =>  'Σ',
                GreekAlphabet::Tau      =>  'Τ',
                GreekAlphabet::Ypsilon  =>  'Υ',
                GreekAlphabet::Phi      =>  'Φ',
                GreekAlphabet::Chi      =>  'Χ',
                GreekAlphabet::Psi      =>  'Ψ',
                GreekAlphabet::Omega    =>  'Ω',
                _                       =>  panic!("charackter not exist in GreekAlphabet")
            }
        }
        /// returns lowercase letter of `GreekAlphabet`
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Example:
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// assert_eq!("π", pi.lowercase());
        /// ```
        pub fn lowercase(&self) -> char {
            match self {
                GreekAlphabet::Alpha    =>  'α',
                GreekAlphabet::Beta     =>  'β',
                GreekAlphabet::Gamma    =>  'γ',
                GreekAlphabet::Delta    =>  'δ',
                GreekAlphabet::Epsilon  =>  'ε',
                GreekAlphabet::Zeta     =>  'ζ',
                GreekAlphabet::Eta      =>  'η',
                GreekAlphabet::Theta    =>  'θ',
                GreekAlphabet::Iota     =>  'ι',
                GreekAlphabet::Kappa    =>  'κ',
                GreekAlphabet::Lambda   =>  'λ',
                GreekAlphabet::My       =>  'μ',
                GreekAlphabet::Ny       =>  'ν',
                GreekAlphabet::Xi       =>  'ξ',
                GreekAlphabet::Omicron  =>  'ο',
                GreekAlphabet::Pi       =>  'π',
                GreekAlphabet::Rho      =>  'ρ',
                GreekAlphabet::Sigma    =>  'σ',
                GreekAlphabet::Tau      =>  'τ',
                GreekAlphabet::Ypsilon  =>  'υ',
                GreekAlphabet::Phi      =>  'φ',
                GreekAlphabet::Chi      =>  'χ',
                GreekAlphabet::Psi      =>  'ψ',
                GreekAlphabet::Omega    =>  'ω',
                _                       =>  panic!("charackter not exist in GreekAlphabet")
            }
        }

        /// transcribes the old Greek characters into the Latin writing system
        /// [source for transcription here...](https://www.code-knacker.de/griechischesalphabet.htm) (21.10.2021/Germany)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let psi = GreekAlphabet::Psi;
        /// assert_eq!("ps", psi.transcription());
        /// ```
        pub fn transcription(&self) -> &str {
            // transcription from:  https://www.code-knacker.de/griechischesalphabet.htm (21.10.2021/Germany)
            match self {
                GreekAlphabet::Alpha    =>  "a",
                GreekAlphabet::Beta     =>  "b",
                GreekAlphabet::Gamma    =>  "g",
                GreekAlphabet::Delta    =>  "d",
                GreekAlphabet::Epsilon  =>  "ĕ",
                GreekAlphabet::Zeta     =>  "z",
                GreekAlphabet::Eta      =>  "ē",
                GreekAlphabet::Theta    =>  "th",
                GreekAlphabet::Iota     =>  "i, j",
                GreekAlphabet::Kappa    =>  "k",
                GreekAlphabet::Lambda   =>  "l",
                GreekAlphabet::My       =>  "m",
                GreekAlphabet::Ny       =>  "n",
                GreekAlphabet::Xi       =>  "x",
                GreekAlphabet::Omicron  =>  "ŏ",
                GreekAlphabet::Pi       =>  "p",
                GreekAlphabet::Rho      =>  "r",
                GreekAlphabet::Sigma    =>  "s",
                GreekAlphabet::Tau      =>  "t",
                GreekAlphabet::Ypsilon  =>  "y, ü",
                GreekAlphabet::Phi      =>  "ph, f",
                GreekAlphabet::Chi      =>  "ch",
                GreekAlphabet::Psi      =>  "ps",
                GreekAlphabet::Omega    =>  "ō",
                _                       =>  panic!("charackter not exist in GreekAlphabet")
            }
        }
        /// transcribes the old Greek characters into the Latin writing system
        /// [source for transcription here...](https://www.code-knacker.de/griechischesalphabet.htm) (21.10.2021/Germany)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let psi = GreekAlphabet::Psi;
        /// assert_eq!(String::from("ps"), psi.transcription_string());
        /// ```
        pub fn transcription_string(&self) -> String {
            self.transcription().to_string()
        }

        /// returns the characters as an [`tuple`](https://doc.rust-lang.org/std/primitive.tuple.html) of [`char`](https://doc.rust-lang.org/std/char/index.html)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : (char,char) = pi.letters();
        /// ```
        pub fn letters(&self) -> (char,char) {
            (self.uppercase(), self.lowercase())
        }
        /// returns the characters as an [`tuple`](https://doc.rust-lang.org/std/primitive.tuple.html) of [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : (String,String) = pi.letters_string();
        /// ```
        pub fn letters_string(&self) -> (String,String) {
            (String::from(self.uppercase()), String::from(self.lowercase()))
        }
        /// returns the characters as an [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) of [`char`](https://doc.rust-lang.org/std/char/index.html)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : Vec<char> = pi.letters_vec();
        /// ```
        pub fn letters_vec(&self) -> Vec<char> {
            Vec::from([self.uppercase(),self.lowercase()])
        }
        /// returns the characters as an [`Vec`](https://doc.rust-lang.org/std/vec/struct.Vec.html) of [`String`](https://doc.rust-lang.org/std/string/struct.String.html)
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : Vec<String> = pi.letters_vec_string();
        /// ```
        pub fn letters_vec_string(&self) -> Vec<String> {
            Vec::from([String::from(self.uppercase()), String::from(self.lowercase())])
        }
        /// returns the characters as an [`array`](https://doc.rust-lang.org/std/array/index.html) of [`char`](https://doc.rust-lang.org/std/char/index.html)s
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : [char;2] = pi.letters_array();
        /// ```
        pub fn letters_array(&self) -> [char;2] {
            [self.uppercase(), self.lowercase()]
        }
        /// returns the characters as an [`array`](https://doc.rust-lang.org/std/array/index.html) of [`String`](https://doc.rust-lang.org/std/string/struct.String.html)s
        /// 
        /// # Panics
        /// Panics when called on `GreekAlphabet::Empty`.
        /// 
        /// # Examples
        /// ```rust
        /// let pi = GreekAlphabet::Pi;
        /// let letters : [String;2] = pi.letters_array_string();
        /// ```
        pub fn letters_array_string(&self) -> [String;2] {
            [String::from(self.uppercase()), String::from(self.lowercase())]
        }
    }
    
    impl std::fmt::Display for GreekAlphabet {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::result::Result<(), std::fmt::Error> {
            let name = match self {
                GreekAlphabet::Alpha    =>  "Alpha",
                GreekAlphabet::Beta     =>  "Beta",
                GreekAlphabet::Gamma    =>  "Gamma",
                GreekAlphabet::Delta    =>  "Delta",
                GreekAlphabet::Epsilon  =>  "Epsilon",
                GreekAlphabet::Zeta     =>  "Zeta",
                GreekAlphabet::Eta      =>  "Eta",
                GreekAlphabet::Theta    =>  "Theta",
                GreekAlphabet::Iota     =>  "Iota",
                GreekAlphabet::Kappa    =>  "Kappa",
                GreekAlphabet::Lambda   =>  "Lambda",
                GreekAlphabet::My       =>  "My",
                GreekAlphabet::Ny       =>  "Ny",
                GreekAlphabet::Xi       =>  "Xi",
                GreekAlphabet::Omicron  =>  "Omicron",
                GreekAlphabet::Pi       =>  "Pi",
                GreekAlphabet::Rho      =>  "Rho",
                GreekAlphabet::Sigma    =>  "Sigma",
                GreekAlphabet::Tau      =>  "Tau",
                GreekAlphabet::Ypsilon  =>  "Ypsilon",
                GreekAlphabet::Phi      =>  "Phi",
                GreekAlphabet::Chi      =>  "Chi",
                GreekAlphabet::Psi      =>  "Psi",
                GreekAlphabet::Omega    =>  "Omega",
                _                       =>  panic!("charackter not exist in GreekAlphabet")
            };
            write!(f,"{}({},{})",name,self.uppercase(),self.lowercase())
        }
    }

    impl std::default::Default for GreekAlphabet {
        fn default() -> Self {
            GreekAlphabet::Empty
        }
    }

    impl std::convert::From<char> for GreekAlphabet {
        fn from(c: char) -> Self {
            match c {
                'Α' =>  GreekAlphabet::Alpha,
                'α' =>  GreekAlphabet::Alpha,
                'Β' =>  GreekAlphabet::Beta,
                'β' =>  GreekAlphabet::Beta,
                'Γ' =>  GreekAlphabet::Gamma,
                'γ' =>  GreekAlphabet::Gamma,
                'Δ' =>  GreekAlphabet::Delta,
                'δ' =>  GreekAlphabet::Delta,
                'Ε' =>  GreekAlphabet::Epsilon,
                'ε' =>  GreekAlphabet::Epsilon,
                'Ζ' =>  GreekAlphabet::Zeta,
                'ζ' =>  GreekAlphabet::Zeta,
                'Η' =>  GreekAlphabet::Eta,
                'η' =>  GreekAlphabet::Eta,
                'Θ' =>  GreekAlphabet::Theta,
                'θ' =>  GreekAlphabet::Theta,
                'Ι' =>  GreekAlphabet::Iota,
                'ι' =>  GreekAlphabet::Iota,
                'Κ' =>  GreekAlphabet::Kappa,
                'κ' =>  GreekAlphabet::Kappa,
                'Λ' =>  GreekAlphabet::Lambda,
                'λ' =>  GreekAlphabet::Lambda,
                'Μ' =>  GreekAlphabet::My,
                'μ' =>  GreekAlphabet::My,
                'Ν' =>  GreekAlphabet::Ny,
                'ν' =>  GreekAlphabet::Ny,
                'Ξ' =>  GreekAlphabet::Xi,
                'ξ' =>  GreekAlphabet::Xi,
                'Ο' =>  GreekAlphabet::Omicron,
                'ο' =>  GreekAlphabet::Omicron,
                'Π' =>  GreekAlphabet::Pi,
                'π' =>  GreekAlphabet::Pi,
                'Ρ' =>  GreekAlphabet::Rho,
                'ρ' =>  GreekAlphabet::Rho,
                'Σ' =>  GreekAlphabet::Sigma,
                'σ' =>  GreekAlphabet::Sigma,
                'Τ' =>  GreekAlphabet::Tau,
                'τ' =>  GreekAlphabet::Tau,
                'Υ' =>  GreekAlphabet::Ypsilon,
                'υ' =>  GreekAlphabet::Ypsilon,
                'Φ' =>  GreekAlphabet::Phi,
                'φ' =>  GreekAlphabet::Phi,
                'Χ' =>  GreekAlphabet::Chi,
                'χ' =>  GreekAlphabet::Chi,
                'Ψ' =>  GreekAlphabet::Psi,
                'ψ' =>  GreekAlphabet::Psi,
                'Ω' =>  GreekAlphabet::Omega,
                'ω' =>  GreekAlphabet::Omega,
                _   =>  GreekAlphabet::Empty
            }
        }
    }

    impl std::convert::From<u8> for GreekAlphabet {
        fn from(p: u8) -> Self {
            match p {
                1   =>  GreekAlphabet::Alpha,
                2   =>  GreekAlphabet::Beta,
                3   =>  GreekAlphabet::Gamma,
                4   =>  GreekAlphabet::Delta,
                5   =>  GreekAlphabet::Epsilon,
                6   =>  GreekAlphabet::Zeta,
                7   =>  GreekAlphabet::Eta,
                8   =>  GreekAlphabet::Theta,
                9   =>  GreekAlphabet::Iota,
                10  =>  GreekAlphabet::Kappa,
                11  =>  GreekAlphabet::Lambda,
                12  =>  GreekAlphabet::My,
                13  =>  GreekAlphabet::Ny,
                14  =>  GreekAlphabet::Xi,
                15  =>  GreekAlphabet::Omicron,
                16  =>  GreekAlphabet::Pi,
                17  =>  GreekAlphabet::Rho,
                18  =>  GreekAlphabet::Sigma,
                19  =>  GreekAlphabet::Tau,
                20  =>  GreekAlphabet::Ypsilon,
                21  =>  GreekAlphabet::Phi,
                22  =>  GreekAlphabet::Chi,
                23  =>  GreekAlphabet::Psi,
                24  =>  GreekAlphabet::Omega,
                _   =>  GreekAlphabet::Empty
            }
        }
    }
    impl std::convert::From<u16> for GreekAlphabet {
        fn from(p: u16) -> Self {
            if p < u8::MAX as u16 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<u32> for GreekAlphabet {
        fn from(p: u32) -> Self {
            if p < u8::MAX as u32 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<u64> for GreekAlphabet {
        fn from(p: u64) -> Self {
            if p < u8::MAX as u64 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<u128> for GreekAlphabet {
        fn from(p: u128) -> Self {
            if p < u8::MAX as u128 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<usize> for GreekAlphabet {
        fn from(p: usize) -> Self {
            if p < u8::MAX as usize {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }

    impl std::convert::From<i8> for GreekAlphabet {
        fn from(p: i8) -> Self {
            if p > 0 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<i16> for GreekAlphabet {
        fn from(p: i16) -> Self {
            if p > 0 && p < u8::MAX as i16 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<i32> for GreekAlphabet {
        fn from(p: i32) -> Self {
            if p > 0 && p < u8::MAX as i32 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<i64> for GreekAlphabet {
        fn from(p: i64) -> Self {
            if p > 0 && p < u8::MAX as i64 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<i128> for GreekAlphabet {
        fn from(p: i128) -> Self {
            if p > 0 && p < u8::MAX as i128 {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<isize> for GreekAlphabet {
        fn from(p: isize) -> Self {
            if p > 0 && p < u8::MAX as isize {
                return GreekAlphabet::from(p as u8)
            }
            GreekAlphabet::Empty
        }
    }

    impl std::convert::From<&str> for GreekAlphabet {
        fn from(t: &str) -> Self {
            if t.len() == 1 {
                for c in t.chars() {
                    return GreekAlphabet::from(c)
                }
            }
            GreekAlphabet::Empty
        }
    }
    impl std::convert::From<String> for GreekAlphabet {
        fn from(t: String) -> Self {
            if t.len() == 1 {
                for c in t.chars() {
                    return GreekAlphabet::from(c)
                }
            }
            GreekAlphabet::Empty
        }
    }

    impl std::convert::Into<char> for GreekAlphabet {
        fn into(self) -> char {
            self.lowercase()
        }
    }
    impl std::convert::Into<String> for GreekAlphabet {
        fn into(self) -> String {
            self.lowercase().to_string()
        }
    }

    impl std::convert::Into<u8> for GreekAlphabet {
        fn into(self) -> u8 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as u8 + 1
            }
        }
    }
    impl std::convert::Into<u16> for GreekAlphabet {
        fn into(self) -> u16 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as u16 + 1
            }
        }
    }
    impl std::convert::Into<u32> for GreekAlphabet {
        fn into(self) -> u32 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as u32 + 1
            }
        }
    }
    impl std::convert::Into<u64> for GreekAlphabet {
        fn into(self) -> u64 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as u64 + 1
            }
        }
    }
    impl std::convert::Into<u128> for GreekAlphabet {
        fn into(self) -> u128 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as u128 + 1
            }
        }
    }
    impl std::convert::Into<usize> for GreekAlphabet {
        fn into(self) -> usize {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as usize + 1
            }
        }
    }
    
    impl std::convert::Into<i8> for GreekAlphabet {
        fn into(self) -> i8 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as i8 + 1
            }
        }
    }
    impl std::convert::Into<i16> for GreekAlphabet {
        fn into(self) -> i16 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as i16 + 1
            }
        }
    }
    impl std::convert::Into<i32> for GreekAlphabet {
        fn into(self) -> i32 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as i32 + 1
            }
        }
    }
    impl std::convert::Into<i64> for GreekAlphabet {
        fn into(self) -> i64 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as i64 + 1
            }
        }
    }
    impl std::convert::Into<i128> for GreekAlphabet {
        fn into(self) -> i128 {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as i128 + 1
            }
        }
    }
    impl std::convert::Into<isize> for GreekAlphabet {
        fn into(self) -> isize {
            match self {
                GreekAlphabet::Empty    =>  0,
                _                       =>  self as isize + 1
            }
        }
    }

    impl std::ops::Add for GreekAlphabet {
        type Output = Self;
        fn add(self, other: Self) -> Self {
            let mut p : u8 = (self as u8 + 1) + (other as u8 + 1);
            if p > GreekAlphabet::Omega.into() {
                p = p - (GreekAlphabet::Omega as u8 + 1);
            }
            GreekAlphabet::from(p)
        }
    }
    impl std::ops::Sub for GreekAlphabet {
        type Output = Self;
        fn sub(self, other: Self) -> Self {
            let mut p : i8 = (self as i8 + 1) - (other as i8 + 1);
            if p < 1 {
                p = p + (GreekAlphabet::Omega as i8 + 1);
            }
            GreekAlphabet::from(p)
        }
    }

    impl std::ops::AddAssign for GreekAlphabet {
        fn add_assign(&mut self, other: Self) {
            *self = *self + other;
        }
    }
    impl std::ops::SubAssign for GreekAlphabet {
        fn sub_assign(&mut self, other: Self) {
            *self = *self - other;
        }
    }
}