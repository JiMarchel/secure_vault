import { toast } from "vue-sonner";
import type { VaultItem } from "~/lib/wasm/type";
import { decryptVaultItem, encryptVaultItem } from "~/lib/wasm/vault";
import type { SuccessResponse } from "~/utils/model/response";
import type { EncryptedVault, Vaults } from "~/utils/model/vault";
import {
  type addNoteType,
  type addPasswordType,
  type addCreditCardType,
  type addContactType,
} from "~/utils/validation/vaults";

export const useVaults = () => {
  const isLoading = ref(false);
  const headers = useRequestHeaders(["cookie"]);
  const { useDek } = useAuth();

  const {
    data: vaults,
    refresh: refreshVaults,
    error: errorVaults,
  } = useAsyncData(
    "vaults",
    async () => {
      const res = await $fetch<SuccessResponse<Vaults[]>>("/vault/all", {
        method: "GET",
        baseURL: "http://localhost:8000/api",
        credentials: "include",
        headers,
      });
      return res.data;
    },
    {
      server: false,
    },
  );

  const encryptVault = async (value: string) => {
    const dek = useDek();
    const encryptedVaultRes = await encryptVaultItem(dek, value);
    return encryptedVaultRes;
  };

  const decryptVault = async (value: VaultItem) => {
    const dek = useDek();
    const decryptedVaultRes = await decryptVaultItem(dek, value);
    return decryptedVaultRes;
  };

  const add = async (body: EncryptedVault) => {
    isLoading.value = true;

    const { $api } = useNuxtApp();

    const res = await $api<SuccessResponse<void>>("/vault", {
      body,
      method: "POST",
    });

    toast.success(res.message);

    await refreshVaults();
  };

  const deleteVault = async (id: string) => {
    isLoading.value = true;

    const { $api } = useNuxtApp();

    const res = await $api<SuccessResponse<void>>(`/vault/${id}`, {
      method: "DELETE",
    });

    toast.success(res.message);

    await refreshVaults();

    isLoading.value = false;
  };

  const updateVault = async (body: EncryptedVault) => {
    isLoading.value = true;

    const { $api } = useNuxtApp();

    const res = await $api<SuccessResponse<void>>(`/vault`, {
      body,
      method: "PUT",
    });

    toast.success(res.message);

    await refreshVaults();
  };

  const password = async (
    value: addPasswordType,
    update?: boolean,
    id?: string,
  ) => {
    const credsValue = {
      usernameOrEmail: value.usernameOrEmail,
      password: value.password,
      websiteOrApp: value.websiteOrApp,
    };

    const encryptedVaultRes = await encryptVault(JSON.stringify(credsValue));

    if (update) {
      const bodyReq = {
        id,
        title: value.title,
        itemType: "Password",
        ...encryptedVaultRes,
      };
      await updateVault(bodyReq);
    } else {
      const bodyReq = {
        title: value.title,
        itemType: "Password",
        ...encryptedVaultRes,
      };
      await add(bodyReq);
    }
    isLoading.value = false;
  };

  const note = async (value: addNoteType, update?: boolean, id?: string) => {
    const credsValue = {
      note: value.note,
    };

    const encryptedVaultRes = await encryptVault(JSON.stringify(credsValue));

    if (update) {
      const bodyReq = {
        id,
        title: value.title,
        itemType: "Note",
        ...encryptedVaultRes,
      };
      await updateVault(bodyReq);
    } else {
      const bodyReq = {
        title: value.title,
        itemType: "Note",
        ...encryptedVaultRes,
      };
      await add(bodyReq);
    }
    isLoading.value = false;
  };

  const creditCard = async (
    value: addCreditCardType,
    update?: boolean,
    id?: string,
  ) => {
    const credsValue = {
      cardHolderName: value.cardHolderName,
      cardNumber: value.cardNumber,
      cardExpirationDate: value.cardExpirationDate,
      cardCvv: value.cardCvv,
      pin: value.pin,
      zipOrPostalCode: value.zipOrPostalCode,
    };

    const encryptedVaultRes = await encryptVault(JSON.stringify(credsValue));

    if (update) {
      const bodyReq = {
        id,
        title: value.title,
        itemType: "CreditCard",
        ...encryptedVaultRes,
      };
      await updateVault(bodyReq);
    } else {
      const bodyReq = {
        title: value.title,
        itemType: "CreditCard",
        ...encryptedVaultRes,
      };
      await add(bodyReq);
    }
    isLoading.value = false;
  };

  const contact = async (
    value: addContactType,
    update?: boolean,
    id?: string,
  ) => {
    const credsValue = {
      fullName: value.fullName,
      email: value.email,
      phoneNumber: value.phoneNumber,
      address: value.address,
      city: value.city,
      state: value.state,
      country: value.country,
    };

    const encryptedVaultRes = await encryptVault(JSON.stringify(credsValue));

    if (update) {
      const bodyReq = {
        id,
        title: value.title,
        itemType: "Contact",
        ...encryptedVaultRes,
      };
      await updateVault(bodyReq);
    } else {
      const bodyReq = {
        title: value.title,
        itemType: "Contact",
        ...encryptedVaultRes,
      };
      await add(bodyReq);
    }
    isLoading.value = false;
  };

  return {
    vaults,
    refreshVaults,
    errorVaults,

    isLoading,

    password,
    note,
    creditCard,
    contact,
    deleteVault,
  };
};
