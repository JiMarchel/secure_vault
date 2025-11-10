export const checkUserSession = async (
  cookieHeader?: string
): Promise<CheckSessionResponse | undefined> => {
  try {
    const response = await fetch(
      "http://localhost:8000/api/user/check-session",
      {
        headers: cookieHeader ? { Cookie: cookieHeader } : {},
      }
    );
    return response.json();
  } catch (e) {
    console.error("Error checking session:", e);
  }
};

type CheckSessionResponse = {
  authenticated: boolean;
  message: string | null;
};
