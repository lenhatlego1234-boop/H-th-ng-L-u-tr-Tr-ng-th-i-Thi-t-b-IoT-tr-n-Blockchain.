#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol};

#[contract]
pub struct IotDeviceTracker;

#[contractimpl]
impl IotDeviceTracker {
    
    /// Cập nhật trạng thái của thiết bị IoT (Bật/Tắt)
    /// - `device_id`: Mã định danh của thiết bị (ví dụ: Symbol::new(&env, "SENSOR_1"))
    /// - `is_active`: Trạng thái thiết bị (true = Bật, false = Tắt)
    pub fn set_status(env: Env, device_id: Symbol, is_active: bool) {
        // Lưu trữ trạng thái vào Instance Storage của Contract
        env.storage().instance().set(&device_id, &is_active);
    }

    /// Truy vấn trạng thái hiện tại của thiết bị
    /// Nếu thiết bị chưa từng được cập nhật, hàm sẽ trả về giá trị mặc định là `false`
    pub fn get_status(env: Env, device_id: Symbol) -> bool {
        env.storage()
            .instance()
            .get(&device_id)
            .unwrap_or(false)
    }
}
