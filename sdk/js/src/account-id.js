/**
 * Either `AccountId` or `AccountAlias`. Some transactions and queries
 * accept `AccountAddress` as an input. All transactions and queries
 * return only `AccountId` as an output however.
 */
export class AccountAddress {
    /**
     * @param {BigInt} shard
     * @param {BigInt} realm
     */
    constructor(shard, realm) {
        if (!(this instanceof AccountId || this instanceof AccountAlias)) {
            throw new Error(
                "unsupported instantiation of AccountAddress, please use AccountId or AccountAlias"
            );
        }

        /** @type {BigInt} */
        this.shard = shard;

        /** @type {BigInt} */
        this.realm = realm;
    }
}

/**
 * The unique identifier for a cryptocurrency account on Hedera.
 */
export class AccountId extends AccountAddress {
    /**
     * @param {BigInt} num
     * @param {BigInt} shard
     * @param {BigInt} realm
     */
    constructor(num, shard = 0, realm = 0) {
        super(shard, realm);

        /** @type {BigInt} */
        this.num;

        Object.freeze(this);
    }
}

/**
 * The unique identifier for a cryptocurrency account represented with an
 * alias instead of an account number.
 */
export class AccountAlias extends AccountAddress {
    constructor() {
        super();

        /** @type {PublicKey} */
        this.alias;

        Object.freeze(this);
    }
}
