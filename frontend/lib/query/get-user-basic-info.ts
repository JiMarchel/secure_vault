export const getUserBasicInfo = async (
  id: string | undefined
): Promise<UserBasicInfo> => {
  const baseApiUrl = process.env.BASE_API_URL;

  const response = await fetch(`${baseApiUrl}/users/basic/${id}`);

  if (!response.ok) {
    const res = await response.json();
    throw new Error(`${res.error}, did you try to change the Uuid? please don't do that.&&${response.status} - ${response.statusText}`);
  }

  return response.json();
};

type UserBasicInfo = {
  id: string;
  username: string;
  email: string;
};
