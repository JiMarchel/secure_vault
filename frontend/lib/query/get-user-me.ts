import { UserType } from "../types/user";

export const getUserMe = async (cookieHeader?: string): Promise<UserType> => {
  const response = await fetch(
    `http://localhost:8000/api/user/session/get-me`,
    {
      headers: cookieHeader ? { Cookie: cookieHeader } : {},
    }
  );

  if (!response.ok) {
    const res = await response.json();
    throw new Error(
      `${res.error}, did you try to change the Uuid? please don't do that.&&${response.status} - ${response.statusText}`
    );
  }

  return response.json();
};
