import { getUserBasicInfo } from "@/lib/query/get-user-basic-info";
import { UserInfo } from "./user-info";
import { cookies } from "next/headers";

const AuthVerifOtp = async () => {
  const cookie = cookies();
  const userId = (await cookie).get("sc-verif-otp")?.value;
  const getUser = await getUserBasicInfo(userId);
  return <UserInfo />;
};

export default AuthVerifOtp;
