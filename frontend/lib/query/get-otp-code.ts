export const getOtpCode = async (cookieHeader?: string): Promise<OtpVerif> => {
  const response = await fetch(
    "http://localhost:8000/api/user/session/get-otp",
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

type OtpVerif = {
  // id: string;
  otp_code: string;
  otp_expires_at: string;
};
