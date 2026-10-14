// src/ch08_error.rs
// 練習：錯誤處理 (Error Handling: Option & Result)

// --- 1. 基礎：Result 與 Match (已完成) ---
// 這裡保留之前的邏輯作為對照

pub struct Account {
    pub balance: f64,
}

impl Account {
    pub fn new(initial_balance: f64) -> Self {
        Self { balance: initial_balance }
    }

    pub fn withdraw(&mut self, amount: f64) -> Result<f64, String> {
        if amount > self.balance {
            Err(String::from("餘額不足"))
        } else {
            self.balance -= amount;
            Ok(self.balance)
        }
    }
}

// --- 2. 進階：自定義錯誤類型 (Custom Error Enums) ---
// 在實際開發中，我們通常會定義一個 Enum 來代表所有可能的錯誤。

#[derive(Debug)]
pub enum BankError {
    InsufficientFunds(f64), // 傳入不足的金額
    AccountLocked,          // 帳戶被鎖定
    InvalidAmount,          // 無效的金額
}

// 讓我們的錯誤類型可以被印出 (Implement Display-like behavior)
impl std::fmt::Display for BankError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            BankError::InsufficientFunds(amount) => write!(f, "餘額不足！您試圖提款 ${}", amount),
            BankError::AccountLocked => write!(f, "帳戶已被鎖定"),
            BankError::InvalidAmount => write!(f, "金額必須為正數"),
        }
    }
}

pub struct AdvancedAccount {
    pub balance: f64,
    pub is_locked: bool,
}

impl AdvancedAccount {
    pub fn new(balance: f64) -> Self {
        Self { balance, is_locked: false }
    }

    // 使用自定義錯誤的提款方法
    pub fn withdraw_advanced(&mut self, amount: f64) -> Result<f64, BankError> {
        if self.is_locked {
            return Err(BankError::AccountLocked);
        }
        if amount <= 0.0 {
            return Err(BankError::InvalidAmount);
        }
        if amount > self.balance {
            return Err(BankError::InsufficientFunds(amount));
        }

        self.balance -= amount;
        Ok(self.balance)
    }
}

// --- 3. 進階：問號運算子 (The `?` Operator) ---
// `?` 可以讓我們簡化錯誤傳遞的流程。

pub struct User {
    pub name: String,
    pub account: AdvancedAccount,
}

impl User {
    pub fn new(name: &str, balance: f64) -> Self {
        Self {
            name: name.to_string(),
            account: AdvancedAccount::new(balance),
        }
    }

    // 這個函數會「傳遞」錯誤給呼叫者
    pub fn perform_transaction(&mut self, amount: f64) -> Result<f64, BankError> {
        // 使用 `?` 如果發生錯誤，會立即從函數返回 Err
        let new_balance = self
            .account
            .withdraw_advanced(amount)?;
        Ok(new_balance)
    }
}

// --- 測試函數 ---
pub fn run_exercises() {
    println!("Running Chapter 8 exercises...");

    // --- 測試案例 1: 基礎 Result ---
    println!("\n--- 1. Basic Result ---");
    let mut my_account = Account::new(100.0);
    match my_account.withdraw(30.0) {
        Ok(new_balance) => println!("✅ 提款成功！剩餘餘額: ${}", new_balance),
        Err(e) => println!("❌ 提款失敗: {}", e),
    }

    // --- 測試案例 2: 自定義錯誤 ---
    println!("\n--- 2. Custom Error Enum ---");
    let mut adv_acc = AdvancedAccount::new(50.0);
    
    match adv_acc.withdraw_advanced(100.0) {
        Ok(b) => println!("✅ 成功: ${}", b),
        Err(e) => println!("❌ 錯誤: {}", e),
    }

    // --- 測試案例 3: 問號運算子 (The `?` Operator) ---
    println!("\n--- 3. The `?` Operator ---");
    let mut user = User::new("Alice", 200.0);

    // 測試成功路徑
    match user.perform_transaction(50.0) {
        Ok(b) => println!("✅ {} 提款成功，剩餘: ${}", user.name, b),
        Err(e) => println!("❌ 提款失敗: {}", e),
    }

    // 測試錯誤路徑 (餘額不足)
    match user.perform_transaction(300.0) {
        Ok(b) => println!("✅ 提款成功: ${}", b),
        Err(e) => println!("❌ 提款失敗: {}", e),
    }
}
