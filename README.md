# Title
Stellar Deflationary Voucher Dapp (voucher-contract)

# Project Description
In traditional customer appreciation programs, physical paper vouchers/coupons or standard QR codes are highly vulnerable to counterfeiting, unauthorized duplication, and circulation fraud by merchants. This project was developed to eliminate these limitations entirely, making it an ideal solution for retail stores, fashion brands, coffee shops, or any business looking to operate a loyalty program.

By implementing a voucher issuance mechanism on the Stellar Blockchain network (utilizing Rust and the Soroban SDK), this smart contract allows merchants to issue a fixed supply of vouchers with strict expiration limits. The system ensures absolute transparency, as the circulating supply of vouchers on the market moves in only one direction—downward (deflationary)—each time a customer redeems a real-world reward.

# Core Features

## Secure Initialization
The contract tightly binds administrative rights to the Store Admin account immediately upon deployment. The merchant will set the total token supply one time only, along with an immutable expiration deadline.

## Time-Bound Enforcement
Hệ thống tích hợp thước đo thời gian thực của mạng lưới Blockchain (Stellar Ledger Timestamp). Khi khách hàng thực hiện đổi quà, hợp đồng sẽ tự động kiểm tra thời gian:
- Nếu thời gian hiện tại vượt quá `deadline`, toàn bộ giao dịch sẽ bị khóa lại.
- Phiếu đổi quà lập tức mất hiệu lực, ngăn chặn việc sử dụng phiếu quá hạn.

## Deflationary Burn Mechanism (`redeem`)
The system integrates the blockchain network's real-time clock (Stellar Ledger Timestamp). When a customer attempts to redeem a voucher, the smart contract automatically verifies the time:
- If the current time exceeds the deadline, the entire transaction is automatically locked.
- The voucher immediately becomes invalid, strictly preventing the usage of expired vouchers.

## Transparent Tracking
Data read functions (balance_of, total_vouchers_left) allow both customers and merchants to publicly check individual wallet balances and the total remaining voucher supply on the market without incurring network gas fees.

## Strict Authentication
The contract maximizes the utilization of Soroban's require_auth() function to ensure that every voucher transfer or redemption (redeem) must be digitally signed and verified directly by the customer's wallet. This prevents any unauthorized interference or voucher theft.

# Contract Link
Contract on Stellar Expert (Testnet):  
https://stellar.expert/explorer/testnet/tx/a6064e529334a4fe5d27a11b8863b0f3bdf03ab605d70b8e24327c0c2cb37d6a

## Interaction Screenshots
- Initialization and Store Setup: `screenshot_deploy.png`
- Successful Voucher Redemption (Burn): `screenshot_Invoke_redeem.png`

# Future Scope

## Front-end & Mobile Integration
A convenient mobile application developed with Flutter integrates seamlessly with the Stellar Freighter wallet. Customers simply present a QR code containing their wallet address or the Contract ID at the counter; the store staff scans the code, automatically triggering the redeem function on the blockchain to burn the voucher and distribute the real-world reward.

## Anti-Whale & Anti-Speculation
An additional ownership limit logic is implemented (e.g., each customer wallet can only receive and hold a maximum of 1 or 2 vouchers). This strictly prevents hoarding or any individual from monopolizing the merchant's entire supply of discount vouchers.

## Multi-Campaign Support
The contract is upgraded to enable a single merchant to manage and launch multiple distinct reward campaigns simultaneously under a single deployed contract (utilizing separate Token IDs or Campaign IDs).

# Author Profile
- Full Name: Nguyễn Võ Nhật Thuận
- Role: Student
- Contact/GitHub: https://github.com/thuan65