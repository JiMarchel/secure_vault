import { getOtpCode } from "@/lib/services/get-otp-code";
import { VerifOtpCard } from "./card";
import { getUserMe } from "@/lib/services/get-user-me";
import { getSession } from "@/lib/actions/get-session-cookie";

const AuthVerifOtp = async () => {
  const cookieString = await getSession();

  const otpRecord = await getOtpCode(cookieString);
  const user = await getUserMe(cookieString);

  return (
    <VerifOtpCard
      cookieString={cookieString}
      id={user.data?.id!}
      username={user.data?.username!}
      email={user.data?.email!}
      otp_code={otpRecord.data?.otpCode!}
      otp_expires_at={otpRecord.data?.otpExpiresAt!}
    />
  );
};

export default AuthVerifOtp;
