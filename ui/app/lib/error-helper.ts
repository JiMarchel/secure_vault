import { toast } from "vue-sonner";
import { toAppError, type AppError, type FieldError } from "./errors";

/**
 * Universal error handler for SecureVault
 * Handles both API errors and AppError instances consistently
 */
export async function errorHelper(error: unknown): Promise<void> {
  const appError = toAppError(error);

  await handleError(appError);
}

async function handleError(error: AppError): Promise<void> {
  if (error.status >= 500) {
    await navigateTo({
      path: `/error/${error.requestId || "unknown"}`,
      query: {
        message: error.message,
      },
    });
    return;
  }

  if (error.status >= 400) {
    const fieldErrors = error.fieldErrors;

    const description =
      fieldErrors && fieldErrors.length > 0
        ? h(
            "div",
            { class: "flex flex-col gap-1" },
            fieldErrors.map((v: FieldError) => h("div", `• ${v.message}`))
          )
        : undefined;

    toast.error(error.message, {
      duration: 10000,
      description,
    });
    return;
  }

  console.error("Unexpected error:", error);
  toast.error(error.message || "An unexpected error occurred");
}
