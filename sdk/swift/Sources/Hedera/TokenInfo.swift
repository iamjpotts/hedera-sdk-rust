import Foundation

public class TokenInfo: Codable {
    /// The ID of the token for which information is requested.
    public let tokenId: TokenId

    /// Name of token.
    public let name: String

    /// Symbol of token.
    public let symbol: String

    /// The amount of decimal places that this token supports.
    public let decimals: UInt32

    /// Total Supply of token.
    public let totalSupply: UInt64

    /// The ID of the account which is set as Treasury.
    public let treasuryAccountId: AccountId

    /// The key which can perform update/delete operations on the token.
    public let adminKey: Key?

    /// The key which can grant or revoke KYC of an account for the token's transactions.
    public let kycKey: Key?

    /// The key which can freeze or unfreeze an account for token transactions.
    public let freezeKey: Key?

    /// The key which can wipe token balance of an account.
    public let wipeKey: Key?

    /// The key which can change the supply of a token.
    public let supplyKey: Key?

    /// The key which can change the custom fees of the token.
    public let feeScheduleKey: Key?

    /// The default Freeze status (not applicable, frozen or unfrozen)
    public let defaultFreezeStatus: Bool?

    /// The default KYC status (KycNotApplicable or Revoked) of Hedera accounts relative to this token.
    public let defaultKycStatus: Bool?

    /// Specifies whether the token was deleted or not.
    public let isDeleted: Bool

    /// An account which will be automatically charged to renew the token's expiration,
    /// at autoRenewPeriod interval.
    public let autoRenewAccountId: AccountId?

    /// The interval at which the auto-renew account will be charged to extend the token's expiry
    public let autoRenewPeriod: TimeInterval?

    /// The epoch second at which the token will expire
    public let expirationTime: Date?

    /// The memo associated with the token
    public let tokenMemo: String

    /// The token type.
    public let tokenType: TokenType

    /// The token supply type
    public let tokenSupplyType: TokenSupplyType

    /// The Maximum number of tokens that can be in circulation.
    public let maxSupply: UInt64

    /// The custom fees to be assessed during a transfer that transfers units of this token.
    public let customFees: [CustomFee]

    /// The Key which can pause and unpause the Token.
    public let pauseKey: Key?

    /// Specifies whether the token is paused or not.
    public let pauseStatus: Bool?
}