export default defineNuxtRouteMiddleware(() => {
  if (import.meta.client) return;

  const { value: authSession } = useCookie("auth_session");

  if (!authSession) {
    return navigateTo("/");
  }
});
