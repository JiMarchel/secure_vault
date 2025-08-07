import { getUserBasicInfo } from "@/lib/query/get-user-basic-info";
import { UserInfo } from "./user-info";
import { cookies } from "next/headers";
import { getOtpCode } from "@/lib/query/get-otp-code";

const AuthVerifOtp = async () => {
  const cookie = cookies();
  const userId = (await cookie).get("sc-verif-otp")?.value;
  const user = await getUserBasicInfo(userId);
  const otpCode = await getOtpCode(userId);

  console.log(otpCode);

  return <UserInfo username={user.username} email={user.email} otpExpiresAt={otpCode.otp_expires_at} id={userId}/>;
};

export default AuthVerifOtp;
