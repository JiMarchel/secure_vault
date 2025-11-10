import { getOtpCode } from "@/lib/query/get-otp-code";
import { VerifOtpCard } from "./card";
import { getUserMe } from "@/lib/query/get-user-me";
import { getAuthSession } from "@/lib/actions/get-session-cookie";

const AuthVerifOtp = async () => {
  const cookieString = await getAuthSession();
  console.log(cookieString)

  const otpRecord = await getOtpCode(cookieString);
  const user = await getUserMe(cookieString);

  return (
    <VerifOtpCard
      cookieString={cookieString}
      id={user.id}
      username={user.username}
      email={user.email}
      otp_code={otpRecord.otp_code}
      otp_expires_at={otpRecord.otp_expires_at}
    />
  );
};

export default AuthVerifOtp;
