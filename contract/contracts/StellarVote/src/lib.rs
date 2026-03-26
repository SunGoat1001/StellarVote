#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol, Vec};

#[contract]
pub struct VotingContract;

const VOTE_COUNT: Symbol = symbol_short!("VOTES");
const VOTED_ALREADY: Symbol = symbol_short!("HAS_VOTED");

#[contractimpl]
impl VotingContract {
    // 1. Khởi tạo hoặc cập nhật lựa chọn bỏ phiếu
    pub fn vote(env: Env, voter: Address, choice: u32) {
    voter.require_auth();

    let key = (VOTED_ALREADY, voter.clone());
    if env.storage().persistent().has(&key) {
        panic!("Ban da bau chon roi!");
    }

    let mut votes: Vec<u32> = env
        .storage()
        .persistent()
        .get(&VOTE_COUNT)
        .unwrap_or(Vec::new(&env));

    // SỬA TẠI ĐÂY: Nếu lựa chọn (index) lớn hơn chiều dài hiện tại của mảng
    // Chúng ta cần thêm các phần tử 0 vào cho đến khi đạt tới index đó.
    while votes.len() <= choice {
        votes.push_back(0);
    }

    // Bây giờ index 'choice' chắc chắn đã tồn tại, ta có thể lấy và tăng giá trị
    let current_count = votes.get(choice).unwrap_or(0);
    votes.set(choice, current_count + 1);

    env.storage().persistent().set(&VOTE_COUNT, &votes);
    env.storage().persistent().set(&key, &true);
}

    // 2. Lấy kết quả hiện tại
    pub fn get_results(env: Env) -> Vec<u32> {
        env.storage()
            .persistent()
            .get(&VOTE_COUNT)
            .unwrap_or(Vec::new(&env))
    }
}
