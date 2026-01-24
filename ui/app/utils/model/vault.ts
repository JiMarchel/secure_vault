export interface Vaults {
    id: string;
    title: string;
    encryptedData: string;
    nonce: string;
    itemType: string;
    createdAt: string;
    updatedAt: string;
}

export interface EncryptedVault {
    title: string;
    itemType: string;
    encryptedData: string;
    nonce: string;
}

export interface PasswordDecrypted {
    usernameOrEmail: string;
    password: string;
    websiteOrApp: string;
}