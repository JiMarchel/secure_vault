import { queryOptions } from "@tanstack/react-query";

export default function userBasicInfoQuery(id: string | undefined) {
  return queryOptions({
    queryKey: ["user-basic-info", id],
    queryFn: () => getUserBasicInfo(id),
    enabled: !!id,
  });
}

export const getUserBasicInfo = async (
  id: string | undefined
): Promise<UserBasicInfo> => {
  const baseApiUrl = process.env.BASE_API_URL;

  const response = await fetch(`${baseApiUrl}/users/basic/${id}`);

  return response.json();
};

type UserBasicInfo = {
  id: string;
  username: string;
  email: string;
};
