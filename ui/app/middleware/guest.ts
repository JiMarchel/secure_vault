export default defineNuxtRouteMiddleware(async () => {
  // Skip on SSR - cannot validate token without API call
  if (import.meta.server) return;

  const { checkAuth } = useAuth();
  const isLoggedIn = await checkAuth();

  if (isLoggedIn) {
    return navigateTo("/dashboard");
  }
});
