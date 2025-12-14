import { toast } from "vue-sonner";
import type { ErrorResponse } from "~/utils/model/response";

function isApiError(error: unknown): error is { data?: ErrorResponse, status: number } {
    return typeof error === 'object' && error !== null && 'data' in error
}

export async function errorHelper(error: unknown) {
  if (isApiError(error)) {
    if (error.status >= 500) {
      await navigateTo({
        path: `/error/${error.data?.requestId}`,
        query: {
          message: error.data?.error.message,
        },
      });
    } else if (error.status >= 400 && error.status < 500) {
      const validationErrors = error.data?.error.details?.validationErrors;
      const description =
        validationErrors && validationErrors.length > 0
          ? h(
              "div",
              { class: "flex flex-col gap-1" },
              validationErrors.map((v) => h("div", `• ${v.message}`))
            )
          : undefined;

      toast.error(error.data?.error.message || "Error occurred", {
        duration: 10000,
        description,
      });
    }
  } else {
    console.error("Unexpected error", error);
  }
}
