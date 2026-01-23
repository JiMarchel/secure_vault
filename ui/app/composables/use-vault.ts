import type { SuccessResponse } from "~/utils/model/response";
import type { Vaults } from "~/utils/model/vault";

export const useVaults = () => {
  const headers = useRequestHeaders(["cookie"]);

  const { data: vaults, refresh: refreshVaults, error: errorVaults } = useAsyncData(
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

  return {
    vaults,
    refreshVaults,
    errorVaults
  };
};
