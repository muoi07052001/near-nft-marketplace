// Hàm mint NFT

use crate::*;

#[near_bindgen]
impl Contract {
    /**
     * - Yêu cầu user nạp tiền để cover phí lưu trữ
     * - Thêm token vào tokens_by_id
     * - Thêm token metadata
     * - Thêm token vào danh sách sở hữu bởi owner
     * - Refund lại NEAR user deposit thừa
     */
    #[payable]
    pub fn nft_mint(&mut self, token_id: TokenId, metadata: TokenMetadata, receiver_id: AccountId) {
        let before_storage_usage = env::storage_usage(); // Dùng để tính toán lượng near thừa khi deposit

        let token = Token {
            owner_id: receiver_id,
        };

        // Nếu token_id đã tồn tại trong list tokens_by_id thì báo lỗi
        // Trong LookupMap, nếu key chưa tồn tại trong map -> Hàm insert return None
        assert!(
            self.tokens_by_id.insert(&token_id, &token).is_none(),
            "Token already exists"
        );

        // Thêm token metadata
        self.token_metadata_by_id.insert(&token_id, &metadata);
        
        // Thêm token vào danh sách sở hữu bởi owner
        self.internal_add_token_to_owner(&token_id, &token.owner_id);

        // Luợng data storage sử dụng = after_storage_usage - before_storage_usage
        let after_storage_usage = env::storage_usage();
        // Refund NEAR
        refund_deposit(after_storage_usage - before_storage_usage);
    }
}
