#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Map};

#[contract]
pub struct VoucherContract;

#[contractimpl]
impl VoucherContract {
    // 1. Khởi tạo: Cửa hàng phát hành 1 lần duy nhất X số phiếu và cài ngày hết hạn
    pub fn initialize(env: Env, store_admin: Address, amount: i128, deadline: u64) {
        if env.storage().instance().has(&symbol_short!("deadline")) {
            panic!("Dot phieu nay da duoc khoi tao truoc do roi!");
        }

        let mut balances: Map<Address, i128> = Map::new(&env);
        // Trao toàn bộ phiếu ban đầu cho cửa hàng để họ phân phối cho khách
        balances.set(store_admin.clone(), amount);

        env.storage().instance().set(&symbol_short!("balances"), &balances);
        env.storage().instance().set(&symbol_short!("total"), &amount);
        env.storage().instance().set(&symbol_short!("deadline"), &deadline);
    }

    // 2. Hàm ĐỔI QUÀ
    // Khách hàng gọi hàm này tại cửa hàng để nhận quà. Phiếu sẽ bị HỦY VĨNH VIỄN.
    pub fn redeem(env: Env, customer: Address, amount: i128) {
        // Khách hàng phải ký duyệt bằng ví của họ
        customer.require_auth();

        // Lấy ngày hết hạn từ Blockchain ra kiểm tra
        let deadline: u64 = env.storage().instance().get(&symbol_short!("deadline")).unwrap();
        // Lỗi nếu quá thời gian giới hạn
        if env.ledger().timestamp() > deadline {
            panic!("Phieu doi qua da het han su dung!");
        }

        let mut balances: Map<Address, i128> = env
            .storage()
            .instance()
            .get(&symbol_short!("balances"))
            .expect("Chua khoi tao phieu");

        let customer_balance = balances.get(customer.clone()).unwrap_or(0);
        if customer_balance < amount {
            panic!("Ban khong du phieu de doi qua!");
        }

        // TÍNH TOÁN GIẢM DẦN:
        // Bước 1: Trừ phiếu của khách hàng
        balances.set(customer.clone(), customer_balance - amount);
        
        // Bước 2: Giảm tổng số lượng phiếu đang tồn tại trên thế giới (Đốt phiếu)
        let total_supply: i128 = env.storage().instance().get(&symbol_short!("total")).unwrap();
        let new_total = total_supply - amount;

        // Lưu lại trạng thái mới lên chuỗi
        env.storage().instance().set(&symbol_short!("balances"), &balances);
        env.storage().instance().set(&symbol_short!("total"), &new_total);

        // (Tại đây, cửa hàng sẽ kiểm tra lịch sử Blockchain, thấy hàm này chạy thành công 
        // thì sẽ đưa sản phẩm/quà tặng vật lý cho khách hàng).
    }

    // Hàm xem tổng số phiếu còn sót lại trên thị trường (Xem nó giảm dần thế nào)
    pub fn total_vouchers_left(env: Env) -> i128 {
        env.storage().instance().get(&symbol_short!("total")).unwrap_or(0)
    }
}