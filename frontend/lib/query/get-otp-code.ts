export const getOtpCode = async (id: string | undefined): Promise<OtpVerif> => {
  const baseApiUrl = process.env.BASE_API_URL;
  const response = await fetch(`${baseApiUrl}/users/otp-code/${id}`);

  if (!response.ok) {
    const res = await response.json();
    throw new Error(
      `${res.error}, did you try to change the Uuid? please don't do that.&&${response.status} - ${response.statusText}`
    );
  }

  return response.json();
};

type OtpVerif = {
  id: string;
  otp_code: string;
  otp_expires_at: string;
};
