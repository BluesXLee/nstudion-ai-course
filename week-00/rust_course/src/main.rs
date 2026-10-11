// 1. 定義列舉 (Enum)
// 描述了包裹可能處於的不同狀態
enum PackageStatus {
    Ordered,                         // 單純的標記
    Shipped(String),                 // 帶有一個字串 (物流單號)
    Delivered { recipient: String }, // 帶有一個結構化數據 (收件人)
}

// 2. 定義結構體 (Struct)
struct Package {
    id: u32,
    status: PackageStatus,
}

// 3. 為 Package 實作方法 (Impl)
impl Package {
    // 關聯函數：建立一個初始狀態為 Ordered 的新包裹
    fn new(id: u32) -> Self {
        Self {
            id,
            status: PackageStatus::Ordered,
        }
    }

    // 可變方法：更新包裹的狀態
    fn update_status(&mut self, new_status: PackageStatus) {
        self.status = new_status;
    }
}

// 4. 獨立函數：使用模式匹配 (Pattern Matching) 來處理狀態
// 注意：我們傳入的是 &Package (借用)，所以我們不會擁有包裹的所有權
fn print_status(package: &Package) {
    println!("--- 包裹 ID: {} ---", package.id);

    // 關鍵點：我們對 &package.status 進行模式匹配
    // 使用 & 是因為我們只是「借用」狀態來讀取，而不是要把狀態從結構體中「移走」
    match &package.status {
        PackageStatus::Ordered => {
            println!("狀態: 📦 已訂購，等待發貨。");
        }
        PackageStatus::Shipped(tracking_number) => {
            // 這裡的 tracking_number 會綁定到 Enum 內部的字串
            println!("狀態: 🚚 已發貨。物流單號: {}", tracking_number);
        }
        PackageStatus::Delivered { recipient } => {
            // 這裡的 recipient 會綁定到結構化欄位中
            println!("狀態: ✅ 已送達。收件人: {}", recipient);
        }
    }
}

fn main() {
    // --- 測試流程 ---

    // 1. 建立新包裹 (初始狀態為 Ordered)
    let mut my_package = Package::new(1001);
    print_status(&my_package);

    // 2. 更新狀態為 Shipped (傳入一個新的 String)
    my_package.update_status(PackageStatus::Shipped(String::from("TRACK-ABC-123")));
    print_status(&my_package);

    // 3. 更新狀態為 Delivered
    my_package.update_status(PackageStatus::Delivered {
        recipient: String::from("Blues"),
    });
    print_status(&my_package);
}
