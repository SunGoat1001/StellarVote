🗳️ StellarVote: Decentralized Voting System
StellarVote là một ứng dụng bỏ phiếu phi tập trung (dApp) được xây dựng trên nền tảng Stellar Soroban. Dự án giải quyết bài toán minh bạch và toàn vẹn dữ liệu trong các cuộc bầu cử cộng đồng bằng cách sử dụng hợp đồng thông minh (Smart Contract).

📋 Tổng quan dự án (Project Brief)
Vấn đề: Các tổ chức hiện nay gặp khó khăn trong việc tổ chức bỏ phiếu đảm bảo tính bất biến, dễ bị thao túng kết quả và tốn kém chi phí kiểm định.

Giải pháp: Sử dụng Soroban Smart Contracts để tự động hóa việc kiểm phiếu và lưu trữ kết quả trực tiếp trên sổ cái (On-chain).

Người dùng mục tiêu: Các tổ chức DAO, câu lạc bộ sinh viên, ban quản trị chung cư.

Tại sao chọn Stellar: Chi phí cực thấp (< 0.0001 USD/phiếu), tốc độ xác thực nhanh (5 giây) và bảo mật cao với cơ chế require_auth.

🛠️ Tính năng cốt lõi (MVP)
[x] Khởi tạo cuộc bầu cử: Thiết lập các lựa chọn bỏ phiếu.

[x] Bỏ phiếu bảo mật: Mỗi ví chỉ được phép bỏ phiếu một lần (Anti-double voting).

[x] Xác thực danh tính: Sử dụng chữ ký số của người dùng để xác nhận quyền bầu cử.

[x] Truy vấn thời gian thực: Xem kết quả kiểm phiếu ngay lập tức từ Blockchain.

💻 Cấu trúc Smart Contract
Hợp đồng được viết bằng Rust và triển khai trên môi trường Soroban.

Các hàm chính:
vote(voter: Address, choice: u32): Tiếp nhận địa chỉ ví và chỉ số lựa chọn. Thực hiện kiểm tra quyền hạn và ghi đè trạng thái.

get_results() -> Vec<u32>: Trả về mảng tổng số phiếu bầu cho từng phương án.

🚀 Hướng dẫn triển khai (Deployment)
Tiền đề (Prerequisites)
Rust & Cargo

Stellar CLI

Tài khoản Testnet (đã được cấu hình là student)

Các bước thực hiện
Build Contract:

Bash
cargo build --target wasm32-unknown-unknown --release
Deploy lên Testnet:

Bash
stellar contract deploy \
  --wasm target/wasm32-unknown-unknown/release/stellar_vote.wasm \
  --source-account student \
  --network testnet
Thực hiện bỏ phiếu:

Bash
stellar contract invoke \
  --id <YOUR_CONTRACT_ID> \
  --source-account student \
  --network testnet \
  --send=yes \
  -- vote \
  --voter student \
  --choice 0
🛡️ Bảo mật & Tối ưu hóa
Storage: Sử dụng persistent storage để đảm bảo dữ liệu không bị xóa sau khi hợp đồng hết hạn TTL (Time To Live).

Validation: Ngăn chặn lỗi IndexBounds bằng cách kiểm tra và mở rộng độ dài Vector (Vec) tự động trước khi ghi dữ liệu.

Auth: Ép buộc xác thực thông qua voter.require_auth() để tránh tấn công giả mạo (Impersonation).
