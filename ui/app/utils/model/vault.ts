export interface Vaults {
    id: string;
    title: string;
    encryptedData: string;
    nonce: string;
    itemType: string;
    createdAt: string;
    updatedAt: string;
}

export interface PasswordDecrypted {
    usernameOrEmail: string;
    password: string;
    websiteOrApp: string;
}